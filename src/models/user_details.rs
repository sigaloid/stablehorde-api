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
pub struct UserDetails {
    /// The user's unique Username. It is a combination of their chosen alias plus their ID.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The user unique ID. It is always an integer.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The amount of Kudos this user has. The amount of Kudos determines the priority when requesting image generations.
    #[serde(rename = "kudos", skip_serializing_if = "Option::is_none")]
    pub kudos: Option<f32>,
    /// (Privileged) The amount of Evaluating Kudos this untrusted user has from generations and uptime. When this number reaches 50000, they automatically become trusted.
    #[serde(rename = "evaluating_kudos", skip_serializing_if = "Option::is_none")]
    pub evaluating_kudos: Option<f32>,
    /// How many concurrent generations this user may request.
    #[serde(rename = "concurrency", skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i32>,
    /// Whether this user has been invited to join a worker to the horde and how many of them. When 0, this user cannot add (new) workers to the horde.
    #[serde(rename = "worker_invited", skip_serializing_if = "Option::is_none")]
    pub worker_invited: Option<i32>,
    /// This user is a Horde moderator.
    #[serde(rename = "moderator", skip_serializing_if = "Option::is_none")]
    pub moderator: Option<bool>,
    #[serde(rename = "kudos_details", skip_serializing_if = "Option::is_none")]
    pub kudos_details: Option<Box<crate::models::UserKudosDetails>>,
    /// How many workers this user has created (active or inactive)
    #[serde(rename = "worker_count", skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i32>,
    #[serde(rename = "worker_ids", skip_serializing_if = "Option::is_none")]
    pub worker_ids: Option<Vec<String>>,
    #[serde(rename = "monthly_kudos", skip_serializing_if = "Option::is_none")]
    pub monthly_kudos: Option<Box<crate::models::MonthlyKudos>>,
    /// This user is a trusted member of the Horde.
    #[serde(rename = "trusted", skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    /// (Privileged) How much suspicion this user has accumulated
    #[serde(rename = "suspicious", skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<i32>,
}

impl UserDetails {
    pub fn new() -> UserDetails {
        UserDetails {
            username: None,
            id: None,
            kudos: None,
            evaluating_kudos: None,
            concurrency: None,
            worker_invited: None,
            moderator: None,
            kudos_details: None,
            worker_count: None,
            worker_ids: None,
            monthly_kudos: None,
            trusted: None,
            suspicious: None,
        }
    }
}


