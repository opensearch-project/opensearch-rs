/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Round-trip tests for the generated response types: realistic server
//! response fixtures must deserialize into the typed models and serialize
//! back to the same JSON. No server is required.

use opensearch::types::{
    bulk::BulkResponse, common::WriteResponseBase, get::GetResponse, search::SearchResponse,
};
use serde_json::{json, Value};

/// Deserializes the fixture as `T`, serializes it back, and asserts the
/// JSON survived unchanged
fn round_trip<T>(fixture: &Value) -> T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let typed: T = serde_json::from_value(fixture.clone())
        .unwrap_or_else(|e| panic!("deserialization failed: {}", e));
    let back = serde_json::to_value(&typed).unwrap();
    assert_eq!(fixture, &back, "round-trip changed the JSON");
    typed
}

#[test]
fn search_response_round_trips() {
    let fixture = json!({
        "took": 5,
        "timed_out": false,
        "_shards": { "total": 2, "successful": 2, "skipped": 0, "failed": 0 },
        "hits": {
            "total": { "value": 42, "relation": "eq" },
            "max_score": 1.3,
            "hits": [
                {
                    "_index": "movies",
                    "_id": "1",
                    "_score": 1.3,
                    "_source": { "title": "Moneyball", "year": 2011 }
                }
            ]
        }
    });
    let resp: SearchResponse = round_trip(&fixture);
    assert_eq!(resp.took, 5);
    assert!(!resp.timed_out);
    assert_eq!(resp.shards.total, 2);
    assert_eq!(resp.hits.hits.len(), 1);

    let hit = &resp.hits.hits[0];
    assert_eq!(hit.index.as_deref(), Some("movies"));
    // _source is intentionally untyped: the caller decides the type
    let source = hit.source.as_ref().unwrap();
    let doc: Value = serde_json::from_str(source.get()).unwrap();
    assert_eq!(doc["title"], "Moneyball");
}

#[test]
fn total_hits_dispatches_on_json_kind() {
    // rest_total_hits_as_int=true collapses total to a bare integer
    let as_object = json!({ "value": 42, "relation": "eq" });
    let as_int = json!(42);

    use opensearch::types::search::SearchResponseHitsTotal;
    let object: SearchResponseHitsTotal = round_trip(&as_object);
    match object {
        SearchResponseHitsTotal::TotalHits(t) => assert_eq!(t.value, 42),
        other => panic!("expected the object branch, got {:?}", other),
    }
    let int: SearchResponseHitsTotal = round_trip(&as_int);
    match int {
        SearchResponseHitsTotal::Integer(v) => assert_eq!(v, 42),
        other => panic!("expected the integer branch, got {:?}", other),
    }
}

#[test]
fn aggregations_decode_lazily() {
    let fixture = json!({
        "took": 3,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": { "hits": [] },
        "aggregations": {
            "my_genres": {
                "doc_count_error_upper_bound": 0,
                "sum_other_doc_count": 0,
                "buckets": [
                    {
                        "key": "rock",
                        "doc_count": 12,
                        "avg_year": { "value": 1998.5 }
                    },
                    {
                        "key": "jazz",
                        "doc_count": 3
                    }
                ]
            },
            "avg_year": { "value": 2001.25 }
        }
    });
    let resp: SearchResponse = round_trip(&fixture);
    let aggregations = resp.aggregations.as_ref().unwrap();

    // top level: parse each aggregation as the branch the request asked for
    let genres = aggregations["my_genres"].as_sterms().unwrap();
    let avg = aggregations["avg_year"].as_avg().unwrap();
    assert_eq!(avg.value, Some(2001.25));

    // string terms buckets are declared as the array form by the spec
    let buckets = &genres.buckets;
    assert_eq!(buckets.len(), 2);
    assert_eq!(buckets[0].key, "rock");
    assert_eq!(buckets[0].doc_count, 12);

    // sub-aggregations: residual keys of the bucket, decoded lazily again
    let sub = buckets[0].aggregations["avg_year"].as_avg().unwrap();
    assert_eq!(sub.value, Some(1998.5));
    assert!(buckets[1].aggregations.is_empty());
}

