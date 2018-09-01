// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate winit;

use bgfx::{Bgfx, PlatformData, RenderFrame, RendererType};

#[cfg(target_os = "macos")]
use winit::os::macos::WindowExt;

#[cfg(target_os = "windows")]
use winit::os::windows::WindowExt;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Events received by the main thread, sent by the render thread.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {
  /// Window close event.
  Close,

  /// Window size event.
  Size(u16, u16),
}

/// Event queue for communicating with the render thread.
pub struct EventQueue {
  event_rx: Receiver<Event>,
}

impl EventQueue {
  /// Handles events received from the render thread. If there are no events to process, returns
  /// instantly.
  ///
  /// Returns `true` if the app should exit.
  pub fn handle_events(
    &self,
    bgfx: &Bgfx,
    width: &mut u16,
    height: &mut u16,
    reset: bgfx::ResetFlags,
  ) -> bool {
    let mut should_close = false;

    while let Ok(result) = self.event_rx.try_recv() {
      match result {
        Event::Close => should_close = true,
        Event::Size(w, h) => {
          *width = w;
          *height = h;
          bgfx.reset(w, h, reset);
        }
      }
    }

    should_close
  }
}

/// Process window events on the render thread.
fn process_events(events_loop: &mut winit::EventsLoop, event_tx: &Sender<Event>) -> bool {
  let mut should_close = false;

  events_loop.poll_events(|event| match event {
    winit::Event::WindowEvent { event, .. } => match event {
      winit::WindowEvent::CloseRequested => {
        should_close = true;
        event_tx.send(Event::Close).unwrap();
      }
      winit::WindowEvent::Resized(logical_size) => {
        event_tx
          .send(Event::Size(
            logical_size.width as u16,
            logical_size.height as u16,
          ))
          .unwrap();
      }
      _ => (),
    },
    _ => (),
  });

  should_close
}

/// Loads the contents of a file and returns it.
fn load_file(name: &str) -> Vec<u8> {
  let mut data = Vec::new();
  let mut file = File::open(name).unwrap();
  file.read_to_end(&mut data).unwrap();
  data
}

/// Loads the two given shaders from disk, and creates a program containing the loaded shaders.
pub fn load_program<'a, 'b>(
  bgfx: &'a Bgfx,
  vsh_name: &'b str,
  fsh_name: &'b str,
) -> bgfx::Program<'a> {
  let exe_path = env::current_exe().unwrap();
  let exe_stem = exe_path.file_stem().unwrap();
  let renderer_path = match bgfx.get_renderer_type() {
    RendererType::Noop => "dx9", // Load shaders from somewhere, but bgfx ignores them
    RendererType::Direct3D9 => "dx9",
    RendererType::Direct3D11 => "dx11",
    RendererType::Direct3D12 => "dx11", // Uses same shaders as dx11
    RendererType::Metal => "metal",
    RendererType::OpenGLES => "nacl",
    RendererType::OpenGL => "gl",
    RendererType::Vulkan => "vulkan",
    _ => panic!("Unsupported renderer"),
  };
  let out_path = format!("examples/{}/out", exe_stem.to_str().unwrap());
  let vsh_path = format!("{}/{}/{}.bin", out_path, renderer_path, vsh_name);
  let fsh_path = format!("{}/{}/{}.bin", out_path, renderer_path, fsh_name);
  let vsh_mem = bgfx::Memory::copy(bgfx, &load_file(&vsh_path));
  let fsh_mem = bgfx::Memory::copy(bgfx, &load_file(&fsh_path));
  let vsh = bgfx::Shader::new(vsh_mem);
  let fsh = bgfx::Shader::new(fsh_mem);

  bgfx::Program::new(vsh, fsh)
}

/// Set the platform data to be used by BGFX.
#[cfg(target_os = "linux")]
fn init_bgfx_platform(window: &Window) {
  PlatformData::new()
    .display(unsafe { window.platform_display() as *mut std::os::raw::c_void })
    .window(unsafe { window.platform_window() as *mut std::os::raw::c_void })
    .apply()
    .unwrap();
}

#[cfg(target_os = "macos")]
fn get_platform_window(window: &winit::Window) -> *mut std::os::raw::c_void {
  return window.get_nswindow();
}

#[cfg(target_os = "windows")]
fn get_platform_window(window: &winit::Window) -> *mut std::os::raw::c_void {
  return window.get_hwnd() as *mut std::os::raw::c_void;
}

/// Set the platform data to be used by BGFX.
#[cfg(not(target_os = "linux"))]
fn init_bgfx_platform(window: &winit::Window) {
  PlatformData::new()
    .window(get_platform_window(window))
    .apply()
    .unwrap();
}

pub fn run_example<M>(width: u16, height: u16, main: M) -> Result<(), String>
where
  M: Send + 'static + FnOnce(EventQueue),
{
  let mut events_loop = winit::EventsLoop::new();
  let window = winit::WindowBuilder::new()
    .with_dimensions(winit::dpi::LogicalSize::new(width as f64, height as f64))
    .with_title(String::from("BGFX"))
    .build(&events_loop)
    .map_err(|err| format!("Error creating window: {}", err))?;

  // Create the channel used for communication between the main and render threads.
  let (event_tx, event_rx) = channel::<Event>();

  // Set the platform data for BGFX to use.
  init_bgfx_platform(&window);

  // Initialize this thread as the render thread by pumping it once *before* calling bgfx::init.
  bgfx::render_frame();

  // Spawn a new thread to use as the main thread.
  let main_thread = thread::spawn(move || {
    main(EventQueue { event_rx: event_rx });
  });

  // Pump window events until the window is closed.
  while !process_events(&mut events_loop, &event_tx) {
    bgfx::render_frame();
  }

  // Pump the render thread until the main thread has shut down.
  while bgfx::render_frame() != RenderFrame::NoContext {
    thread::sleep(Duration::from_millis(1));
  }

  main_thread
    .join()
    .map_err(|_| "Error joining main theread:")?;
  return Ok(());
}
