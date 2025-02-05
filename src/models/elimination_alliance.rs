/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.9.9
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EliminationAlliance {
    /// Alliance name, may be null.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "backup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub backup: Option<Option<Box<models::EliminationAllianceBackup>>>,
    /// List of teams that declined the alliance.
    #[serde(rename = "declines")]
    pub declines: Vec<String>,
    /// List of team keys picked for the alliance. First pick is captain.
    #[serde(rename = "picks")]
    pub picks: Vec<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::EliminationAllianceStatus>>,
}

impl EliminationAlliance {
    pub fn new(declines: Vec<String>, picks: Vec<String>) -> EliminationAlliance {
        EliminationAlliance {
            name: None,
            backup: None,
            declines,
            picks,
            status: None,
        }
    }
}

