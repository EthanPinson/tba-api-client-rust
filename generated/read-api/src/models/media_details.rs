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

/// MediaDetails : If required, a JSON dict of additional media information.
/// If required, a JSON dict of additional media information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaDetails {
    Object(serde_json::Value),
    MediaDetailsOneOf(Box<models::MediaDetailsOneOf>),
    MediaDetailsOneOf1(Box<models::MediaDetailsOneOf1>),
    MediaDetailsOneOf2(Box<models::MediaDetailsOneOf2>),
    MediaDetailsOneOf3(Box<models::MediaDetailsOneOf3>),
}

impl Default for MediaDetails {
    fn default() -> Self {
        Self::Object(Default::default())
    }
}

