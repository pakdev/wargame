extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx_core::format::{DepthStencil, Rgba8};

fn main() {
    let builder = glutin::WindowBuilder::new().with_title("Wargame".to_string());
    let (window, device, factory, rtv, stv) = 
        gfx_window_glutin::init::<Rgba8, DepthStencil>(builder);

    let _ = unsafe { window.make_current() };

    for event in window.wait_events() {

        println!("{:?}", event);

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
