// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};

use core_lib::{
    decode,
    values::{ValueType as VT, initialize_all_save_values},
};
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    const SAVE_BYTES: &[u8] = include_bytes!("../../user1.dat");
    let mut json_string = String::from_utf8(decode(SAVE_BYTES)?)?;

    let mut values = initialize_all_save_values(&json_string)?;
    let mut values_sorted: Vec<_> = values.into_iter().collect();

    values_sorted.sort_by(|(_, a), (_, b)| {
        if a.get_priority() > b.get_priority() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    let values_vec: Vec<ListDesc> = values_sorted
        .into_iter()
        .map(|(key, save)| ListDesc {
            title: SharedString::from(save.get_title()),
            description: SharedString::from(save.get_description()),
            value: match save.get_value() {
                VT::String(s) => SharedString::from(s),
                VT::Int(i) => SharedString::from(format!("{i}")),
                VT::Boolean(b) => SharedString::from(format!("{b}")),
            },
            value_type: match save.get_value() {
                VT::Int(_) => ValueType::Integer,
                VT::String(_) => ValueType::Str,
                VT::Boolean(_) => ValueType::Boolean,
            },
            key: SharedString::from(key),
        })
        .collect();

    let model = Rc::new(VecModel::from(values_vec));

    ui.set_values(ModelRc::from(model));

    ui.run()?;

    Ok(())
}
