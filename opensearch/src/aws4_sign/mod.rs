use chrono::{DateTime, Utc};
use generic_array::typenum::U32;
use generic_array::GenericArray;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Request,
};
use sha2::Sha256;

use crate::auth::AwsCredentials;

type HmacSha256 = Hmac<Sha256>;

// Opensearch still uses its old name, "Elasticsearch"
const AWS_OPENSEARCH_SERVICE: &'static str = "es";

pub fn add_aws_auth_header(request: &mut Request, aws_creds: &AwsCredentials, domain: &str) {
    let now = chrono::Utc::now();

    request.headers_mut().insert(
        "X-Amz-Date",
        now.format("%Y%m%dT%H%M%SZ").to_string().parse().unwrap(),
    );
    request
        .headers_mut()
        .insert("X-Amz-Content-Sha256", "UNSIGNED-PAYLOAD".parse().unwrap());
    request
        .headers_mut()
        .insert("host", domain.parse().unwrap());

    if let Some(session_token) = aws_creds.session_token.as_ref() {
        request.headers_mut().insert(
            "x-amz-security-token",
            HeaderValue::from_str(session_token).expect("Invalid session token"),
        );
    }

    let auth_hdr = auth_header(
        &*request,
        &now,
        &aws_creds.access_key_id,
        &aws_creds.secret_access_key,
        AWS_OPENSEARCH_SERVICE,
        &aws_creds.region,
    );
    request
        .headers_mut()
        .insert("authorization", HeaderValue::from_str(&auth_hdr).unwrap());
}

/// Generates the string that should be added as the "authorization" header.
/// Query param authorization is not currently supported.
///
/// Best reference at the time of writing is:
/// https://docs.aws.amazon.com/general/latest/gr/sigv4-signed-request-examples.html
fn auth_header(
    req: &Request,
    timestamp: &DateTime<Utc>,
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    aws_service_name: &str,
    aws_region: &str,
) -> String {
    let (timestamp, datestamp) = datetime_to_amz_strings(timestamp);

    let verb = req.method().as_str();
    let canonical_uri = req.url().path();
    let canonical_querystring = req.url().query().unwrap_or("");

    let mut canonical_header_list: Vec<(&HeaderName, &HeaderValue)> = req
        .headers()
        .iter()
        .filter(|(k, _)| is_canonical_header(k))
        .collect();
    canonical_header_list.sort_by_key(|item| item.0.as_str());

    // Canonical headers: All the headers to be signed, along with their
    // values, one per line.
    // Signed headers: All the headers to be signed, header name only,
    // separated by semicolons.
    // See the AWS4 signing algorithm for details
    let mut canonical_headers = String::with_capacity(256);
    let mut signed_headers = String::with_capacity(256);
    let mut signed_headers_sep = "";
    for (k, v) in &canonical_header_list {
        canonical_headers += k.as_str();
        canonical_headers += ":";
        canonical_headers += v.to_str().unwrap(); // TODO Handle better
        canonical_headers += "\n";

        signed_headers += signed_headers_sep;
        signed_headers += k.as_str();
        signed_headers_sep = ";";
    }

    let body = req
        .body()
        .expect("Streaming bodies are not supported")
        .as_bytes()
        .expect("Streaming bodies are not supported");
    let payload_hash = hash(body);
    let payload_hash_hex = bytes_to_hex_string(&payload_hash);

    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        verb,
        &canonical_uri,
        &canonical_querystring,
        &canonical_headers,
        &signed_headers,
        &payload_hash_hex,
    );

    let creq_hash = hash(canonical_request.as_bytes());
    let creq_hash_str = bytes_to_hex_string(&creq_hash);
    let algorithm = "AWS4-HMAC-SHA256";
    let credential_scope = format!(
        "{}/{}/{}/aws4_request",
        &datestamp, aws_region, aws_service_name
    );
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        algorithm, &timestamp, credential_scope, creq_hash_str,
    );

    let signing_key = get_signature_key(
        aws_secret_access_key.as_bytes(),
        datestamp.as_bytes(),
        aws_region.as_bytes(),
        aws_service_name.as_bytes(),
    );

    let signature = sign(&signing_key, string_to_sign.as_bytes());

    let authorization_header = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        &algorithm,
        &aws_access_key_id,
        &credential_scope,
        &signed_headers,
        &bytes_to_hex_string(&signature),
    );

    authorization_header
}

fn is_canonical_header(name: &HeaderName) -> bool {
    const CANONICAL_HEADERS: [&'static str; 8] = [
        "content-length",
        "content-type",
        "host",
        "x-amz-content-sha256",
        "x-amz-date",
        "x-amz-security-token",
        "x-amz-target",
        "x-amz-user-agent",
    ];
    CANONICAL_HEADERS
        .iter()
        .find(|n| **n == name.as_str())
        .is_some()
}

fn hash(input: &[u8]) -> GenericArray<u8, U32> {
    use sha2::Digest;
    let mut sha256 = Sha256::new();
    sha256.update(input);
    sha256.finalize()
}

