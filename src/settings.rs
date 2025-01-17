use serde_derive::{Deserialize, Serialize};
use service_sdk::my_settings_reader::SettingsModel;
service_sdk::macros::use_settings!();

#[derive(
    SettingsModel,
    SdkSettingsTraits,
    AutoGenerateSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,
    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: Option<String>,
    #[serde(rename = "PostgresConnectionString")]
    pub postgres_conn_string: String,
}
