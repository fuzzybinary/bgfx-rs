// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate bgfx_sys;
extern crate examples_lib;

use bgfx::*;
use examples_lib::EventQueue;
use std::cmp::max;

const LOGO: &'static [u8] = include_bytes!("logo.bin");

fn example(events: EventQueue) {
    let mut width: u16 = 1280;
    let mut height: u16 = 720;
    let debug = DEBUG_TEXT;
    let reset = RESET_VSYNC;

    let mut init_params: bgfx_sys::BgfxInitParams = Default::default();
    init_params.resolution.width = 1280;
    init_params.resolution.height = 720;

    let bgfx = bgfx::init(init_params).unwrap();

    // Enable debug text.
    bgfx.set_debug(debug);

    // Set view 0 clear state.
    let clear = CLEAR_COLOR | CLEAR_DEPTH;
    bgfx.set_view_clear(0, clear, 0x303030ff, 1.0_f32, 0);

    while !events.handle_events(&bgfx, &mut width, &mut height, reset) {
        // Set view 0 default viewport.
        bgfx.set_view_rect(0, 0, 0, width, height);

        // This dummy draw call is here to make sure that view 0 is cleared
        // if no other draw calls are submitted to view 0.
        bgfx.touch(0);

        // Use debug font to print information about this example.
        let x: u16 = max(width / 2 / 8, 20) - 20;
        let y: u16 = max(height / 2 / 16, 6) - 6;
        bgfx.dbg_text_clear(None, None);
        bgfx.dbg_text_image(x, y, 40, 12, LOGO, 160);
        bgfx.dbg_text_print(0, 1, 0x4f, "examples/00-helloworld/main.rs");
        bgfx.dbg_text_print(0, 2, 0x6f, "Description: Initialization and debug text.");

        // Advance to next frame. Rendering thread will be kicked to
        // process submitted rendering primitives.
        bgfx.frame(false);
    }

    // bgfx will automatically be shut down when the local `bgfx` binding
    // goes out of scope.
}

fn main() {
    match examples_lib::run_example(1280, 720, example) {
        Ok(_) => (),
        Err(err) => println!("Error running example: {}", err)
    }
}