#[test]
fn wrong_branch_reports_union_error() {
    let fixture = json!({
        "took": 1,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": { "hits": [] },
        "aggregations": {
            "avg_year": { "value": 2001.25 }
        }
    });
    let resp: SearchResponse = round_trip(&fixture);
    let aggregations = resp.aggregations.as_ref().unwrap();

    // asking for the wrong branch fails without poisoning the value
    let err = aggregations["avg_year"].as_sterms().unwrap_err();
    assert_eq!(err.attempted(), "sterms");
    // the raw JSON stays accessible
    assert!(aggregations["avg_year"].raw().get().contains("2001.25"));
    // and the right branch still works
    assert!(aggregations["avg_year"].as_avg().is_ok());
}

#[test]
fn unknown_enum_value_does_not_fail_the_response() {
    use opensearch::types::search::TotalHitsRelation;
    let known: TotalHitsRelation = serde_json::from_value(json!("gte")).unwrap();
    assert_eq!(known, TotalHitsRelation::Gte);
    // a value the specification does not know must not fail deserialization,
    // and is kept verbatim so that it round-trips
    let unknown: TotalHitsRelation = serde_json::from_value(json!("approximate")).unwrap();
    assert_eq!(
        unknown,
        TotalHitsRelation::Unknown("approximate".to_string())
    );
    assert_eq!(unknown.as_str(), "approximate");
    assert_eq!(
        serde_json::to_value(&unknown).unwrap(),
        json!("approximate")
    );
}

#[test]
fn index_response_round_trips() {
    // the index API returns the shared write response
    let fixture = json!({
        "_index": "movies",
        "_id": "1",
        "_version": 1,
        "result": "created",
        "_shards": { "total": 2, "successful": 1, "failed": 0 },
        "_seq_no": 0,
        "_primary_term": 1
    });
    let resp: WriteResponseBase = round_trip(&fixture);
    assert_eq!(resp.index, "movies");
    assert_eq!(resp.version, 1);

    // IndexResponse is an alias of the shared type
    let _alias: opensearch::types::index::IndexResponse = round_trip(&fixture);
}

#[test]
fn bulk_response_round_trips() {
    let fixture = json!({
        "took": 11,
        "errors": true,
        "items": [
            {
                "index": {
                    "_index": "movies",
                    "_id": "1",
                    "_version": 1,
                    "result": "created",
                    "status": 201,
                    "_shards": { "total": 2, "successful": 1, "failed": 0 },
                    "_seq_no": 0,
                    "_primary_term": 1
                }
            },
            {
                "delete": {
                    "_index": "movies",
                    "_id": "404",
                    "status": 404,
                    "error": {
                        "type": "document_missing_exception",
                        "reason": "[404]: document missing"
                    }
                }
            }
        ]
    });
    let resp: BulkResponse = round_trip(&fixture);
    assert_eq!(resp.took, 11);
    assert!(resp.errors);
    assert_eq!(resp.items.len(), 2);
}

#[test]
fn get_response_round_trips() {
    let fixture = json!({
        "_index": "movies",
        "_id": "1",
        "_version": 1,
        "_seq_no": 0,
        "_primary_term": 1,
        "found": true,
        "_source": { "title": "Moneyball", "year": 2011 }
    });
    let resp: GetResponse = round_trip(&fixture);
    assert_eq!(resp.index, "movies");
    assert!(resp.found);
    let source = resp.source.as_ref().unwrap();
    let doc: Value = serde_json::from_str(source.get()).unwrap();
    assert_eq!(doc["year"], 2011);

    // a get miss: found=false and no source
    let miss = json!({
        "_index": "movies",
        "_id": "999",
        "found": false
    });
    let resp: GetResponse = round_trip(&miss);
    assert!(!resp.found);
    assert!(resp.source.is_none());
}

