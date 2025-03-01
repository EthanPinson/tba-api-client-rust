/*
 * The Blue Alliance Trusted APIv1
 *
 * # Overview Import FIRST Robotics Competition Data to The Blue Alliance. The API is accessed through POST requests made to the endpoints described below. To simplify the removal of incorrect data, most endpoints must be called with the full dataset. For example, there is no endpoint to add a single award-- only to update all awards. It must be called with all awards, and the ones that are no longer present will be deleted. **The exception to this is the matches endpoint**, where there are separate endpoints to update and delete matches. Also, the **match videos endpoint** only allows the addition (and not the removal) of videos. # Authentication The X-TBA-Auth-Id and X-TBA-Auth-Sig must be included as request headers in all requests.  *auth_id*: ``  *auth_secret*: ``  *X-TBA-Auth-Id*: `auth_id`  *X-TBA-Auth-Sig*: `md5_hexdigest()`  For example, an X-TBA-Auth-Sig may look like `md5_hexdigest(ExqeZK3Gbo9v95YnqmsiADzESo9HNgyhIOYSMyRpqJqYv13EazNRaDIPPJuOXrQp/api/trusted/v1/event/2014casj/matches/delete[\"qm1\"])`  Which, after hashing, becomes: `ca5c3e5c1b0e7132e4af13f805eca0be`
 *
 * The version of the OpenAPI document: 3.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`update_awards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAwardsError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}


/// Overwrite the event's award listing.
pub async fn update_awards(configuration: &configuration::Configuration, event_key: &str, award: Vec<models::Award>) -> Result<(), Error<UpdateAwardsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_event_key = event_key;
    let p_award = award;

    let uri_str = format!("{}/event/{eventKey}/awards/update", configuration.base_path, eventKey=crate::apis::urlencode(p_event_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-TBA-Auth-Id", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-TBA-Auth-Sig", value);
    };
    req_builder = req_builder.json(&p_award);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAwardsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

