use learn::utils::{conversion::*, some::*};

fn main() {
    some();
    conversion();
}

fn some() {
    Some::last_one_of_number();
    Some::last_tow_of_string(2);

    Some::first_tow_of_string(2);

    Some::cut_tow_of_string(2);
}

fn conversion() {
    Conversion::from_str();
    Conversion::from_string();
    Conversion::str_string_to_number();
    Conversion::str_to_number();
    Conversion::string_to_number();
    Conversion::vec_of_bytes();
    Conversion::vec_of_chars();
}