fn bytes_to_hex_string(input: &[u8]) -> String {
    // It's kinda annoying that there's no easy way with just the Rust std
    // lib to do this, but it's easy enough to implement:
    use std::fmt::Write;
    let mut s = String::with_capacity(input.len() * 2);
    for b in input {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

fn get_signature_key(
    key: &[u8],
    datestamp: &[u8],
    region_name: &[u8],
    service_name: &[u8],
) -> GenericArray<u8, U32> {
    let mut aws_key = "AWS4".as_bytes().to_vec();
    aws_key.extend_from_slice(key);
    let kdate = sign(&aws_key, datestamp);
    let kregion = sign(&kdate, region_name);
    let kservice = sign(&kregion, service_name);
    let ksigning = sign(&kservice, b"aws4_request");
    ksigning
}

// Applies the signing algorithm (HMAC using SHA265)
fn sign(key: &[u8], input: &[u8]) -> GenericArray<u8, U32> {
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(input);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    code_bytes
}

// Returns 't' as both a date-time string, and a date-only string. The
// format of the strings is mandated by AWS.
fn datetime_to_amz_strings(t: &DateTime<Utc>) -> (String, String) {
    let datetime = t.format("%Y%m%dT%H%M%SZ").to_string();
    let date = t.format("%Y%m%d").to_string();
    (datetime, date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_token() {
        const HOST: &'static str = "some.domain.ap-southeast-2.es.amazonaws.com";
        const USER_AGENT: &'static str = "test-user-agent";
        const AMZ_USER_AGENT: &'static str = "test-amz-user-agent";

        let client = reqwest::Client::new();

        let url = format!("https://{HOST}");
        let canonical_uri = "/index-name/_search";
        let target_url = format!("{url}{canonical_uri}");

        const TIMESTAMP: &str = "2022-01-23T06:08:41Z";
        let t: DateTime<Utc> = DateTime::parse_from_rfc3339(TIMESTAMP).unwrap().into();
        let (timestamp, _) = datetime_to_amz_strings(&t);

        let body = r#"{"Content": "Some JSON, doesn't matter what for testing"}"#;
        let builder = client
            .post(&target_url)
            .header("host", HOST)
            .header("content-type", "application/json")
            .header("content-length", format!("{}", body.len()))
            .header("user-agent", USER_AGENT)
            .header("x-amz-user-agent", AMZ_USER_AGENT)
            .header("x-amz-date", &timestamp);
        let request = builder.body(body).build().unwrap();
        let auth = auth_header(
            &request,
            &t,
            "AKAKAKAKAKAKAKAKAKAK",
            "VMVMVMVMVMVMVMVMVMVMVMVMVMVMVMVMVMVMVMVM",
            "es",
            "ap-southeast-2",
        );

        const EXPECTED: &str = "AWS4-HMAC-SHA256 Credential=AKAKAKAKAKAKAKAKAKAK/20220123/\
            ap-southeast-2/es/aws4_request, SignedHeaders=content-length;content-type;\
            host;x-amz-date;x-amz-user-agent, \
            Signature=fc3a4f5d9f0926af23e6e886120089996ba71d0890f89fddee05cf5e864b3985";
        assert_eq!(&auth, EXPECTED);
    }

    #[test]
    fn test_with_token() {
        const HOST: &'static str = "dummy.domain.ap-southeast-2.amazonaws.com";
        const USER_AGENT: &'static str = "test-user-agent";
        const AMZ_USER_AGENT: &'static str = "test-amz-user-agent";

        let client = reqwest::Client::new();

        let url = format!("https://{HOST}");
        let canonical_uri = "/my-index-name/_doc/12345";
        let target_url = format!("{url}{canonical_uri}");

        const TIMESTAMP: &str = "2022-01-23T06:42:30Z";
        let t: DateTime<Utc> = DateTime::parse_from_rfc3339(TIMESTAMP).unwrap().into();
        let (timestamp, _) = datetime_to_amz_strings(&t);

        let aws_session_token = "ValA".repeat(231); // Dummy session ID

        let body = r#"{"Content":{"Simple":{"Subject":{"Data":..."#;
        let builder = client
            .post(&target_url)
            .header("host", HOST)
            .header("content-type", "application/json")
            .header("content-length", format!("{}", body.len()))
            .header("user-agent", USER_AGENT)
            .header("x-amz-user-agent", AMZ_USER_AGENT)
            .header("x-amz-date", &timestamp)
            .header("x-amz-security-token", aws_session_token);
        let request = builder.body(body).build().unwrap();
        let auth = auth_header(
            &request,
            &t,
            "ASASASASASASASASASAS",
            "8s8s8s8s8s8s8s8s8s8s8s8s8s8s8s8s8s8s8s8s",
            "es",
            "ap-southeast-2",
        );

        const EXPECTED: &str = "AWS4-HMAC-SHA256 Credential=ASASASASASASASASASAS/20220123/\
            ap-southeast-2/es/aws4_request, SignedHeaders=content-length;content-type;\
            host;x-amz-date;x-amz-security-token;x-amz-user-agent, \
            Signature=246f0f0c9ff73d3de189eb0b93457c4a077f76627a15fc170f4cad0ccd983f93";
        assert_eq!(&auth, EXPECTED);
    }
}
