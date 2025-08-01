/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.10.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EliminationAllianceBackup : Backup team called in, may be null.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EliminationAllianceBackup {
    /// Team key that was called in as the backup.
    #[serde(rename = "in")]
    pub r#in: String,
    /// Team key that was replaced by the backup team.
    #[serde(rename = "out")]
    pub out: String,
}

impl EliminationAllianceBackup {
    /// Backup team called in, may be null.
    pub fn new(r#in: String, out: String) -> EliminationAllianceBackup {
        EliminationAllianceBackup {
            r#in,
            out,
        }
    }
}

