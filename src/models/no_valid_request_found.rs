/*
 * Stable Horde
 *
 * The API documentation for the Stable Horde
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NoValidRequestFound {
    /// How many waiting requests were skipped because they demanded a specific worker
    #[serde(rename = "worker_id", skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<i32>,
    /// How many waiting requests were skipped because they demanded a specific worker
    #[serde(rename = "performance", skip_serializing_if = "Option::is_none")]
    pub performance: Option<i32>,
    /// How many waiting requests were skipped because they demanded a nsfw generation which this worker does not provide.
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<i32>,
    /// How many waiting requests were skipped because they demanded a generation with a word that this worker does not accept.
    #[serde(rename = "blacklist", skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<i32>,
    /// How many waiting requests were skipped because they demanded a trusted worker which this worker is not.
    #[serde(rename = "untrusted", skip_serializing_if = "Option::is_none")]
    pub untrusted: Option<i32>,
    /// How many waiting requests were skipped because they demanded a different model than what this worker provides.
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<i32>,
}

impl NoValidRequestFound {
    pub fn new() -> NoValidRequestFound {
        NoValidRequestFound {
            worker_id: None,
            performance: None,
            nsfw: None,
            blacklist: None,
            untrusted: None,
            models: None,
        }
    }
}


