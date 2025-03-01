/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.9.11
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Media : The `Media` object contains a reference for most any media associated with a team or event on TBA.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Media {
    /// String type of the media element.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The key used to identify this media on the media site.
    #[serde(rename = "foreign_key")]
    pub foreign_key: String,
    /// If required, a JSON dict of additional media information.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// True if the media is of high quality.
    #[serde(rename = "preferred", skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
    /// List of teams that this media belongs to. Most likely length 1.
    #[serde(rename = "team_keys")]
    pub team_keys: Vec<String>,
    /// Direct URL to the media.
    #[serde(rename = "direct_url", skip_serializing_if = "Option::is_none")]
    pub direct_url: Option<String>,
    /// The URL that leads to the full web page for the media, if one exists.
    #[serde(rename = "view_url", skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}

impl Media {
    /// The `Media` object contains a reference for most any media associated with a team or event on TBA.
    pub fn new(r#type: Type, foreign_key: String, team_keys: Vec<String>) -> Media {
        Media {
            r#type,
            foreign_key,
            details: None,
            preferred: None,
            team_keys,
            direct_url: None,
            view_url: None,
        }
    }
}
/// String type of the media element.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "cdphotothread")]
    Cdphotothread,
    #[serde(rename = "imgur")]
    Imgur,
    #[serde(rename = "facebook-profile")]
    FacebookProfile,
    #[serde(rename = "youtube-channel")]
    YoutubeChannel,
    #[serde(rename = "twitter-profile")]
    TwitterProfile,
    #[serde(rename = "github-profile")]
    GithubProfile,
    #[serde(rename = "instagram-profile")]
    InstagramProfile,
    #[serde(rename = "periscope-profile")]
    PeriscopeProfile,
    #[serde(rename = "gitlab-profile")]
    GitlabProfile,
    #[serde(rename = "grabcad")]
    Grabcad,
    #[serde(rename = "instagram-image")]
    InstagramImage,
    #[serde(rename = "external-link")]
    ExternalLink,
    #[serde(rename = "avatar")]
    Avatar,
}

impl Default for Type {
    fn default() -> Type {
        Self::Youtube
    }
}

