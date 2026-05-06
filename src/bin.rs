use thirdpass_js_lib;
use thirdpass_lib::extension::FromLib;

fn main() {
    let mut extension = thirdpass_js_lib::JsExtension::new();
    thirdpass_lib::extension::commands::run(&mut extension).unwrap();
}
