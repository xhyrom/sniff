mod gpapi_manager;
mod serializable_types;

use gpapi_manager::{create_manager, SharedGpapiManager};
use serde::Serialize;
use serializable_types::SerializableDetailsResponse;
use worker::*;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

struct AppState {
    gpapi_manager: SharedGpapiManager,
}

async fn handle_details_request(
    _: Request,
    state: &AppState,
    package_name: &str,
) -> Result<Response> {
    match state.gpapi_manager.get_details(package_name, None).await {
        Ok(details_opt) => {
            if let Some(details) = details_opt {
                let response = ApiResponse {
                    success: true,
                    data: Some(SerializableDetailsResponse(details)),
                    error: None,
                };

                Response::from_json(&response)
            } else {
                let response = ApiResponse::<SerializableDetailsResponse> {
                    success: false,
                    data: None,
                    error: Some(format!("App '{}' not found", package_name)),
                };

                Ok(Response::from_json(&response)?.with_status(404))
            }
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

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let device_name = env.var("DEVICE_NAME")?.to_string();
    let email = env.var("EMAIL")?.to_string();
    let aas_token = env.var("AAS_TOKEN")?.to_string();

    let gpapi_manager = create_manager(&device_name, &email, &aas_token).await;
    let state = AppState { gpapi_manager };

    let router = Router::with_data(state);

    router
        .get_async("/v1/details/:package_name", |req, ctx| async move {
            let package_name = ctx.param("package_name").unwrap();

            handle_details_request(req, &ctx.data, package_name).await
        })
        .run(req, env)
        .await
}
