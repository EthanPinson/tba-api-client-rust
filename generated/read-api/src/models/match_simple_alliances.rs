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

/// MatchSimpleAlliances : A list of alliances, the teams on the alliances, and their score.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchSimpleAlliances {
    #[serde(rename = "red")]
    pub red: Box<models::MatchAlliance>,
    #[serde(rename = "blue")]
    pub blue: Box<models::MatchAlliance>,
}

impl MatchSimpleAlliances {
    /// A list of alliances, the teams on the alliances, and their score.
    pub fn new(red: models::MatchAlliance, blue: models::MatchAlliance) -> MatchSimpleAlliances {
        MatchSimpleAlliances {
            red: Box::new(red),
            blue: Box::new(blue),
        }
    }
}

