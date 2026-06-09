use std::error::Error;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    #[cfg(not(target_arch = "wasm32"))]
    graphics::run()?;
    #[cfg(target_arch = "wasm32")]
    run_web();
    Ok(())
}
