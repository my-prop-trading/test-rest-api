use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::{ApiHttpResultWithData, ApiResultStatus, GetClientId};

use crate::app::AppContext;

use super::contracts::*;

service_sdk::macros::use_my_http_server!();
#[http_route(
    method: "GET",
    route: "/api/personal_data/v1/{id}",
    summary: "Gets personal data of trader",
    description: "Returns personal data of trader",
    controller: "personal_data",
    input_data:"GetPersonalDataHttpInputData",
    result:[
        {status_code: 200, description: "Ok response", model: "ApiHttpResultWithData<PersonalDataHttpModel>"},
    ]
)] 
pub struct GetAction {
    app: Arc<AppContext>,
}

impl GetAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetAction,
    input_data: GetPersonalDataHttpInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let my_telemetry = ctx.telemetry_context.clone();

    match action
        .app
        .personal_data_repo
        .try_get(&input_data.id, &my_telemetry)
        .await
    {
        Ok(result) => match result {
            Some(dto) => {
                let result = ApiHttpResultWithData::<PersonalDataHttpModel> {
                    result: ApiResultStatus::Ok,
                    data: Some(dto.into()),
                };
                return HttpOutput::as_json(result).into_ok_result(true).into();
            }
            None => {
                let msg = format!("User not found");
                return HttpOutput::as_text(msg).into_fail_result(404, true);
            }
        },
        Err(err) => {
            let msg = format!("Failed to get data. Error: {:?}", err);
            return HttpOutput::as_text(msg).into_fail_result(500, true);
        }
    }
}
