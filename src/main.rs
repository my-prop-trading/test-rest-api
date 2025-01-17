use std::sync::Arc;

mod app;
mod grpc;
mod http;
mod postgres;
mod settings;

pub mod personal_data_grpc {
    tonic::include_proto!("personal_data");
}

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".settings/settings.yaml").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;
    let app = Arc::new(app::AppContext::new(settings_reader).await);

    service_context.configure_http_server(|builder| {
        rest_api_wl_shared::configure_rest_api_server(builder);
        http::builder::build_controllers(&app, builder);
    });

    service_context.start_application().await;
}
