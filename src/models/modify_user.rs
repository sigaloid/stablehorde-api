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
pub struct ModifyUser {
    /// The new total Kudos this user has after this request
    #[serde(rename = "new_kudos", skip_serializing_if = "Option::is_none")]
    pub new_kudos: Option<f32>,
    /// The request concurrency this user has after this request
    #[serde(rename = "concurrency", skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i32>,
    /// Multiplies the amount of kudos lost when generating images.
    #[serde(rename = "usage_multiplier", skip_serializing_if = "Option::is_none")]
    pub usage_multiplier: Option<f32>,
    /// Whether this user has been invited to join a worker to the horde and how many of them. When 0, this user cannot add (new) workers to the horde.
    #[serde(rename = "worker_invited", skip_serializing_if = "Option::is_none")]
    pub worker_invited: Option<i32>,
    /// The user's new moderator status.
    #[serde(rename = "moderator", skip_serializing_if = "Option::is_none")]
    pub moderator: Option<bool>,
    /// The user's new public_workers status.
    #[serde(rename = "public_workers", skip_serializing_if = "Option::is_none")]
    pub public_workers: Option<bool>,
    /// The user's new username.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The user's new monthly kudos total
    #[serde(rename = "monthly_kudos", skip_serializing_if = "Option::is_none")]
    pub monthly_kudos: Option<i32>,
    /// The user's new trusted status
    #[serde(rename = "trusted", skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
}

impl ModifyUser {
    pub fn new() -> ModifyUser {
        ModifyUser {
            new_kudos: None,
            concurrency: None,
            usage_multiplier: None,
            worker_invited: None,
            moderator: None,
            public_workers: None,
            username: None,
            monthly_kudos: None,
            trusted: None,
        }
    }
}

