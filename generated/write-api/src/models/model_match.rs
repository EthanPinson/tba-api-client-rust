/*
 * The Blue Alliance Trusted APIv1
 *
 * # Overview Import FIRST Robotics Competition Data to The Blue Alliance. The API is accessed through POST requests made to the endpoints described below. To simplify the removal of incorrect data, most endpoints must be called with the full dataset. For example, there is no endpoint to add a single award-- only to update all awards. It must be called with all awards, and the ones that are no longer present will be deleted. **The exception to this is the matches endpoint**, where there are separate endpoints to update and delete matches. Also, the **match videos endpoint** only allows the addition (and not the removal) of videos. # Authentication The X-TBA-Auth-Id and X-TBA-Auth-Sig must be included as request headers in all requests.  *auth_id*: ``  *auth_secret*: ``  *X-TBA-Auth-Id*: `auth_id`  *X-TBA-Auth-Sig*: `md5_hexdigest()`  For example, an X-TBA-Auth-Sig may look like `md5_hexdigest(ExqeZK3Gbo9v95YnqmsiADzESo9HNgyhIOYSMyRpqJqYv13EazNRaDIPPJuOXrQp/api/trusted/v1/event/2014casj/matches/delete[\"qm1\"])`  Which, after hashing, becomes: `ca5c3e5c1b0e7132e4af13f805eca0be`
 *
 * The version of the OpenAPI document: 3.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    /// Competition level of the match
    #[serde(rename = "comp_level")]
    pub comp_level: CompLevel,
    /// Set number of the match
    #[serde(rename = "set_number")]
    pub set_number: i32,
    /// Number of the match in the current set
    #[serde(rename = "match_number")]
    pub match_number: i32,
    #[serde(rename = "alliances")]
    pub alliances: Box<models::MatchAlliances>,
    /// Dict of {'red': {K1: V1, K2: V2, ...}, 'blue': {...}}. Where Kn are keys and Vn are values for those keys. Valid keys Kn vary by year. For 2014, valid keys are: 'auto', 'assist', 'truss+catch', 'teleop_goal+foul'.
    #[serde(rename = "score_breakdown", skip_serializing_if = "Option::is_none")]
    pub score_breakdown: Option<String>,
    /// String in the format \"(H)H:MM AM/PM\" for when the match will be played in the event's local timezone
    #[serde(rename = "time_str", skip_serializing_if = "Option::is_none")]
    pub time_str: Option<String>,
    /// UTC time of the match's scheduled time in ISO 8601 format (YYYY-MM-DDTHH:MM:SS)
    #[serde(rename = "time_utc", skip_serializing_if = "Option::is_none")]
    pub time_utc: Option<String>,
    /// UTC time of when the match actually started in ISO 8601 format (YYYY-MM-DDTHH:MM:SS)
    #[serde(rename = "actual_start_time_utc", skip_serializing_if = "Option::is_none")]
    pub actual_start_time_utc: Option<String>,
    /// UTC time of when the match results were shown to the audience in ISO 8601 format (YYYY-MM-DDTHH:MM:SS)
    #[serde(rename = "post_results_time_utc", skip_serializing_if = "Option::is_none")]
    pub post_results_time_utc: Option<String>,
    /// The name for the match, to be shown on the event page
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Match {
    pub fn new(comp_level: CompLevel, set_number: i32, match_number: i32, alliances: models::MatchAlliances) -> Match {
        Match {
            comp_level,
            set_number,
            match_number,
            alliances: Box::new(alliances),
            score_breakdown: None,
            time_str: None,
            time_utc: None,
            actual_start_time_utc: None,
            post_results_time_utc: None,
            display_name: None,
        }
    }
}
/// Competition level of the match
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompLevel {
    #[serde(rename = "qm")]
    Qm,
    #[serde(rename = "ef")]
    Ef,
    #[serde(rename = "qf")]
    Qf,
    #[serde(rename = "sf")]
    Sf,
    #[serde(rename = "f")]
    F,
}

impl Default for CompLevel {
    fn default() -> CompLevel {
        Self::Qm
    }
}

