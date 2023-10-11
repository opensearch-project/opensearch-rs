use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[doc = "Cluster information"]
pub struct InfoResponse {
    name: String,
    cluster_name: String,
    cluster_uuid: String,
    version: OpenSearchVersionInfo,
    #[serde(rename = "tagline")]
    tag_line: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenSearchVersionInfo {
    distribution: String,
    number: String,
    build_type: String,
    build_hash: String,
    build_date: String,
    build_snapshot: bool,
    lucene_version: String,
    minimum_wire_compatibility_version: String,
    minimum_index_compatibility_version: String,
}
