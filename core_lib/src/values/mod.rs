use crate::error::ParsingError;

pub mod rosaries;

pub enum ValueType {
    Int(i64),
    String(String),
    Boolean(bool),
}

pub trait SaveValue {
    fn get_title() -> String
    where
        Self: Sized;
    fn get_description() -> String
    where
        Self: Sized;
    fn get_value(&self) -> ValueType;

    fn update_json_with_value(
        &mut self,
        json: &mut String,
        value: ValueType,
    ) -> Result<(), ParsingError>;
}

pub fn initialize_all_save_values(json: &str) -> Vec<Box<dyn SaveValue>> {
    todo!()
}
