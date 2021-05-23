#[cfg(not(target_env = "musl"))]
mod windows;

mod macos;

pub fn setup() {
    match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("macos") => macos::setup(),
        Ok("windows") => {
            if cfg!(not(target_env = "musl")) {
                windows::setup()
            } else {
                eprintln!("Cross compiling to windows-msvc is not supported from *-musl hosts")
            }
        }
        _ => {}
    }
}
