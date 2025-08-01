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
pub struct MatchScoreBreakdown2024Alliance {
    #[serde(rename = "adjustPoints", skip_serializing_if = "Option::is_none")]
    pub adjust_points: Option<i32>,
    #[serde(rename = "autoAmpNoteCount", skip_serializing_if = "Option::is_none")]
    pub auto_amp_note_count: Option<i32>,
    #[serde(rename = "autoAmpNotePoints", skip_serializing_if = "Option::is_none")]
    pub auto_amp_note_points: Option<i32>,
    #[serde(rename = "autoLeavePoints", skip_serializing_if = "Option::is_none")]
    pub auto_leave_points: Option<i32>,
    #[serde(rename = "autoLineRobot1", skip_serializing_if = "Option::is_none")]
    pub auto_line_robot1: Option<String>,
    #[serde(rename = "autoLineRobot2", skip_serializing_if = "Option::is_none")]
    pub auto_line_robot2: Option<String>,
    #[serde(rename = "autoLineRobot3", skip_serializing_if = "Option::is_none")]
    pub auto_line_robot3: Option<String>,
    #[serde(rename = "autoPoints", skip_serializing_if = "Option::is_none")]
    pub auto_points: Option<i32>,
    #[serde(rename = "autoSpeakerNoteCount", skip_serializing_if = "Option::is_none")]
    pub auto_speaker_note_count: Option<i32>,
    #[serde(rename = "autoSpeakerNotePoints", skip_serializing_if = "Option::is_none")]
    pub auto_speaker_note_points: Option<i32>,
    #[serde(rename = "autoTotalNotePoints", skip_serializing_if = "Option::is_none")]
    pub auto_total_note_points: Option<i32>,
    #[serde(rename = "coopNotePlayed", skip_serializing_if = "Option::is_none")]
    pub coop_note_played: Option<bool>,
    #[serde(rename = "coopertitionBonusAchieved", skip_serializing_if = "Option::is_none")]
    pub coopertition_bonus_achieved: Option<bool>,
    #[serde(rename = "coopertitionCriteriaMet", skip_serializing_if = "Option::is_none")]
    pub coopertition_criteria_met: Option<bool>,
    #[serde(rename = "endGameHarmonyPoints", skip_serializing_if = "Option::is_none")]
    pub end_game_harmony_points: Option<i32>,
    #[serde(rename = "endGameNoteInTrapPoints", skip_serializing_if = "Option::is_none")]
    pub end_game_note_in_trap_points: Option<i32>,
    #[serde(rename = "endGameOnStagePoints", skip_serializing_if = "Option::is_none")]
    pub end_game_on_stage_points: Option<i32>,
    #[serde(rename = "endGameParkPoints", skip_serializing_if = "Option::is_none")]
    pub end_game_park_points: Option<i32>,
    #[serde(rename = "endGameRobot1", skip_serializing_if = "Option::is_none")]
    pub end_game_robot1: Option<String>,
    #[serde(rename = "endGameRobot2", skip_serializing_if = "Option::is_none")]
    pub end_game_robot2: Option<String>,
    #[serde(rename = "endGameRobot3", skip_serializing_if = "Option::is_none")]
    pub end_game_robot3: Option<String>,
    #[serde(rename = "endGameSpotLightBonusPoints", skip_serializing_if = "Option::is_none")]
    pub end_game_spot_light_bonus_points: Option<i32>,
    #[serde(rename = "endGameTotalStagePoints", skip_serializing_if = "Option::is_none")]
    pub end_game_total_stage_points: Option<i32>,
    #[serde(rename = "ensembleBonusAchieved", skip_serializing_if = "Option::is_none")]
    pub ensemble_bonus_achieved: Option<bool>,
    #[serde(rename = "ensembleBonusOnStageRobotsThreshold", skip_serializing_if = "Option::is_none")]
    pub ensemble_bonus_on_stage_robots_threshold: Option<i32>,
    #[serde(rename = "ensembleBonusStagePointsThreshold", skip_serializing_if = "Option::is_none")]
    pub ensemble_bonus_stage_points_threshold: Option<i32>,
    #[serde(rename = "foulCount", skip_serializing_if = "Option::is_none")]
    pub foul_count: Option<i32>,
    #[serde(rename = "foulPoints", skip_serializing_if = "Option::is_none")]
    pub foul_points: Option<i32>,
    #[serde(rename = "g206Penalty", skip_serializing_if = "Option::is_none")]
    pub g206_penalty: Option<bool>,
    #[serde(rename = "g408Penalty", skip_serializing_if = "Option::is_none")]
    pub g408_penalty: Option<bool>,
    #[serde(rename = "g424Penalty", skip_serializing_if = "Option::is_none")]
    pub g424_penalty: Option<bool>,
    #[serde(rename = "melodyBonusAchieved", skip_serializing_if = "Option::is_none")]
    pub melody_bonus_achieved: Option<bool>,
    #[serde(rename = "melodyBonusThreshold", skip_serializing_if = "Option::is_none")]
    pub melody_bonus_threshold: Option<i32>,
    #[serde(rename = "melodyBonusThresholdCoop", skip_serializing_if = "Option::is_none")]
    pub melody_bonus_threshold_coop: Option<i32>,
    #[serde(rename = "melodyBonusThresholdNonCoop", skip_serializing_if = "Option::is_none")]
    pub melody_bonus_threshold_non_coop: Option<i32>,
    #[serde(rename = "micCenterStage", skip_serializing_if = "Option::is_none")]
    pub mic_center_stage: Option<bool>,
    #[serde(rename = "micStageLeft", skip_serializing_if = "Option::is_none")]
    pub mic_stage_left: Option<bool>,
    #[serde(rename = "micStageRight", skip_serializing_if = "Option::is_none")]
    pub mic_stage_right: Option<bool>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<i32>,
    #[serde(rename = "techFoulCount", skip_serializing_if = "Option::is_none")]
    pub tech_foul_count: Option<i32>,
    #[serde(rename = "teleopAmpNoteCount", skip_serializing_if = "Option::is_none")]
    pub teleop_amp_note_count: Option<i32>,
    #[serde(rename = "teleopAmpNotePoints", skip_serializing_if = "Option::is_none")]
    pub teleop_amp_note_points: Option<i32>,
    #[serde(rename = "teleopPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_points: Option<i32>,
    #[serde(rename = "teleopSpeakerNoteAmplifiedCount", skip_serializing_if = "Option::is_none")]
    pub teleop_speaker_note_amplified_count: Option<i32>,
    #[serde(rename = "teleopSpeakerNoteAmplifiedPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_speaker_note_amplified_points: Option<i32>,
    #[serde(rename = "teleopSpeakerNoteCount", skip_serializing_if = "Option::is_none")]
    pub teleop_speaker_note_count: Option<i32>,
    #[serde(rename = "teleopSpeakerNotePoints", skip_serializing_if = "Option::is_none")]
    pub teleop_speaker_note_points: Option<i32>,
    #[serde(rename = "teleopTotalNotePoints", skip_serializing_if = "Option::is_none")]
    pub teleop_total_note_points: Option<i32>,
    #[serde(rename = "totalPoints", skip_serializing_if = "Option::is_none")]
    pub total_points: Option<i32>,
    #[serde(rename = "trapCenterStage", skip_serializing_if = "Option::is_none")]
    pub trap_center_stage: Option<bool>,
    #[serde(rename = "trapStageLeft", skip_serializing_if = "Option::is_none")]
    pub trap_stage_left: Option<bool>,
    #[serde(rename = "trapStageRight", skip_serializing_if = "Option::is_none")]
    pub trap_stage_right: Option<bool>,
}

