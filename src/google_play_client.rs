use googleplay_protobuf::DetailsResponse;
use gpapi::{DownloadInfo, Gpapi};
use std::collections::HashSet;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BETA_PACKAGES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("com.discord");
        set
    };
    pub static ref ALPHA_PACKAGES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("com.discord");
        set
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Stable,
    Beta,
    Alpha,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Stable => write!(f, "stable"),
            Channel::Beta => write!(f, "beta"),
            Channel::Alpha => write!(f, "alpha"),
        }
    }
}

impl Channel {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "alpha" => Ok(Channel::Alpha),
            _ => Err(format!("Invalid Channel: {}", s)),
        }
    }

    pub fn is_available_for_package(self, package_name: &str) -> bool {
        match self {
            Channel::Stable => true,
            Channel::Beta => BETA_PACKAGES.contains(package_name),
            Channel::Alpha => ALPHA_PACKAGES.contains(package_name),
        }
    }
}

pub struct GooglePlayClient {
    client: Gpapi,
    channel: Channel,
}

impl GooglePlayClient {
    pub fn new(device_name: &str, email: &str, aas_token: &str, channel: Channel) -> Self {
        let mut client = Gpapi::new(device_name, email);
        client.set_aas_token(aas_token);

        Self { client, channel }
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        self.client
            .login()
            .await
            .map_err(|e| format!("Login error for {} channel: {:?}", self.channel, e))
    }

    pub async fn get_details(&self, package_name: &str) -> Result<Option<DetailsResponse>, String> {
        self.client
            .details(package_name)
            .await
            .map_err(|e| format!("API error for {} channel: {:?}", self.channel, e))
    }

    pub async fn get_download_info(
        &self,
        package_name: &str,
        version_code: Option<i32>,
    ) -> Result<DownloadInfo, String> {
        self.client
            .get_download_info(package_name, version_code)
            .await
            .map_err(|e| format!("API error for {} channel: {:?}", self.channel, e))
    }
}
