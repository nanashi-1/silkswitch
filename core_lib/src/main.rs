use std::fs::write;

use core_lib::{decode, values::initialize_all_save_values};

fn main() {
    // const FILE_BYTES: &[u8] = include_bytes!("../user1.dat.json");
    //
    // let encoded_bytes = encode(FILE_BYTES).unwrap();
    //
    // write("user3.dat", encoded_bytes).unwrap();

    // const FILE_BYTES: &[u8] = include_bytes!("../user1.dat");
    //
    // let decoded_bytes = decode(FILE_BYTES).unwrap();
    //
    // write("user1.dat.json", decoded_bytes).unwrap();

    let bytes = include_bytes!("../user1.dat");
    let decoded_bytes = decode(bytes).unwrap();

    write("user1.dat.json", &decoded_bytes).unwrap();

    let json_str = unsafe { String::from_utf8_unchecked(decoded_bytes) };

    let values = initialize_all_save_values(&json_str).unwrap();

    dbg!(values);
}
