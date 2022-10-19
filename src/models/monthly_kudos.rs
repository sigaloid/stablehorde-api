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
pub struct MonthlyKudos {
    /// How much recurring Kudos this user receives monthly.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// Last date this user received monthly Kudos.
    #[serde(rename = "last_received", skip_serializing_if = "Option::is_none")]
    pub last_received: Option<String>,
}

impl MonthlyKudos {
    pub fn new() -> MonthlyKudos {
        MonthlyKudos {
            amount: None,
            last_received: None,
        }
    }
}


