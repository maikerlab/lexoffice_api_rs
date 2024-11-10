/*
 * lexoffice Public API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfileCreated {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userEmail", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl UserProfileCreated {
    pub fn new() -> UserProfileCreated {
        UserProfileCreated {
            user_id: None,
            user_name: None,
            user_email: None,
            date: None,
        }
    }
}

