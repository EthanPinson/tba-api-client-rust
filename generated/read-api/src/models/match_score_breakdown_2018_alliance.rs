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
pub struct MatchScoreBreakdown2018Alliance {
    #[serde(rename = "adjustPoints", skip_serializing_if = "Option::is_none")]
    pub adjust_points: Option<i32>,
    #[serde(rename = "autoOwnershipPoints", skip_serializing_if = "Option::is_none")]
    pub auto_ownership_points: Option<i32>,
    #[serde(rename = "autoPoints", skip_serializing_if = "Option::is_none")]
    pub auto_points: Option<i32>,
    #[serde(rename = "autoQuestRankingPoint", skip_serializing_if = "Option::is_none")]
    pub auto_quest_ranking_point: Option<bool>,
    #[serde(rename = "autoRobot1", skip_serializing_if = "Option::is_none")]
    pub auto_robot1: Option<String>,
    #[serde(rename = "autoRobot2", skip_serializing_if = "Option::is_none")]
    pub auto_robot2: Option<String>,
    #[serde(rename = "autoRobot3", skip_serializing_if = "Option::is_none")]
    pub auto_robot3: Option<String>,
    #[serde(rename = "autoRunPoints", skip_serializing_if = "Option::is_none")]
    pub auto_run_points: Option<i32>,
    #[serde(rename = "autoScaleOwnershipSec", skip_serializing_if = "Option::is_none")]
    pub auto_scale_ownership_sec: Option<i32>,
    #[serde(rename = "autoSwitchAtZero", skip_serializing_if = "Option::is_none")]
    pub auto_switch_at_zero: Option<bool>,
    #[serde(rename = "autoSwitchOwnershipSec", skip_serializing_if = "Option::is_none")]
    pub auto_switch_ownership_sec: Option<i32>,
    #[serde(rename = "endgamePoints", skip_serializing_if = "Option::is_none")]
    pub endgame_points: Option<i32>,
    #[serde(rename = "endgameRobot1", skip_serializing_if = "Option::is_none")]
    pub endgame_robot1: Option<String>,
    #[serde(rename = "endgameRobot2", skip_serializing_if = "Option::is_none")]
    pub endgame_robot2: Option<String>,
    #[serde(rename = "endgameRobot3", skip_serializing_if = "Option::is_none")]
    pub endgame_robot3: Option<String>,
    #[serde(rename = "faceTheBossRankingPoint", skip_serializing_if = "Option::is_none")]
    pub face_the_boss_ranking_point: Option<bool>,
    #[serde(rename = "foulCount", skip_serializing_if = "Option::is_none")]
    pub foul_count: Option<i32>,
    #[serde(rename = "foulPoints", skip_serializing_if = "Option::is_none")]
    pub foul_points: Option<i32>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<i32>,
    #[serde(rename = "techFoulCount", skip_serializing_if = "Option::is_none")]
    pub tech_foul_count: Option<i32>,
    #[serde(rename = "teleopOwnershipPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_ownership_points: Option<i32>,
    #[serde(rename = "teleopPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_points: Option<i32>,
    #[serde(rename = "teleopScaleBoostSec", skip_serializing_if = "Option::is_none")]
    pub teleop_scale_boost_sec: Option<i32>,
    #[serde(rename = "teleopScaleForceSec", skip_serializing_if = "Option::is_none")]
    pub teleop_scale_force_sec: Option<i32>,
    #[serde(rename = "teleopScaleOwnershipSec", skip_serializing_if = "Option::is_none")]
    pub teleop_scale_ownership_sec: Option<i32>,
    #[serde(rename = "teleopSwitchBoostSec", skip_serializing_if = "Option::is_none")]
    pub teleop_switch_boost_sec: Option<i32>,
    #[serde(rename = "teleopSwitchForceSec", skip_serializing_if = "Option::is_none")]
    pub teleop_switch_force_sec: Option<i32>,
    #[serde(rename = "teleopSwitchOwnershipSec", skip_serializing_if = "Option::is_none")]
    pub teleop_switch_ownership_sec: Option<i32>,
    #[serde(rename = "totalPoints", skip_serializing_if = "Option::is_none")]
    pub total_points: Option<i32>,
    #[serde(rename = "vaultBoostPlayed", skip_serializing_if = "Option::is_none")]
    pub vault_boost_played: Option<i32>,
    #[serde(rename = "vaultBoostTotal", skip_serializing_if = "Option::is_none")]
    pub vault_boost_total: Option<i32>,
    #[serde(rename = "vaultForcePlayed", skip_serializing_if = "Option::is_none")]
    pub vault_force_played: Option<i32>,
    #[serde(rename = "vaultForceTotal", skip_serializing_if = "Option::is_none")]
    pub vault_force_total: Option<i32>,
    #[serde(rename = "vaultLevitatePlayed", skip_serializing_if = "Option::is_none")]
    pub vault_levitate_played: Option<i32>,
    #[serde(rename = "vaultLevitateTotal", skip_serializing_if = "Option::is_none")]
    pub vault_levitate_total: Option<i32>,
    #[serde(rename = "vaultPoints", skip_serializing_if = "Option::is_none")]
    pub vault_points: Option<i32>,
    /// Unofficial TBA-computed value of the FMS provided GameData given to the alliance teams at the start of the match. 3 Character String containing `L` and `R` only. The first character represents the near switch, the 2nd the scale, and the 3rd the far, opposing, switch from the alliance's perspective. An `L` in a position indicates the platform on the left will be lit for the alliance while an `R` will indicate the right platform will be lit for the alliance. See also [WPI Screen Steps](https://wpilib.screenstepslive.com/s/currentCS/m/getting_started/l/826278-2018-game-data-details).
    #[serde(rename = "tba_gameData", skip_serializing_if = "Option::is_none")]
    pub tba_game_data: Option<String>,
}

impl MatchScoreBreakdown2018Alliance {
    pub fn new() -> MatchScoreBreakdown2018Alliance {
        MatchScoreBreakdown2018Alliance {
            adjust_points: None,
            auto_ownership_points: None,
            auto_points: None,
            auto_quest_ranking_point: None,
            auto_robot1: None,
            auto_robot2: None,
            auto_robot3: None,
            auto_run_points: None,
            auto_scale_ownership_sec: None,
            auto_switch_at_zero: None,
            auto_switch_ownership_sec: None,
            endgame_points: None,
            endgame_robot1: None,
            endgame_robot2: None,
            endgame_robot3: None,
            face_the_boss_ranking_point: None,
            foul_count: None,
            foul_points: None,
            rp: None,
            tech_foul_count: None,
            teleop_ownership_points: None,
            teleop_points: None,
            teleop_scale_boost_sec: None,
            teleop_scale_force_sec: None,
            teleop_scale_ownership_sec: None,
            teleop_switch_boost_sec: None,
            teleop_switch_force_sec: None,
            teleop_switch_ownership_sec: None,
            total_points: None,
            vault_boost_played: None,
            vault_boost_total: None,
            vault_force_played: None,
            vault_force_total: None,
            vault_levitate_played: None,
            vault_levitate_total: None,
            vault_points: None,
            tba_game_data: None,
        }
    }
}

