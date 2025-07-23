mod client_registry;
mod google_play_client;
mod handlers;
mod openapi_schema;
mod serializable_types;

use client_registry::create_registry;
use openapi_schema::ApiDoc;
use utoipa::OpenApi;
use worker::*;

struct AppState {
    client_registry: client_registry::SharedClientRegistry,
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
        .get("/openapi.json", |_req, _ctx| {
            let spec = ApiDoc::openapi().to_pretty_json().unwrap();

            let mut headers = Headers::new();
            headers.set("Content-Type", "application/json")?;

            Ok(Response::ok(&spec)?.with_headers(headers))
        })
        .get_async("/v1/details/:package_name", |_req, ctx| async move {
            let package_name = ctx.param("package_name").unwrap().to_string();
            handlers::get_details_multi(package_name, ctx.data.client_registry.clone()).await
        })
        .get_async(
            "/v1/details/:package_name/:channel",
            |_req, ctx| async move {
                let package_name = ctx.param("package_name").unwrap().to_string();
                let channel = ctx.param("channel").unwrap().to_string();
                handlers::get_details_single(
                    package_name,
                    channel,
                    ctx.data.client_registry.clone(),
                )
                .await
            },
        )
        .get_async(
            "/v1/download/:package_name/:channel/:version_code",
            |_req, ctx| async move {
                let package_name = ctx.param("package_name").unwrap().to_string();
                let channel = ctx.param("channel").unwrap().to_string();
                let version_code: i32 = ctx.param("version_code").unwrap().parse().unwrap_or(0);
                handlers::get_download_info(
                    package_name,
                    channel,
                    version_code,
                    ctx.data.client_registry.clone(),
                )
                .await
            },
        )
        .run(req, env)
        .await
}
