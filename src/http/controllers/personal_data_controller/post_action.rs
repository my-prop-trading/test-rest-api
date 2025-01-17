use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::{ApiHttpResultWithData, ApiResultStatus};
use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{app::AppContext, postgres::dto::PersonalDataDto};

use super::contracts::*;

service_sdk::macros::use_my_http_server!();
#[http_route(
    method: "POST",
    route: "/api/personal_data/v1",
    summary: "Update personal data",
    description: "Update personal data",
    controller: "personal_data",
    input_data:"PersonalDataUpdateHttpInputData",
    result:[
        {status_code: 200, description: "Ok response", model: "ApiHttpResultWithData<PersonalDataHttpModel>"},
    ]
)]
pub struct PostAction {
    app: Arc<AppContext>,
}

impl PostAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &PostAction,
    input_data: PersonalDataUpdateHttpInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let telemetry_context = ctx.telemetry_context.clone();

    match get_existing_dto(&action.app, &input_data.id, &telemetry_context).await {
        Ok(Some(mut existing_dto)) => {
            // Dto exists, update the necessary fields
            existing_dto.first_name = input_data.first_name.clone();
            existing_dto.last_name = input_data.last_name.clone();
            existing_dto.email = input_data.email.clone();
    
            match action.app
                .personal_data_repo
                .insert_or_update(&existing_dto, &telemetry_context)
                .await
            {
                Ok(_) => {
                    let result = ApiHttpResultWithData::<PersonalDataHttpModel> {
                        result: ApiResultStatus::Ok,
                        data: Some(existing_dto.into()),
                    };
                    return HttpOutput::as_json(result).into_ok_result(true).into();
                },
                Err(err) => {
                    let msg = format!("Failed to update. Error: {:?}", err);
                    return HttpOutput::as_text(msg).into_fail_result(500, true);
                },
            }
        }
        Ok(None) => {
            // Dto does not exist, create a new one
            let new_dto = input_data.into();
    
            match action.app
                .personal_data_repo
                .insert_or_update(&new_dto, &telemetry_context)
                .await
            {
                Ok(_) => {
                    let result = ApiHttpResultWithData::<PersonalDataHttpModel> {
                        result: ApiResultStatus::Ok,
                        data: Some(new_dto.into()),
                    };
                    return HttpOutput::as_json(result).into_ok_result(true).into();
                },
                Err(err) => {
                    let msg = format!("Failed to create. Error: {:?}", err);
                    return HttpOutput::as_text(msg).into_fail_result(500, true);
                },
            }
        }
        Err(err) => {
            let msg = format!("Failed to get data. Error: {:?}", err);
            return HttpOutput::as_text(msg).into_fail_result(500, true);
        }
    }
}

async fn get_existing_dto(
    app: &Arc<AppContext>,
    id: &str,
    telemetry_context: &MyTelemetryContext,
) -> Result<Option<PersonalDataDto>, String> {
    match app
        .personal_data_repo
        .try_get(id, &telemetry_context)
        .await
    {
        Ok(result) => match result {
            Some(dto) => {
                return Ok(Some(dto)); 
            }
            None => {
                return Ok(None);
            }
        },
        Err(err) => {
            let msg = format!("Failed to get data. Error: {:?}", err);
            return Err(msg);
        }
    }
}

