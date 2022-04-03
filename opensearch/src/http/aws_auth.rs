use std::time::SystemTime;

use aws_sigv4::{
    http_request::{
        sign, PayloadChecksumKind, PercentEncodingMode, SignableBody, SignableRequest,
        SignatureLocation, SigningParams, SigningSettings,
    },
    signing_params::BuildError,
};
use aws_types::{
    credentials::{ProvideCredentials, SharedCredentialsProvider},
    region::Region,
    Credentials,
};
use reqwest::Request;

const SERVICE_NAME: &str = "es";

fn get_signing_params<'a>(
    credentials: &'a Credentials,
    region: &'a Region,
) -> Result<SigningParams<'a>, BuildError> {
    let mut settings = SigningSettings::default();
    settings.percent_encoding_mode = PercentEncodingMode::Double;
    settings.payload_checksum_kind = PayloadChecksumKind::NoHeader;
    settings.signature_location = SignatureLocation::Headers;
    settings.expires_in = None;

    let mut builder = SigningParams::builder()
        .access_key(credentials.access_key_id())
        .secret_key(credentials.secret_access_key())
        .service_name(SERVICE_NAME)
        .region(region.as_ref())
        .time(SystemTime::now())
        .settings(settings);

    if let Some(session_token) = credentials.session_token() {
        builder = builder.security_token(session_token);
    }

    builder.build()
}

pub async fn sign_request(
    request: &mut Request,
    credentials_provider: &SharedCredentialsProvider,
    region: &Region,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let credentials = credentials_provider.provide_credentials().await?;

    let params = get_signing_params(&credentials, region)?;

    let uri = request.url().as_str().parse()?;

    let signable_request = SignableRequest::new(
        request.method(),
        &uri,
        request.headers(),
        SignableBody::Bytes(request.body().and_then(|b| b.as_bytes()).unwrap_or(&[])),
    );

    let (mut instructions, _) = sign(signable_request, &params)?.into_parts();

    if let Some(new_headers) = instructions.take_headers() {
        for (name, value) in new_headers.into_iter() {
            request.headers_mut().insert(name.unwrap(), value);
        }
    }

    Ok(())
}
