mod client_registry;
mod google_play_client;
mod serializable_types;

use client_registry::{create_registry, SharedClientRegistry};
use google_play_client::Channel;
use serde::Serialize;
use serializable_types::SerializableDetailsResponse;
use std::collections::HashMap;
use worker::*;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Serialize)]
struct MultiChannelApiResponse<T> {
    success: bool,
    data: Option<HashMap<String, T>>,
    error: Option<String>,
}

struct AppState {
    client_registry: SharedClientRegistry,
}

async fn handle_details_request(
    _: Request,
    state: &AppState,
    package_name: &str,
    channel: Channel,
) -> Result<Response> {
    let result = state
        .client_registry
        .lock()
        .expect("Failed to lock client registry")
        .get_details_with_fallback(package_name, channel)
        .await;

    match result {
        Ok(Some((_, details))) => {
            let response = ApiResponse {
                success: true,
                data: Some(SerializableDetailsResponse(details)),
                error: None,
            };

            Ok(Response::from_json(&response)?)
        }
        Ok(None) => {
            let response = ApiResponse::<SerializableDetailsResponse> {
                success: false,
                data: None,
                error: Some(format!("App '{}' not found", package_name)),
            };

            Ok(Response::from_json(&response)?.with_status(404))
        }
        Err(e) => {
            let response = ApiResponse::<SerializableDetailsResponse> {
                success: false,
                data: None,
                error: Some(e),
            };

            Ok(Response::from_json(&response)?.with_status(500))
        }
    }
}

async fn handle_details_multi_request(
    _: Request,
    state: &AppState,
    package_name: &str,
) -> Result<Response> {
    match state
        .client_registry
        .lock()
        .expect("Failed to lock client registry")
        .get_details_multi(package_name)
        .await
    {
        Ok(details_map) => {
            let serialized_map: HashMap<String, SerializableDetailsResponse> = details_map
                .into_iter()
                .map(|(channel, details)| {
                    (channel.to_string(), SerializableDetailsResponse(details))
                })
                .collect();

            let available_channels = serialized_map.keys().cloned().collect::<Vec<_>>().join(",");

            let response = MultiChannelApiResponse {
                success: true,
                data: Some(serialized_map),
                error: None,
            };

            let mut headers = Headers::new();

            headers.set("Content-Type", "application/json")?;
            headers.set("X-Available-Channels", available_channels.as_str())?;

            Ok(Response::from_json(&response)?.with_headers(headers))
        }
        Err(e) => {
            let response = MultiChannelApiResponse::<SerializableDetailsResponse> {
                success: false,
                data: None,
                error: Some(e),
            };

            Ok(Response::from_json(&response)?.with_status(500))
        }
    }
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let client_registry = create_registry(env.clone()).await;
    let state = AppState { client_registry };

    let router = Router::with_data(state);

    router
        .get("/", |_req, _ctx| {
            let url = Url::parse("https://xhyrom.dev/docs/sniff")?;

            Response::redirect(url)
        })
        .get_async("/v1/details/:package_name", |req, ctx| async move {
            let package_name = ctx.param("package_name").unwrap();
            handle_details_multi_request(req, &ctx.data, package_name).await
        })
        .get_async(
            "/v1/details/:package_name/:channel",
            |req, ctx| async move {
                let package_name = ctx.param("package_name").unwrap();
                let channel = ctx.param("channel").unwrap();

                match Channel::from_str(channel) {
                    Ok(track) => handle_details_request(req, &ctx.data, package_name, track).await,
                    Err(e) => {
                        let response = ApiResponse::<()> {
                            success: false,
                            data: None,
                            error: Some(e),
                        };

                        Ok(Response::from_json(&response)?.with_status(400))
                    }
                }
            },
        )
        .run(req, env)
        .await
}
