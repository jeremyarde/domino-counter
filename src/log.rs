use web_sys::console;

pub enum Platform {
    Wasm,
    Windows,
}

pub fn logger(message: String, platform: &Platform) {
    match platform {
        Platform::Wasm => unsafe {
            console::log_1(&message.into());
        },
        Platform::Windows => {
            println!("{}", message);
        }
    }
}