impl MatchScoreBreakdown2024Alliance {
    pub fn new() -> MatchScoreBreakdown2024Alliance {
        MatchScoreBreakdown2024Alliance {
            adjust_points: None,
            auto_amp_note_count: None,
            auto_amp_note_points: None,
            auto_leave_points: None,
            auto_line_robot1: None,
            auto_line_robot2: None,
            auto_line_robot3: None,
            auto_points: None,
            auto_speaker_note_count: None,
            auto_speaker_note_points: None,
            auto_total_note_points: None,
            coop_note_played: None,
            coopertition_bonus_achieved: None,
            coopertition_criteria_met: None,
            end_game_harmony_points: None,
            end_game_note_in_trap_points: None,
            end_game_on_stage_points: None,
            end_game_park_points: None,
            end_game_robot1: None,
            end_game_robot2: None,
            end_game_robot3: None,
            end_game_spot_light_bonus_points: None,
            end_game_total_stage_points: None,
            ensemble_bonus_achieved: None,
            ensemble_bonus_on_stage_robots_threshold: None,
            ensemble_bonus_stage_points_threshold: None,
            foul_count: None,
            foul_points: None,
            g206_penalty: None,
            g408_penalty: None,
            g424_penalty: None,
            melody_bonus_achieved: None,
            melody_bonus_threshold: None,
            melody_bonus_threshold_coop: None,
            melody_bonus_threshold_non_coop: None,
            mic_center_stage: None,
            mic_stage_left: None,
            mic_stage_right: None,
            rp: None,
            tech_foul_count: None,
            teleop_amp_note_count: None,
            teleop_amp_note_points: None,
            teleop_points: None,
            teleop_speaker_note_amplified_count: None,
            teleop_speaker_note_amplified_points: None,
            teleop_speaker_note_count: None,
            teleop_speaker_note_points: None,
            teleop_total_note_points: None,
            total_points: None,
            trap_center_stage: None,
            trap_stage_left: None,
            trap_stage_right: None,
        }
    }
}