/// Dispatch unions route on the first byte of the raw JSON text, so they
/// decode identically with and without serde_json's `arbitrary_precision`
/// (run this file with `--features _test-arbitrary-precision` too)
#[test]
fn dispatch_unions_decode_every_kind() {
    use opensearch::types::common::FieldValue;

    let parse = |text: &str| serde_json::from_str::<FieldValue>(text);
    assert!(matches!(parse("true").unwrap(), FieldValue::Boolean(true)));
    assert!(matches!(
        parse("false").unwrap(),
        FieldValue::Boolean(false)
    ));
    assert!(matches!(parse("1.5").unwrap(), FieldValue::Number(n) if n == 1.5));
    assert!(matches!(parse("-3").unwrap(), FieldValue::Number(n) if n == -3.0));
    assert!(matches!(parse("1e3").unwrap(), FieldValue::Number(n) if n == 1000.0));
    assert!(matches!(parse(r#""abc""#).unwrap(), FieldValue::String(s) if s == "abc"));
    assert!(matches!(parse("null").unwrap(), FieldValue::Null));
    // surrounding whitespace is not part of the value
    assert!(matches!(parse("  42 ").unwrap(), FieldValue::Number(n) if n == 42.0));

    // a kind the union does not have is an error naming both sides
    let err = parse("{}").unwrap_err().to_string();
    assert!(
        err.contains("invalid type: map, expected a FieldValue value"),
        "{}",
        err
    );
    let err = parse("[1]").unwrap_err().to_string();
    assert!(err.contains("invalid type: sequence"), "{}", err);
    // a malformed value of the right kind still fails
    assert!(parse("tru").is_err());

    // round trip through serde_json::Value, as done by `round_trip`
    for v in [json!(true), json!(1.5), json!("x"), json!(null)].iter() {
        let typed: FieldValue = serde_json::from_value(v.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&typed).unwrap(), v);
    }
}

#[test]
fn search_sort_values_and_integer_totals_decode() {
    // `sort` holds FieldValues, `rest_total_hits_as_int` turns total into
    // the integer branch of a dispatch union: both broke with
    // `arbitrary_precision` when unions relied on `deserialize_any`
    let fixture = json!({
        "took": 1,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": {
            "total": 2,
            "hits": [
                { "_index": "i", "_id": "1", "sort": [900719925474099.2, "a", true] },
                // FieldValue::Number is f64: an integer sort value would
                // come back as `-1.0`, which the strict helper rejects
                { "_index": "i", "_id": "2", "sort": [-1.5, "b", false] }
            ]
        }
    });
    let _: SearchResponse = round_trip(&fixture);
    // decode from text, as `Response::json` does: `from_value` takes a
    // different path through serde_json and hides the number handling
    let resp: SearchResponse = serde_json::from_str(&fixture.to_string()).unwrap();

    use opensearch::types::common::FieldValue;
    use opensearch::types::search::SearchResponseHitsTotal;
    assert!(matches!(
        resp.hits.total,
        Some(SearchResponseHitsTotal::Integer(2))
    ));
    let sort = resp.hits.hits[0].sort.as_ref().unwrap();
    assert!(matches!(sort[0], FieldValue::Number(n) if n == 900719925474099.2));
    assert!(matches!(&sort[1], FieldValue::String(s) if s == "a"));
    assert!(matches!(sort[2], FieldValue::Boolean(true)));
}

#[test]
fn fields_unknown_to_the_specification_are_ignored() {
    // `ext` (search pipeline processors) and `plugin_stats` are not part of
    // the specification's search response; `extra` is not part of shard
    // statistics. They are ignored, as by the other OpenSearch clients: the
    // typed client can report them instead (`ignore_unknown_fields(false)`)
    let fixture = json!({
        "took": 5,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0, "extra": [1, 2] },
        "hits": {
            "total": { "value": 1, "relation": "eq" },
            "max_score": 1.0,
            "hits": [
                { "_index": "docs", "_id": "1", "_score": 1.0, "_source": { "t": "x" } }
            ]
        },
        "ext": { "retrieval_augmented_generation": { "answer": "42" } },
        "plugin_stats": 7
    });
    let resp: SearchResponse = serde_json::from_value(fixture).unwrap();
    assert_eq!(resp.took, 5);
    assert_eq!(resp.shards.total, 1);
    let hit = &resp.hits.hits[0];
    assert_eq!(hit.source.as_ref().unwrap().get(), r#"{"t":"x"}"#);

    // serializing writes the known fields only
    let back = serde_json::to_value(&resp).unwrap();
    assert!(back.get("ext").is_none() && back.get("plugin_stats").is_none());
    assert!(back["_shards"].get("extra").is_none());
}

#[test]
fn a_duplicate_known_key_is_still_an_error() {
    // the generated impls must reject what derived impls reject, for
    // required and optional fields alike
    let required = r#"{"took":1,"took":2,"timed_out":false,"_shards":{"total":1,"successful":1,"failed":0},"hits":{"hits":[]}}"#;
    let err = serde_json::from_str::<SearchResponse>(required).unwrap_err();
    assert!(
        err.to_string().contains("duplicate field `took`"),
        "{}",
        err
    );

    let optional = r#"{"took":1,"timed_out":false,"_shards":{"total":1,"successful":1,"failed":0},"hits":{"hits":[]},"terminated_early":null,"terminated_early":true}"#;
    let err = serde_json::from_str::<SearchResponse>(optional).unwrap_err();
    assert!(
        err.to_string()
            .contains("duplicate field `terminated_early`"),
        "{}",
        err
    );
}

#[test]
fn source_as_decodes_documents_into_the_callers_type() {
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct Movie {
        title: String,
        year: i32,
    }
    // a type borrowing from the response: no copy of the strings that need
    // no unescaping (`&str` alone would fail on a string with escapes)
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct MovieRef<'a> {
        #[serde(borrow)]
        title: std::borrow::Cow<'a, str>,
    }

    let fixture = json!({
        "took": 1,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": {
            "hits": [
                { "_index": "movies", "_id": "1", "_source": { "title": "Moneyball", "year": 2011 } },
                // `_source: false` in the request, or a mapping disabling it
                { "_index": "movies", "_id": "2" },
                { "_index": "movies", "_id": "3", "_source": { "title": "Capote" } },
                { "_index": "movies", "_id": "4", "_source": { "title": "Say \"hi\"" } }
            ]
        }
    });
    let resp: SearchResponse = serde_json::from_str(&fixture.to_string()).unwrap();
    let hits = &resp.hits.hits;

    assert_eq!(
        hits[0].source_as::<Movie>().unwrap(),
        Some(Movie {
            title: "Moneyball".into(),
            year: 2011
        })
    );
    let borrowed = hits[0].source_as::<MovieRef>().unwrap().unwrap();
    assert_eq!(borrowed.title, "Moneyball");
    assert!(matches!(borrowed.title, std::borrow::Cow::Borrowed(_)));
    // a string with escapes is unescaped into an owned value
    let escaped = hits[3].source_as::<MovieRef>().unwrap().unwrap();
    assert_eq!(escaped.title, "Say \"hi\"");
    // no `_source`: Ok(None), not a panic
    assert_eq!(hits[1].source_as::<Movie>().unwrap(), None);
    // a document not matching the type: an error naming the problem
    let err = hits[2].source_as::<Movie>().unwrap_err();
    assert!(err.to_string().contains("missing field `year`"), "{}", err);
    // untyped access stays available
    let any: Value = hits[2].source_as().unwrap().unwrap();
    assert_eq!(any["title"], "Capote");
}

