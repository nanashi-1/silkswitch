use std::{collections::HashMap, fmt::Debug};

use crate::{
    error::ParsingError,
    values::{generic_value::GenericValue, generic_values::GENERIC_VALUE_DESC},
};

pub mod generic_value;
pub mod generic_values;

#[derive(Clone, Debug)]
pub enum ValueType {
    Int(i64),
    String(String),
    Boolean(bool),
}

pub trait SaveValue: Debug {
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_pointer(&self) -> &str;
    fn get_value(&self) -> &ValueType;

    /// Smaller numbers gets the highest priority
    fn get_priority(&self) -> u64;

    fn update_json_with_value(
        &mut self,
        json: &mut String,
        value: ValueType,
    ) -> Result<(), ParsingError>;
}

pub fn initialize_all_save_values(
    json: &str,
) -> Result<HashMap<String, Box<dyn SaveValue>>, ParsingError> {
    let mut save_value_map: HashMap<String, Box<dyn SaveValue>> = HashMap::new();

    // generate generic value parsers
    GENERIC_VALUE_DESC.iter().try_for_each(|desc| {
        let generic_value = GenericValue::new(
            json,
            desc.pointer,
            desc.generic_type,
            desc.title,
            desc.description,
            desc.priority,
        )?;

        save_value_map.insert(desc.pointer.into(), Box::new(generic_value));

        Ok::<(), ParsingError>(())
    })?;

    Ok(save_value_map)
}
