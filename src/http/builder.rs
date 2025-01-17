use std::sync::Arc;

use service_sdk::HttpServerBuilder;
use crate::app::AppContext;

use super::controllers::personal_data_controller;

pub fn build_controllers(
    app: &Arc<AppContext>, 
    http_server_builder: &mut HttpServerBuilder){

    http_server_builder.register_get_action(
        personal_data_controller::GetAction::new(app.clone())
    );

    http_server_builder.register_post_action(
        personal_data_controller::PostAction::new(app.clone())
    );
}
