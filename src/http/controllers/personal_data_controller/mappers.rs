use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::postgres::dto::PersonalDataDto;
use super::contracts::{PersonalDataHttpModel, PersonalDataUpdateHttpInputData};


impl From<PersonalDataDto> for PersonalDataHttpModel {
    fn from(dto: PersonalDataDto) -> Self {
        PersonalDataHttpModel {
            id: dto.id,
            email: dto.email,
            first_name: dto.first_name,
            last_name: dto.last_name,
            country: dto.country,
            created_at: dto.created_at.to_rfc3339(), 
        }
    }
}

impl From<PersonalDataUpdateHttpInputData> for PersonalDataDto {
    fn from(http_model: PersonalDataUpdateHttpInputData) -> Self {
        PersonalDataDto {
            id: http_model.id,
            email: http_model.email,
            first_name: http_model.first_name,
            last_name: http_model.last_name,
            country: http_model.country,
            created_at: DateTimeAsMicroseconds::now(), 
        }
    }
}