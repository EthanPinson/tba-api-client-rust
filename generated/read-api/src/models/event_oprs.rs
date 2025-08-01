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

/// EventOprs : OPR, DPR, and CCWM for teams at the event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventOprs {
    /// A key-value pair with team key (eg `frc254`) as key and OPR as value.
    #[serde(rename = "oprs", skip_serializing_if = "Option::is_none")]
    pub oprs: Option<std::collections::HashMap<String, f32>>,
    /// A key-value pair with team key (eg `frc254`) as key and DPR as value.
    #[serde(rename = "dprs", skip_serializing_if = "Option::is_none")]
    pub dprs: Option<std::collections::HashMap<String, f32>>,
    /// A key-value pair with team key (eg `frc254`) as key and CCWM as value.
    #[serde(rename = "ccwms", skip_serializing_if = "Option::is_none")]
    pub ccwms: Option<std::collections::HashMap<String, f32>>,
}

impl EventOprs {
    /// OPR, DPR, and CCWM for teams at the event.
    pub fn new() -> EventOprs {
        EventOprs {
            oprs: None,
            dprs: None,
            ccwms: None,
        }
    }
}

