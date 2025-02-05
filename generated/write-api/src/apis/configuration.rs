/*
 * The Blue Alliance Trusted APIv1
 *
 * # Overview Import FIRST Robotics Competition Data to The Blue Alliance. The API is accessed through POST requests made to the endpoints described below. To simplify the removal of incorrect data, most endpoints must be called with the full dataset. For example, there is no endpoint to add a single award-- only to update all awards. It must be called with all awards, and the ones that are no longer present will be deleted. **The exception to this is the matches endpoint**, where there are separate endpoints to update and delete matches. Also, the **match videos endpoint** only allows the addition (and not the removal) of videos. # Authentication The X-TBA-Auth-Id and X-TBA-Auth-Sig must be included as request headers in all requests.  *auth_id*: ``  *auth_secret*: ``  *X-TBA-Auth-Id*: `auth_id`  *X-TBA-Auth-Sig*: `md5_hexdigest()`  For example, an X-TBA-Auth-Sig may look like `md5_hexdigest(ExqeZK3Gbo9v95YnqmsiADzESo9HNgyhIOYSMyRpqJqYv13EazNRaDIPPJuOXrQp/api/trusted/v1/event/2014casj/matches/delete[\"qm1\"])`  Which, after hashing, becomes: `ca5c3e5c1b0e7132e4af13f805eca0be`
 *
 * The version of the OpenAPI document: 3.1
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}


impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://www.thebluealliance.com/api/trusted/v1".to_owned(),
            user_agent: Some("OpenAPI-Generator/3.1/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
