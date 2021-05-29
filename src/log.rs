use web_sys::console;

pub enum Platform {
    wasm,
    windows,
}

pub fn logger(message: String, platform: &Platform) {
    match platform {
        Platform::wasm => unsafe {
            console::log_1(&message.into());
        },
        Platform::windows => {
            println!("{}", message);
        }
    }
}
