use crate::values::rosaries::Rosaries;

pub mod rosaries;

pub enum SaveValueTypes {
    Int(Box<dyn SaveValue<usize>>),
    String(Box<dyn SaveValue<String>>),
    Boolean(Box<dyn SaveValue<bool>>),
}

pub trait SaveValue<T> {
    fn new(json: &str) -> Self
    where
        Self: Sized;

    fn get_title() -> String
    where
        Self: Sized;
    fn get_description() -> String
    where
        Self: Sized;
    fn get_value(&self) -> T;

    fn set_value(&mut self, json: &mut str, value: T);
}

pub fn initialize_all_save_values(json: &str) -> Vec<SaveValueTypes> {
    let rosaries = Rosaries::new(json);

    vec![SaveValueTypes::Int(Box::new(rosaries))]
}
