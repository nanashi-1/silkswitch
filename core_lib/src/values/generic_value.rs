use serde_json::Value;

use crate::{
    error::ParsingError,
    values::{SaveValue, ValueType},
};

#[derive(Debug)]
pub struct GenericValue {
    value: ValueType,
    pointer: String,
    title: String,
    description: String,
    priority: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum GenericType {
    Int,
    String,
    Boolean,
}

impl GenericValue {
    pub fn new(
        json: &str,
        pointer: &str,
        generic_type: GenericType,
        title: &str,
        description: &str,
        priority: u64,
    ) -> Result<Self, ParsingError> {
        let json_serde: serde_json::Value = serde_json::from_str(json)?;
        let value = json_serde
            .pointer(pointer)
            .ok_or(ParsingError::Pointer(pointer.into()))?;

        match generic_type {
            GenericType::Int => Ok(Self {
                value: ValueType::Int(value.as_i64().ok_or(ParsingError::ValueType)?),
                pointer: pointer.into(),
                title: title.into(),
                description: description.into(),
                priority,
            }),
            GenericType::String => Ok(Self {
                value: ValueType::String(value.to_string()),
                pointer: pointer.into(),
                title: title.into(),
                description: description.into(),
                priority,
            }),
            GenericType::Boolean => Ok(Self {
                value: ValueType::Boolean(value.as_bool().ok_or(ParsingError::ValueType)?),
                pointer: pointer.into(),
                title: title.into(),
                description: description.into(),
                priority,
            }),
        }
    }
}

impl SaveValue for GenericValue {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_pointer(&self) -> &str {
        &self.pointer
    }

    fn get_value(&self) -> &ValueType {
        &self.value
    }

    fn get_priority(&self) -> u64 {
        self.priority
    }

    fn update_json_with_value(
        &mut self,
        json: &mut String,
        value: ValueType,
    ) -> Result<(), ParsingError> {
        let mut json_serde: serde_json::Value = serde_json::from_str(json)?;
        let json_value = json_serde
            .pointer_mut(&self.pointer)
            .ok_or(ParsingError::Pointer(self.pointer.clone()))?;

        self.value = value.clone();

        match value {
            ValueType::Int(i) => {
                *json_value = Value::Number(i.into());
            }
            ValueType::String(s) => {
                *json_value = Value::String(s);
            }
            ValueType::Boolean(b) => {
                *json_value = Value::Bool(b);
            }
        }

        *json = serde_json::to_string(&json_serde)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let json_str = r#"{"playerData":{"geo":2000,"health":5}}"#;

        let rosaries =
            GenericValue::new(json_str, "/playerData/geo", GenericType::Int, "", "", 0).unwrap();
        let health =
            GenericValue::new(json_str, "/playerData/health", GenericType::Int, "", "", 0).unwrap();

        if let ValueType::Int(rosaries) = rosaries.get_value()
            && let ValueType::Int(health) = health.get_value()
        {
            assert_eq!(2000, *rosaries);
            assert_eq!(5, *health);
        }
    }

    #[test]
    fn test_edit() {
        let mut json_str = r#"{"playerData":{"geo":2000,"health":5}}"#.to_string();
        let expected_json_str = r#"{"playerData":{"geo":1000,"health":3}}"#;

        let mut rosaries =
            GenericValue::new(&json_str, "/playerData/geo", GenericType::Int, "", "", 0).unwrap();
        let mut health =
            GenericValue::new(&json_str, "/playerData/health", GenericType::Int, "", "", 0)
                .unwrap();

        rosaries
            .update_json_with_value(&mut json_str, ValueType::Int(1000))
            .unwrap();
        health
            .update_json_with_value(&mut json_str, ValueType::Int(3))
            .unwrap();

        assert_eq!(expected_json_str, json_str);
    }
}
