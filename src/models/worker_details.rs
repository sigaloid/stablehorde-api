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
pub struct WorkerDetails {
    /// The Name given to this worker.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The UUID of this worker.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// How many images this worker has generated.
    #[serde(rename = "requests_fulfilled", skip_serializing_if = "Option::is_none")]
    pub requests_fulfilled: Option<i32>,
    /// How many Kudos this worker has been rewarded in total.
    #[serde(rename = "kudos_rewards", skip_serializing_if = "Option::is_none")]
    pub kudos_rewards: Option<f32>,
    #[serde(rename = "kudos_details", skip_serializing_if = "Option::is_none")]
    pub kudos_details: Option<Box<crate::models::WorkerKudosDetails>>,
    /// The average performance of this worker in human readable form.
    #[serde(rename = "performance", skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    /// The amount of seconds this worker has been online for this Horde.
    #[serde(rename = "uptime", skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i32>,
    /// When True, this worker will not pick up any new requests
    #[serde(rename = "maintenance_mode", skip_serializing_if = "Option::is_none")]
    pub maintenance_mode: Option<bool>,
    /// (Privileged) When True, this worker not be given any new requests.
    #[serde(rename = "paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Extra information or comments about this worker provided by its owner.
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// Whether this worker can generate NSFW requests or not.
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Privileged or public if the owner has allowed it. The alias of the owner of this worker.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// The worker is trusted to return valid generations.
    #[serde(rename = "trusted", skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// (Privileged) How much suspicion this worker has accumulated
    #[serde(rename = "suspicious", skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<i32>,
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
}

impl WorkerDetails {
    pub fn new() -> WorkerDetails {
        WorkerDetails {
            name: None,
            id: None,
            requests_fulfilled: None,
            kudos_rewards: None,
            kudos_details: None,
            performance: None,
            uptime: None,
            maintenance_mode: None,
            paused: None,
            info: None,
            nsfw: None,
            owner: None,
            trusted: None,
            suspicious: None,
            models: None,
        }
    }
}


