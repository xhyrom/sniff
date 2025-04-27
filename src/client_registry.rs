use gpapi::DownloadInfo;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use worker::{console_log, Env};

use crate::google_play_client::{Channel, GooglePlayClient};

pub struct ClientRegistry {
    clients: HashMap<Channel, GooglePlayClient>,
    initialized: HashMap<Channel, bool>,
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

    pub async fn get_client(&mut self, channel: Channel) -> Result<&GooglePlayClient, String> {
        if !self.clients.contains_key(&channel) {
            let device_name = self.env.var("DEVICE_NAME").unwrap().to_string();

            let (email, aas_token) = match channel {
                Channel::Stable => (
                    self.env.var("STABLE_EMAIL").unwrap().to_string(),
                    self.env.var("STABLE_AAS_TOKEN").unwrap().to_string(),
                ),
                Channel::Beta => (
                    self.env.var("BETA_EMAIL").unwrap().to_string(),
                    self.env.var("BETA_AAS_TOKEN").unwrap().to_string(),
                ),
                Channel::Alpha => (
                    self.env.var("ALPHA_EMAIL").unwrap().to_string(),
                    self.env.var("ALPHA_AAS_TOKEN").unwrap().to_string(),
                ),
            };

            let client = GooglePlayClient::new(&device_name, &email, &aas_token, channel);
            self.clients.insert(channel, client);
            self.initialized.insert(channel, false);
        }

        if !self.initialized.get(&channel).unwrap_or(&false) {
            let client = self.clients.get_mut(&channel).unwrap();
            client.initialize().await?;
            self.initialized.insert(channel, true);
        }

        Ok(self.clients.get(&channel).unwrap())
    }

    pub async fn get_details_with_fallback(
        &mut self,
        package_name: &str,
        channel: Channel,
    ) -> Result<Option<(Channel, googleplay_protobuf::DetailsResponse)>, String> {
        if !channel.is_available_for_package(package_name) {
            return Err(format!(
                "Channel '{}' is not available for package '{}'",
                channel, package_name
            ));
        }

        let client = self.get_client(channel).await?;
        match client.get_details(package_name).await {
            Ok(Some(response)) => return Ok(Some((channel, response))),
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        }
    }

    pub async fn get_details_multi(
        &mut self,
        package_name: &str,
    ) -> Result<HashMap<Channel, googleplay_protobuf::DetailsResponse>, String> {
        let mut results = HashMap::new();

        match self
            .get_details_with_fallback(package_name, Channel::Stable)
            .await
        {
            Ok(Some((_, response))) => {
                results.insert(Channel::Stable, response);
            }
            Ok(None) => {
                return Err(format!("App '{}' not found", package_name));
            }
            Err(e) => {
                console_log!("Error fetching {} for stable channel: {}", package_name, e);
                return Err(e);
            }
        }

        if Channel::Beta.is_available_for_package(package_name) {
            match self
                .get_client(Channel::Beta)
                .await?
                .get_details(package_name)
                .await
            {
                Ok(Some(response)) => {
                    results.insert(Channel::Beta, response);
                }
                Err(e) => {
                    console_log!("Error fetching {} for beta channel: {}", package_name, e);
                }
                _ => {}
            }
        }

        if Channel::Alpha.is_available_for_package(package_name) {
            match self
                .get_client(Channel::Alpha)
                .await?
                .get_details(package_name)
                .await
            {
                Ok(Some(response)) => {
                    results.insert(Channel::Alpha, response);
                }
                Err(e) => {
                    console_log!("Error fetching {} for alpha channel: {}", package_name, e);
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub async fn get_download_info(
        &mut self,
        package_name: &str,
        channel: Channel,
        version_code: Option<i32>,
    ) -> Result<Option<(Channel, DownloadInfo)>, String> {
        if !channel.is_available_for_package(package_name) {
            return Err(format!(
                "Channel '{}' is not available for package '{}'",
                channel, package_name
            ));
        }

        let client = self.get_client(channel).await?;
        match client.get_download_info(package_name, version_code).await {
            Ok(download_info) => Ok(Some((channel, download_info))),
            Err(e) => Err(e),
        }
    }
}

pub type SharedClientRegistry = Arc<Mutex<ClientRegistry>>;

pub async fn create_registry(env: Env) -> SharedClientRegistry {
    let registry = ClientRegistry::new(env);
    Arc::new(Mutex::new(registry))
}
