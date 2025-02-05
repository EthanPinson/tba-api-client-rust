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

/// MatchScoreBreakdown2023 : See the 2023 FMS API documentation for a description of each value. https://frc-api-docs.firstinspires.org
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2023 {
    #[serde(rename = "blue")]
    pub blue: Box<models::MatchScoreBreakdown2023Alliance>,
    #[serde(rename = "red")]
    pub red: Box<models::MatchScoreBreakdown2023Alliance>,
}

impl MatchScoreBreakdown2023 {
    /// See the 2023 FMS API documentation for a description of each value. https://frc-api-docs.firstinspires.org
    pub fn new(blue: models::MatchScoreBreakdown2023Alliance, red: models::MatchScoreBreakdown2023Alliance) -> MatchScoreBreakdown2023 {
        MatchScoreBreakdown2023 {
            blue: Box::new(blue),
            red: Box::new(red),
        }
    }
}

