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
pub struct DistrictRankingEventPointsInner {
    /// `true` if this event is a District Championship event.
    #[serde(rename = "district_cmp")]
    pub district_cmp: bool,
    /// Total points awarded at this event.
    #[serde(rename = "total")]
    pub total: i32,
    /// Points awarded for alliance selection.
    #[serde(rename = "alliance_points")]
    pub alliance_points: i32,
    /// Points awarded for elimination match performance.
    #[serde(rename = "elim_points")]
    pub elim_points: i32,
    /// Points awarded for event awards.
    #[serde(rename = "award_points")]
    pub award_points: i32,
    /// TBA Event key for this event.
    #[serde(rename = "event_key")]
    pub event_key: String,
    /// Points awarded for qualification match performance.
    #[serde(rename = "qual_points")]
    pub qual_points: i32,
}

impl DistrictRankingEventPointsInner {
    pub fn new(district_cmp: bool, total: i32, alliance_points: i32, elim_points: i32, award_points: i32, event_key: String, qual_points: i32) -> DistrictRankingEventPointsInner {
        DistrictRankingEventPointsInner {
            district_cmp,
            total,
            alliance_points,
            elim_points,
            award_points,
            event_key,
            qual_points,
        }
    }
}

