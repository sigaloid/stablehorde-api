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
pub struct RequestStatusStableAllOf {
    #[serde(rename = "generations", skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<crate::models::GenerationStable>>,
}

impl RequestStatusStableAllOf {
    pub fn new() -> RequestStatusStableAllOf {
        RequestStatusStableAllOf {
            generations: None,
        }
    }
}


