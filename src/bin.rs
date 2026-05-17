use thirdpass_core::extension::FromLib;

fn main() {
    let mut extension = thirdpass_js_lib::JsExtension::new();
    thirdpass_core::extension::run_command(&mut extension).unwrap();
}
