use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use worker::{console_log, Env};

use crate::google_play_client::{GooglePlayClient, Track};

pub struct ClientRegistry {
    clients: HashMap<Track, GooglePlayClient>,
    initialized: HashMap<Track, bool>,
    env: Env,
}

impl ClientRegistry {
    pub fn new(env: Env) -> Self {
        Self {
            clients: HashMap::new(),
            initialized: HashMap::new(),
            env,
        }
    }

    pub async fn get_client(&mut self, track: Track) -> Result<&GooglePlayClient, String> {
        if !self.clients.contains_key(&track) {
            let device_name = self.env.var("DEVICE_NAME").unwrap().to_string();

            let (email, aas_token) = match track {
                Track::Stable => (
                    self.env.var("STABLE_EMAIL").unwrap().to_string(),
                    self.env.var("STABLE_AAS_TOKEN").unwrap().to_string(),
                ),
                Track::Beta => (
                    self.env.var("BETA_EMAIL").unwrap().to_string(),
                    self.env.var("BETA_AAS_TOKEN").unwrap().to_string(),
                ),
                Track::Alpha => (
                    self.env.var("ALPHA_EMAIL").unwrap().to_string(),
                    self.env.var("ALPHA_AAS_TOKEN").unwrap().to_string(),
                ),
            };

            let client = GooglePlayClient::new(&device_name, &email, &aas_token, track);
            self.clients.insert(track, client);
            self.initialized.insert(track, false);
        }

        if !self.initialized.get(&track).unwrap_or(&false) {
            let client = self.clients.get_mut(&track).unwrap();
            client.initialize().await?;
            self.initialized.insert(track, true);
        }

        Ok(self.clients.get(&track).unwrap())
    }

    pub async fn get_details_with_fallback(
        &mut self,
        package_name: &str,
        track: Track,
    ) -> Result<Option<(Track, googleplay_protobuf::DetailsResponse)>, String> {
        if !track.is_available_for_package(package_name) {
            return Err(format!(
                "Track '{}' is not available for package '{}'",
                track, package_name
            ));
        }

        let client = self.get_client(track).await?;
        match client.get_details(package_name).await {
            Ok(Some(response)) => return Ok(Some((track, response))),
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        }
    }

    pub async fn get_details_multi(
        &mut self,
        package_name: &str,
    ) -> Result<HashMap<Track, googleplay_protobuf::DetailsResponse>, String> {
        let mut results = HashMap::new();

        match self
            .get_details_with_fallback(package_name, Track::Stable)
            .await
        {
            Ok(Some((_, response))) => {
                results.insert(Track::Stable, response);
            }
            Ok(None) => {
                return Err(format!("App '{}' not found", package_name));
            }
            Err(e) => {
                console_log!("Error fetching {} for stable track: {}", package_name, e);
                return Err(e);
            }
        }

        if Track::Beta.is_available_for_package(package_name) {
            match self
                .get_client(Track::Beta)
                .await?
                .get_details(package_name)
                .await
            {
                Ok(Some(response)) => {
                    results.insert(Track::Beta, response);
                }
                Err(e) => {
                    console_log!("Error fetching {} for beta track: {}", package_name, e);
                }
                _ => {}
            }
        }

        if Track::Alpha.is_available_for_package(package_name) {
            match self
                .get_client(Track::Alpha)
                .await?
                .get_details(package_name)
                .await
            {
                Ok(Some(response)) => {
                    results.insert(Track::Alpha, response);
                }
                Err(e) => {
                    console_log!("Error fetching {} for alpha track: {}", package_name, e);
                }
                _ => {}
            }
        }

        Ok(results)
    }
}

pub type SharedClientRegistry = Arc<Mutex<ClientRegistry>>;

pub async fn create_registry(env: Env) -> SharedClientRegistry {
    let registry = ClientRegistry::new(env);
    Arc::new(Mutex::new(registry))
}
