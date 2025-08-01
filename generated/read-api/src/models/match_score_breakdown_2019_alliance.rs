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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2019Alliance {
    #[serde(rename = "adjustPoints", skip_serializing_if = "Option::is_none")]
    pub adjust_points: Option<i32>,
    #[serde(rename = "autoPoints", skip_serializing_if = "Option::is_none")]
    pub auto_points: Option<i32>,
    #[serde(rename = "bay1", skip_serializing_if = "Option::is_none")]
    pub bay1: Option<String>,
    #[serde(rename = "bay2", skip_serializing_if = "Option::is_none")]
    pub bay2: Option<String>,
    #[serde(rename = "bay3", skip_serializing_if = "Option::is_none")]
    pub bay3: Option<String>,
    #[serde(rename = "bay4", skip_serializing_if = "Option::is_none")]
    pub bay4: Option<String>,
    #[serde(rename = "bay5", skip_serializing_if = "Option::is_none")]
    pub bay5: Option<String>,
    #[serde(rename = "bay6", skip_serializing_if = "Option::is_none")]
    pub bay6: Option<String>,
    #[serde(rename = "bay7", skip_serializing_if = "Option::is_none")]
    pub bay7: Option<String>,
    #[serde(rename = "bay8", skip_serializing_if = "Option::is_none")]
    pub bay8: Option<String>,
    #[serde(rename = "cargoPoints", skip_serializing_if = "Option::is_none")]
    pub cargo_points: Option<i32>,
    #[serde(rename = "completeRocketRankingPoint", skip_serializing_if = "Option::is_none")]
    pub complete_rocket_ranking_point: Option<bool>,
    #[serde(rename = "completedRocketFar", skip_serializing_if = "Option::is_none")]
    pub completed_rocket_far: Option<bool>,
    #[serde(rename = "completedRocketNear", skip_serializing_if = "Option::is_none")]
    pub completed_rocket_near: Option<bool>,
    #[serde(rename = "endgameRobot1", skip_serializing_if = "Option::is_none")]
    pub endgame_robot1: Option<String>,
    #[serde(rename = "endgameRobot2", skip_serializing_if = "Option::is_none")]
    pub endgame_robot2: Option<String>,
    #[serde(rename = "endgameRobot3", skip_serializing_if = "Option::is_none")]
    pub endgame_robot3: Option<String>,
    #[serde(rename = "foulCount", skip_serializing_if = "Option::is_none")]
    pub foul_count: Option<i32>,
    #[serde(rename = "foulPoints", skip_serializing_if = "Option::is_none")]
    pub foul_points: Option<i32>,
    #[serde(rename = "habClimbPoints", skip_serializing_if = "Option::is_none")]
    pub hab_climb_points: Option<i32>,
    #[serde(rename = "habDockingRankingPoint", skip_serializing_if = "Option::is_none")]
    pub hab_docking_ranking_point: Option<bool>,
    #[serde(rename = "habLineRobot1", skip_serializing_if = "Option::is_none")]
    pub hab_line_robot1: Option<String>,
    #[serde(rename = "habLineRobot2", skip_serializing_if = "Option::is_none")]
    pub hab_line_robot2: Option<String>,
    #[serde(rename = "habLineRobot3", skip_serializing_if = "Option::is_none")]
    pub hab_line_robot3: Option<String>,
    #[serde(rename = "hatchPanelPoints", skip_serializing_if = "Option::is_none")]
    pub hatch_panel_points: Option<i32>,
    #[serde(rename = "lowLeftRocketFar", skip_serializing_if = "Option::is_none")]
    pub low_left_rocket_far: Option<String>,
    #[serde(rename = "lowLeftRocketNear", skip_serializing_if = "Option::is_none")]
    pub low_left_rocket_near: Option<String>,
    #[serde(rename = "lowRightRocketFar", skip_serializing_if = "Option::is_none")]
    pub low_right_rocket_far: Option<String>,
    #[serde(rename = "lowRightRocketNear", skip_serializing_if = "Option::is_none")]
    pub low_right_rocket_near: Option<String>,
    #[serde(rename = "midLeftRocketFar", skip_serializing_if = "Option::is_none")]
    pub mid_left_rocket_far: Option<String>,
    #[serde(rename = "midLeftRocketNear", skip_serializing_if = "Option::is_none")]
    pub mid_left_rocket_near: Option<String>,
    #[serde(rename = "midRightRocketFar", skip_serializing_if = "Option::is_none")]
    pub mid_right_rocket_far: Option<String>,
    #[serde(rename = "midRightRocketNear", skip_serializing_if = "Option::is_none")]
    pub mid_right_rocket_near: Option<String>,
    #[serde(rename = "preMatchBay1", skip_serializing_if = "Option::is_none")]
    pub pre_match_bay1: Option<String>,
    #[serde(rename = "preMatchBay2", skip_serializing_if = "Option::is_none")]
    pub pre_match_bay2: Option<String>,
    #[serde(rename = "preMatchBay3", skip_serializing_if = "Option::is_none")]
    pub pre_match_bay3: Option<String>,
    #[serde(rename = "preMatchBay6", skip_serializing_if = "Option::is_none")]
    pub pre_match_bay6: Option<String>,
    #[serde(rename = "preMatchBay7", skip_serializing_if = "Option::is_none")]
    pub pre_match_bay7: Option<String>,
    #[serde(rename = "preMatchBay8", skip_serializing_if = "Option::is_none")]
    pub pre_match_bay8: Option<String>,
    #[serde(rename = "preMatchLevelRobot1", skip_serializing_if = "Option::is_none")]
    pub pre_match_level_robot1: Option<String>,
    #[serde(rename = "preMatchLevelRobot2", skip_serializing_if = "Option::is_none")]
    pub pre_match_level_robot2: Option<String>,
    #[serde(rename = "preMatchLevelRobot3", skip_serializing_if = "Option::is_none")]
    pub pre_match_level_robot3: Option<String>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<i32>,
    #[serde(rename = "sandStormBonusPoints", skip_serializing_if = "Option::is_none")]
    pub sand_storm_bonus_points: Option<i32>,
    #[serde(rename = "techFoulCount", skip_serializing_if = "Option::is_none")]
    pub tech_foul_count: Option<i32>,
    #[serde(rename = "teleopPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_points: Option<i32>,
    #[serde(rename = "topLeftRocketFar", skip_serializing_if = "Option::is_none")]
    pub top_left_rocket_far: Option<String>,
    #[serde(rename = "topLeftRocketNear", skip_serializing_if = "Option::is_none")]
    pub top_left_rocket_near: Option<String>,
    #[serde(rename = "topRightRocketFar", skip_serializing_if = "Option::is_none")]
    pub top_right_rocket_far: Option<String>,
    #[serde(rename = "topRightRocketNear", skip_serializing_if = "Option::is_none")]
    pub top_right_rocket_near: Option<String>,
    #[serde(rename = "totalPoints", skip_serializing_if = "Option::is_none")]
    pub total_points: Option<i32>,
}

