service_sdk::macros::use_my_http_server!();

use rest_api_wl_shared::{validate_name_optional, validate_email_optional};
use serde::Serialize;

#[derive(MyHttpInput)]
pub struct GetPersonalDataHttpInputData {
    #[http_path(name="id", description: "Personal Data Id")]
    pub id: String,
}

#[derive(Serialize, MyHttpObjectStructure, Default)]
pub struct PersonalDataHttpModel {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "email")]
    pub email: Option<String>,

    #[serde(rename = "firstName")]
    pub first_name: Option<String>,

    #[serde(rename = "lastName", )]
    pub last_name: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "country")]
    pub country: Option<String>,
}

#[derive(Debug, MyHttpInput)]
pub struct PersonalDataUpdateHttpInputData {
    #[http_body(name: "id", description:"Id")]
    pub id: String,

    #[http_body(name: "firstName", description:"First Name", validator: "validate_name_optional")]
    pub first_name: Option<String>,

    #[http_body(name: "lastName", description:"Last Name", validator: "validate_name_optional")]
    pub last_name: Option<String>,

    #[http_body(description = "User Email", validator = "validate_email_optional")]
    pub email: Option<String>,
    #[http_body(name: "country", description: "Country")]
    pub country: Option<String>,
}
