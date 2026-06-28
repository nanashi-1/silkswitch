use serde_json::Value;

use crate::values::SaveValue;

pub struct Rosaries {
    value: usize,
}

impl SaveValue<usize> for Rosaries {
    fn new(json: &str) -> Self {
        let json: Value = serde_json::from_str(json).unwrap();
        let rosaries = json["playerData"]["geo"].as_u64().unwrap();

        Self {
            value: rosaries as usize,
        }
    }

    fn get_title() -> String {
        "Rosaries".into()
    }

    fn get_value(&self) -> usize {
        self.value
    }

    fn get_description() -> String {
        "Amount of rosaries Hornet currently holds".into()
    }

    fn set_value(&mut self, json: &mut str, value: usize) {
        todo!()
    }
}
