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
pub struct PutWorkerSingleRequest {
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<bool>,
    #[serde(rename = "paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PutWorkerSingleRequest {
    pub fn new() -> PutWorkerSingleRequest {
        PutWorkerSingleRequest {
            maintenance: None,
            paused: None,
            info: None,
            name: None,
        }
    }
}


