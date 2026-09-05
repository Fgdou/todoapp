use std::env;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct OidcConfiguration {
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
}

pub struct Oidc {
    client_id: String,
    client_secret: String,
    configuration: OidcConfiguration,
    redirect_uri: String,
}

#[derive(Deserialize)]
struct OidcAuthorisationResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct OidcUserInfoResponse {
    sub: String,
}

pub struct RedirectUri(pub String);

impl Oidc {
    pub async fn new(configuration_uri: String, client_id: String, client_secret: String, redirect_uri: String) -> Self {
        rocket::info!("Setting up OIDC for {}", configuration_uri);

        let configuration = Self::get_configuration(&configuration_uri).await;
        
        rocket::info!("OIDC setup done !");

        Self {
            client_id,
            client_secret,
            configuration,
            redirect_uri
        }
    }

    pub async fn new_from_env() -> Option<Self> {
        match env::var("OIDC_ENABLED") {
            Err(_) => return None,
            Ok(value) => if value.to_lowercase() != "true" {
                return None;
            },
        };
        
        Some(Self::new(
            env::var("OIDC_CONFIGURATION_URI").unwrap(),
            env::var("OIDC_CLIENT_ID").unwrap(),
            env::var("OIDC_CLIENT_SECRET").unwrap(),
            env::var("OIDC_REDIRECT_URI").unwrap(),
        ).await)
    }

    pub fn authorize_new(&self) -> RedirectUri {
        let authorization_endpoint = &self.configuration.authorization_endpoint;
        let client_id = &self.client_id;
        let redirect_uri = &self.redirect_uri;

        let full_redirect_uri = format!("{authorization_endpoint}?client_id={client_id}&response_type=code&scope=openid%20profile&redirect_uri={redirect_uri}");
    
        RedirectUri(full_redirect_uri)    
    }

    pub async fn validate_authorization(&self, code: String) -> String {
        rocket::info!("Authorizing user...");

        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let redirect_uri = &self.redirect_uri;

        let token_endpoint = &self.configuration.token_endpoint;

        let content = format!("grant_type=authorization_code&client_id={client_id}&client_secret={client_secret}&code={code}&redirect_uri={redirect_uri}");

        let res: OidcAuthorisationResponse = Client::new()
            .post(token_endpoint)
            .body(content)
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        rocket::info!("Access token acquired...");

        let access_token = res.access_token;

        let res: OidcUserInfoResponse = Client::new()
            .get(&self.configuration.userinfo_endpoint)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {access_token}"))
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        rocket::info!("User {} authorized !", &res.sub);

        res.sub
    }

    async fn get_configuration(url: &str) -> OidcConfiguration {
        Client::new()
            .get(url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}