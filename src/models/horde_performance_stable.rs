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
pub struct HordePerformanceStable {
    /// The amount of waiting and processing requests currently in this Horde
    #[serde(rename = "queued_requests", skip_serializing_if = "Option::is_none")]
    pub queued_requests: Option<i32>,
    /// How many workers are actively processing image generations in this Horde in the past 5 minutes
    #[serde(rename = "worker_count", skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
    /// The amount of megapixelsteps in waiting and processing requests currently in this Horde
    #[serde(rename = "queued_megapixelsteps", skip_serializing_if = "Option::is_none")]
    pub queued_megapixelsteps: Option<f32>,
    /// How many megapixelsteps this Horde generated in the last minute
    #[serde(rename = "past_minute_megapixelsteps", skip_serializing_if = "Option::is_none")]
    pub past_minute_megapixelsteps: Option<f32>,
}

impl HordePerformanceStable {
    pub fn new() -> HordePerformanceStable {
        HordePerformanceStable {
            queued_requests: None,
            worker_count: None,
            queued_megapixelsteps: None,
            past_minute_megapixelsteps: None,
        }
    }
}