impl MatchScoreBreakdown2019Alliance {
    pub fn new() -> MatchScoreBreakdown2019Alliance {
        MatchScoreBreakdown2019Alliance {
            adjust_points: None,
            auto_points: None,
            bay1: None,
            bay2: None,
            bay3: None,
            bay4: None,
            bay5: None,
            bay6: None,
            bay7: None,
            bay8: None,
            cargo_points: None,
            complete_rocket_ranking_point: None,
            completed_rocket_far: None,
            completed_rocket_near: None,
            endgame_robot1: None,
            endgame_robot2: None,
            endgame_robot3: None,
            foul_count: None,
            foul_points: None,
            hab_climb_points: None,
            hab_docking_ranking_point: None,
            hab_line_robot1: None,
            hab_line_robot2: None,
            hab_line_robot3: None,
            hatch_panel_points: None,
            low_left_rocket_far: None,
            low_left_rocket_near: None,
            low_right_rocket_far: None,
            low_right_rocket_near: None,
            mid_left_rocket_far: None,
            mid_left_rocket_near: None,
            mid_right_rocket_far: None,
            mid_right_rocket_near: None,
            pre_match_bay1: None,
            pre_match_bay2: None,
            pre_match_bay3: None,
            pre_match_bay6: None,
            pre_match_bay7: None,
            pre_match_bay8: None,
            pre_match_level_robot1: None,
            pre_match_level_robot2: None,
            pre_match_level_robot3: None,
            rp: None,
            sand_storm_bonus_points: None,
            tech_foul_count: None,
            teleop_points: None,
            top_left_rocket_far: None,
            top_left_rocket_near: None,
            top_right_rocket_far: None,
            top_right_rocket_near: None,
            total_points: None,
        }
    }
}

