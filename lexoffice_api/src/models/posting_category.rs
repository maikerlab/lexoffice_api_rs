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
pub struct PostingCategory {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "contactRequired", skip_serializing_if = "Option::is_none")]
    pub contact_required: Option<bool>,
    #[serde(rename = "splitAllowed", skip_serializing_if = "Option::is_none")]
    pub split_allowed: Option<bool>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl PostingCategory {
    pub fn new(id: uuid::Uuid, name: String, r#type: Type) -> PostingCategory {
        PostingCategory {
            id,
            name,
            r#type,
            contact_required: None,
            split_allowed: None,
            group_name: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "income")]
    Income,
    #[serde(rename = "outgo")]
    Outgo,
}

impl Default for Type {
    fn default() -> Type {
        Self::Income
    }
}

