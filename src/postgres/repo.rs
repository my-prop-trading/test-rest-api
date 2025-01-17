service_sdk::macros::use_my_postgres!();
use std::sync::Arc;
use std::time::Duration;
use service_sdk::my_postgres::SqlOperationWithRetries;
use super::dto::*;

pub const PD_TABLE_NAME: &str = "pd";
pub const PD_PK_NAME: &str = "pd_pk";

pub struct PersonalDataRepo {
    repo: SqlOperationWithRetries,
}

impl PersonalDataRepo {
    pub async fn new(settings: Arc<crate::settings::SettingsReader>) -> Self {
        Self {
            repo: MyPostgres::from_settings(
                crate::app::APP_NAME.to_string(),
                settings,
            )
            .with_table_schema_verification::<PersonalDataDto>(
                PD_TABLE_NAME,
                PD_PK_NAME.to_string().into(),
            )
            .build()
            .await
            .with_retries(5, Duration::from_millis(250)),
        }
    }

    pub async fn try_get(
        &self,
        id: &str,
        telemetry_context: &MyTelemetryContext,
    ) -> Result<Option<PersonalDataDto>, MyPostgresError> {
        let where_model = WhereByIdModel { id };
        let result: Option<PersonalDataDto> = self
            .repo
            .query_single_row(
                PD_TABLE_NAME,
                Some(&where_model),
                Some(telemetry_context),
            )
            .await?;

        Ok(result)
    }

    pub async fn insert_or_update(
        &self,
        dto: &PersonalDataDto,
        telemetry_context: &MyTelemetryContext,
    ) -> Result<(), MyPostgresError> {
        
        self.repo
            .insert_or_update_db_entity(
                PD_TABLE_NAME,
                UpdateConflictType::OnPrimaryKeyConstraint(PD_PK_NAME.to_string().into()),
                dto,
                Some(telemetry_context),
            )
            .await?;

        Ok(())
    }

}
