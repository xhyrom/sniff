use googleplay_protobuf::DetailsResponse;
use gpapi::Gpapi;
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
pub enum Track {
    Stable,
    Beta,
    Alpha,
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Track::Stable => write!(f, "stable"),
            Track::Beta => write!(f, "beta"),
            Track::Alpha => write!(f, "alpha"),
        }
    }
}

impl Track {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "stable" => Ok(Track::Stable),
            "beta" => Ok(Track::Beta),
            "alpha" => Ok(Track::Alpha),
            _ => Err(format!("Invalid track: {}", s)),
        }
    }

    pub fn is_available_for_package(self, package_name: &str) -> bool {
        match self {
            Track::Stable => true,
            Track::Beta => BETA_PACKAGES.contains(package_name),
            Track::Alpha => ALPHA_PACKAGES.contains(package_name),
        }
    }
}

pub struct GooglePlayClient {
    client: Gpapi,
    track: Track,
}

impl GooglePlayClient {
    pub fn new(device_name: &str, email: &str, aas_token: &str, track: Track) -> Self {
        let mut client = Gpapi::new(device_name, email);
        client.set_aas_token(aas_token);

        Self { client, track }
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        self.client
            .login()
            .await
            .map_err(|e| format!("Login error for {} track: {:?}", self.track, e))
    }

    pub async fn get_details(&self, package_name: &str) -> Result<Option<DetailsResponse>, String> {
        self.client
            .details(package_name)
            .await
            .map_err(|e| format!("API error for {} track: {:?}", self.track, e))
    }
}
