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
pub struct GenerationStableAllOf {
    /// The generated image as a Base64-encoded .webp file
    #[serde(rename = "img", skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
    /// The seed which generated this image
    #[serde(rename = "seed", skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
}

impl GenerationStableAllOf {
    pub fn new() -> GenerationStableAllOf {
        GenerationStableAllOf {
            img: None,
            seed: None,
        }
    }
}


