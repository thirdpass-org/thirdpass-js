use thirdpass_core::extension::FromLib;
use thirdpass_js_lib;

fn main() {
    let mut extension = thirdpass_js_lib::JsExtension::new();
    thirdpass_core::extension::commands::run(&mut extension).unwrap();
}
