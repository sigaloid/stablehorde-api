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
pub struct ContributionsDetailsStable {
    /// How many images this user has generated
    #[serde(rename = "fulfillments", skip_serializing_if = "Option::is_none")]
    pub fulfillments: Option<i32>,
    /// How many megapixelsteps this user has generated
    #[serde(rename = "megapixelsteps", skip_serializing_if = "Option::is_none")]
    pub megapixelsteps: Option<f32>,
}

impl ContributionsDetailsStable {
    pub fn new() -> ContributionsDetailsStable {
        ContributionsDetailsStable {
            fulfillments: None,
            megapixelsteps: None,
        }
    }
}


