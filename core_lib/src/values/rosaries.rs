use serde_json::Value;

use crate::{
    error::ParsingError,
    values::{SaveValue, ValueType},
};

pub struct Rosaries(i64);

impl Rosaries {
    fn new(json: &str) -> Result<Self, ParsingError> {
        let json_serde: serde_json::Value = serde_json::from_str(json)?;
        let rosaries = json_serde
            .pointer("/playerData/geo")
            .ok_or(ParsingError::Pointer("/playerData/geo".into()))?
            .as_i64()
            .ok_or(ParsingError::ValueType)?;

        Ok(Self(rosaries))
    }
}

impl SaveValue for Rosaries {
    fn get_title() -> String
    where
        Self: Sized,
    {
        "Rosaries".into()
    }

    fn get_description() -> String
    where
        Self: Sized,
    {
        "Amount of rosaries Hornet holds.".into()
    }

    fn get_value(&self) -> super::ValueType {
        ValueType::Int(self.0)
    }

    fn update_json_with_value(
        &mut self,
        json: &mut String,
        value: ValueType,
    ) -> Result<(), ParsingError> {
        let mut json_serde: serde_json::Value = serde_json::from_str(json)?;
        let rosaries = json_serde
            .pointer_mut("/playerData/geo")
            .ok_or(ParsingError::Pointer("/playerData/geo".into()))?;

        match value {
            ValueType::Int(i) => {
                self.0 = i;
                *rosaries = Value::Number(i.into());
            }
            _ => return Err(ParsingError::ValueType),
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
        let json = r#"{"playerData":{"geo": 200}}"#;
        let rosaries_value = Rosaries::new(json).unwrap();

        assert_eq!(200, rosaries_value.0);
    }

    #[test]
    fn test_edit() {
        let mut json = r#"{"playerData":{"geo": 200}}"#.to_string();
        let json_target = r#"{"playerData":{"geo": 100}}"#;

        let mut rosaries_value = Rosaries::new(&json).unwrap();

        rosaries_value
            .update_json_with_value(&mut json, ValueType::Int(100))
            .unwrap();

        let actual_value: Value = serde_json::from_str(&json).unwrap();
        let expected_value: Value = serde_json::from_str(json_target).unwrap();

        assert_eq!(expected_value, actual_value);
        assert_eq!(100, rosaries_value.0);
    }
}
