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
pub struct EventInfo {
    /// Event code used to sync data with FIRST
    #[serde(rename = "first_code", skip_serializing_if = "Option::is_none")]
    pub first_code: Option<String>,
    /// Integer constant representing the playoff format. References constants here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/playoff_type.py
    #[serde(rename = "playoff_type", skip_serializing_if = "Option::is_none")]
    pub playoff_type: Option<i32>,
    /// A list of webcast URLs to set for this event. This will overwrite the existing webcast list
    #[serde(rename = "webcasts", skip_serializing_if = "Option::is_none")]
    pub webcasts: Option<Vec<models::EventInfoWebcastsInner>>,
    /// A mapping of temp key --> remapped key (including B team keys)
    #[serde(rename = "remap_teams", skip_serializing_if = "Option::is_none")]
    pub remap_teams: Option<std::collections::HashMap<String, String>>,
    /// Timezone name for the event
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl EventInfo {
    pub fn new() -> EventInfo {
        EventInfo {
            first_code: None,
            playoff_type: None,
            webcasts: None,
            remap_teams: None,
            timezone: None,
        }
    }
}

