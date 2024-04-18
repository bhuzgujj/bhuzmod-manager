use std::fs;
use serde_xml_rs::{from_str, to_string};
use crate::modsettings::Save;

mod modsettings;

fn main() {
    let document = fs::read_to_string("./resource/modsettings.lsx").unwrap();
    let save: Save = from_str(document.as_str()).unwrap();
    dbg!(&save);
    println!("{}", to_string(&save).unwrap())
}
