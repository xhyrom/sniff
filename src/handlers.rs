#![allow(clippy::await_holding_lock)]

use crate::client_registry::SharedClientRegistry;
use crate::google_play_client::Channel;
use crate::openapi_schema::{
    ApiResponse, DownloadInfo, MultiChannelApiResponse, SerializableDetailsResponse,
};
use crate::serializable_types::SerializableDetailsResponse as ActualSerializableDetailsResponse;
use std::collections::HashMap;
use worker::*;

#[utoipa::path(
    get,
    path = "/v1/details/{package_name}",
    params(
        ("package_name" = String, Path, description = "Android package name (e.g., com.discord)")
    ),
    responses(
        (status = 200, description = "App details retrieved successfully",
         body = MultiChannelApiResponse<SerializableDetailsResponse>,
         headers(
             ("X-Available-Channels" = String, description = "Comma-separated list of available channels")
         )
        ),
        (status = 404, description = "App not found", body = MultiChannelApiResponse<SerializableDetailsResponse>),
        (status = 500, description = "Internal server error", body = MultiChannelApiResponse<SerializableDetailsResponse>)
    ),
    tag = "App Details"
)]
pub async fn get_details_multi(
    package_name: String,
    client_registry: SharedClientRegistry,
) -> Result<Response> {
    #[allow(clippy::await_holding_lock)]
    match client_registry
        .lock()
        .expect("Failed to lock client registry")
        .get_details_multi(&package_name)
        .await
    {
        Ok(details_map) => {
            let serialized_map: HashMap<String, ActualSerializableDetailsResponse> = details_map
                .into_iter()
                .map(|(channel, details)| {
                    (
                        channel.to_string(),
                        ActualSerializableDetailsResponse(details),
                    )
                })
                .collect();

            let available_channels = serialized_map.keys().cloned().collect::<Vec<_>>().join(",");

            let response = MultiChannelApiResponse {
                success: true,
                data: Some(serialized_map),
                error: None,
            };

            let headers = Headers::new();
            headers.set("Content-Type", "application/json")?;
            headers.set("X-Available-Channels", &available_channels)?;

            Ok(Response::from_json(&response)?.with_headers(headers))
        }
        Err(e) => {
            let response = MultiChannelApiResponse::<ActualSerializableDetailsResponse> {
                success: false,
                data: None,
                error: Some(e),
            };

            Ok(Response::from_json(&response)?.with_status(500))
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/details/{package_name}/{channel}",
    params(
        ("package_name" = String, Path, description = "Android package name (e.g., com.discord)"),
        ("channel" = String, Path, description = "Release channel", example = "stable")
    ),
    responses(
        (status = 200, description = "App details retrieved successfully", body = ApiResponse<SerializableDetailsResponse>),
        (status = 400, description = "Invalid channel", body = ApiResponse<String>),
        (status = 404, description = "App not found", body = ApiResponse<SerializableDetailsResponse>),
        (status = 500, description = "Internal server error", body = ApiResponse<SerializableDetailsResponse>)
    ),
    tag = "App Details"
)]
pub async fn get_details_single(
    package_name: String,
    channel: String,
    client_registry: SharedClientRegistry,
) -> Result<Response> {
    let channel = match Channel::from_str(&channel) {
        Ok(ch) => ch,
        Err(e) => {
            let response = ApiResponse::<()> {
                success: false,
                data: None,
                error: Some(e),
            };
            return Ok(Response::from_json(&response)?.with_status(400));
        }
    };

    let result = client_registry
        .lock()
        .expect("Failed to lock client registry")
        .get_details_with_fallback(&package_name, channel)
        .await;

    match result {
        Ok(Some((_, details))) => {
            let response = ApiResponse {
                success: true,
                data: Some(ActualSerializableDetailsResponse(details)),
                error: None,
            };
            Ok(Response::from_json(&response)?)
        }
        Ok(None) => {
            let response = ApiResponse::<ActualSerializableDetailsResponse> {
                success: false,
                data: None,
                error: Some(format!("App '{}' not found", package_name)),
            };
            Ok(Response::from_json(&response)?.with_status(404))
        }
        Err(e) => {
            let response = ApiResponse::<ActualSerializableDetailsResponse> {
                success: false,
                data: None,
                error: Some(e),
            };
            Ok(Response::from_json(&response)?.with_status(500))
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/download/{package_name}/{channel}/{version_code}",
    params(
        ("package_name" = String, Path, description = "Android package name"),
        ("channel" = String, Path, description = "Release channel"),
        ("version_code" = i32, Path, description = "Android version code")
    ),
    responses(
        (status = 200, description = "Download info retrieved successfully", body = ApiResponse<DownloadInfo>),
        (status = 400, description = "Invalid parameters", body = ApiResponse<String>),
        (status = 404, description = "App or version not found", body = ApiResponse<DownloadInfo>),
        (status = 500, description = "Internal server error", body = ApiResponse<DownloadInfo>)
    ),
    tag = "Downloads"
)]
pub async fn get_download_info(
    package_name: String,
    channel: String,
    version_code: i64,
    client_registry: SharedClientRegistry,
) -> Result<Response> {
    let channel = match Channel::from_str(&channel) {
        Ok(ch) => ch,
        Err(e) => {
            let response = ApiResponse::<()> {
                success: false,
                data: None,
                error: Some(e),
            };
            return Ok(Response::from_json(&response)?.with_status(400));
        }
    };

    let result = client_registry
        .lock()
        .expect("Failed to lock client registry")
        .get_download_info(&package_name, channel, Some(version_code))
        .await;

    match result {
        Ok(Some((_, download_info))) => {
            let openapi_download_info = DownloadInfo::from(download_info);
            let response = ApiResponse {
                success: true,
                data: Some(openapi_download_info),
                error: None,
            };
            Ok(Response::from_json(&response)?)
        }
        Ok(None) => {
            let response = ApiResponse::<DownloadInfo> {
                success: false,
                data: None,
                error: Some(format!("App '{}' not found", package_name)),
            };
            Ok(Response::from_json(&response)?.with_status(404))
        }
        Err(e) => {
            let response = ApiResponse::<DownloadInfo> {
                success: false,
                data: None,
                error: Some(e),
            };
            Ok(Response::from_json(&response)?.with_status(500))
        }
    }
}
