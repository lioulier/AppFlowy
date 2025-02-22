use crate::impl_from_and_to_type_option;
use crate::services::row::CellDataSerde;

use flowy_derive::ProtoBuf;
use flowy_error::FlowyError;
use flowy_grid_data_model::entities::{FieldMeta, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, ProtoBuf)]
pub struct RichTextDescription {
    #[pb(index = 1)]
    pub format: String,
}
impl_from_and_to_type_option!(RichTextDescription, FieldType::RichText);

impl CellDataSerde for RichTextDescription {
    fn deserialize_cell_data(&self, data: String) -> String {
        data
    }

    fn serialize_cell_data(&self, data: &str) -> Result<String, FlowyError> {
        let data = data.to_owned();
        if data.len() > 10000 {
            Err(FlowyError::text_too_long().context("The len of the text should not be more than 10000"))
        } else {
            Ok(data)
        }
    }
}
