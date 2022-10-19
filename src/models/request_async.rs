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
pub struct RequestAsync {
    /// The UUID of the request. Use this to retrieve the request status in the future
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Any extra information from the horde about this request
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RequestAsync {
    pub fn new() -> RequestAsync {
        RequestAsync {
            id: None,
            message: None,
        }
    }
}


