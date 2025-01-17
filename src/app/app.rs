use std::sync::Arc;

service_sdk::macros::use_my_http_server!();

use crate::postgres::PersonalDataRepo;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub settings_reader: Arc<crate::settings::SettingsReader>,
    pub personal_data_repo: PersonalDataRepo,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<crate::settings::SettingsReader>,
    ) -> Self {
        
        let personal_data_repo = PersonalDataRepo::new(
            settings_reader.clone(),
        )
        .await;

        Self {
            personal_data_repo,
            settings_reader: settings_reader.clone(),
        }
    }
}