#[test]
fn source_as_is_available_on_get_responses() {
    let fixture = json!({
        "_index": "movies", "_id": "1", "_version": 1, "_seq_no": 0, "_primary_term": 1,
        "found": true, "_source": { "title": "Moneyball" }
    });
    let resp: GetResponse = serde_json::from_str(&fixture.to_string()).unwrap();
    let doc: Value = resp.source_as().unwrap().unwrap();
    assert_eq!(doc["title"], "Moneyball");

    let missing: GetResponse =
        serde_json::from_str(r#"{"_index":"movies","_id":"x","found":false}"#).unwrap();
    assert_eq!(missing.source_as::<Value>().unwrap(), None);
}

/// A search response whose `hits.total` is `total` (omitted when `None`),
/// with one hit carrying inner hits counted exactly
fn search_with_total(total: Option<Value>) -> SearchResponse {
    let mut hits = json!({
        "hits": [{
            "_index": "movies", "_id": "1",
            "inner_hits": { "cast": { "hits": {
                "total": { "value": 2, "relation": "eq" }, "hits": []
            } } }
        }]
    });
    if let Some(total) = total {
        hits["total"] = total;
    }
    serde_json::from_value(json!({
        "took": 1, "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": hits
    }))
    .unwrap()
}

#[test]
fn total_hit_count_reads_every_form_of_hits_total() {
    use opensearch::types::search::TotalHitsRelation;

    // the default: an exact count
    let resp = search_with_total(Some(json!({ "value": 3, "relation": "eq" })));
    let total = resp.hits.total.as_ref().unwrap();
    assert_eq!((total.value(), total.is_accurate()), (3, true));
    assert_eq!(total.relation(), &TotalHitsRelation::Eq);
    assert_eq!(resp.hits.total_value(), Some(3));

    // `track_total_hits` exceeded: a lower bound
    let resp = search_with_total(Some(json!({ "value": 10000, "relation": "gte" })));
    let total = resp.hits.total.as_ref().unwrap();
    assert_eq!((total.value(), total.is_accurate()), (10000, false));
    assert_eq!(total.relation(), &TotalHitsRelation::Gte);

    // `rest_total_hits_as_int=true`: always exact
    let resp = search_with_total(Some(json!(3)));
    let total = resp.hits.total.as_ref().unwrap();
    assert_eq!((total.value(), total.is_accurate()), (3, true));
    assert_eq!(total.relation(), &TotalHitsRelation::Eq);

    // a relation this client does not know is not taken as exact
    let resp = search_with_total(Some(json!({ "value": 5, "relation": "approx" })));
    let total = resp.hits.total.as_ref().unwrap();
    assert_eq!((total.value(), total.is_accurate()), (5, false));
    assert_eq!(
        total.relation(),
        &TotalHitsRelation::Unknown("approx".to_string())
    );

    // `track_total_hits: false`: no count, rather than zero
    assert_eq!(search_with_total(None).hits.total_value(), None);
}

#[test]
fn total_hit_count_is_shared_by_the_hits_of_every_kind() {
    use opensearch::types::TotalHitCount;

    fn describe(total: &impl TotalHitCount) -> String {
        if total.is_accurate() {
            format!("{}", total.value())
        } else {
            format!("{}+", total.value())
        }
    }

    let resp = search_with_total(Some(json!({ "value": 10000, "relation": "gte" })));
    // the top level hits and the inner hits are different types
    let top = resp.hits.total.as_ref().unwrap();
    let inner = &resp.hits.hits[0].inner_hits.as_ref().unwrap()["cast"].hits;
    assert_eq!(describe(top), "10000+");
    assert_eq!(describe(inner.total.as_ref().unwrap()), "2");
    assert_eq!(inner.total_value(), Some(2));

    // the object form itself
    match top {
        opensearch::types::search::SearchResponseHitsTotal::TotalHits(t) => {
            assert_eq!(describe(t), "10000+")
        }
        other => panic!("unexpected {:?}", other),
    }
}
