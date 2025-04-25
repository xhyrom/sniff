use googleplay_protobuf::DetailsResponse;
use gpapi::Gpapi;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GpapiManager {
    default_client: Gpapi,
    special_clients: HashMap<String, HashMap<String, Gpapi>>,
}

impl GpapiManager {
    pub fn new(device_name: &str, email: &str, aas_token: &str) -> Self {
        let mut default_client = Gpapi::new(device_name, email);
        default_client.set_aas_token(aas_token);

        Self {
            default_client,
            special_clients: HashMap::new(),
        }
    }

    pub fn add_special_client(&mut self, package_name: &str, track: &str, client: Gpapi) {
        self.special_clients
            .entry(package_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(track.to_string(), client);
    }

    pub async fn get_details(
        &self,
        package_name: &str,
        track: Option<&str>,
    ) -> Result<Option<DetailsResponse>, String> {
        if let Some(track) = track {
            if let Some(clients) = self.special_clients.get(package_name) {
                if let Some(client) = clients.get(track) {
                    return client
                        .details(package_name)
                        .await
                        .map_err(|e| format!("API error: {:?}", e));
                }
            }

            return Err(format!(
                "Track '{}' not available for '{}'",
                track, package_name
            ));
        }

        self.default_client
            .details(package_name)
            .await
            .map_err(|e| format!("API error: {:?}", e))
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        self.default_client
            .login()
            .await
            .map_err(|e| format!("Login error: {:?}", e))?;

        Ok(())
    }
}

pub type SharedGpapiManager = Arc<GpapiManager>;

pub async fn create_manager(device_name: &str, email: &str, aas_token: &str) -> SharedGpapiManager {
    let mut manager = GpapiManager::new(device_name, email, aas_token);
    if let Err(e) = manager.initialize().await {
        panic!("Failed to initialize GPAPI manager: {}", e);
    }
    Arc::new(manager)
}
