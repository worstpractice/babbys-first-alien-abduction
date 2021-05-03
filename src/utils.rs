use winit::{
  dpi::PhysicalSize,
  event_loop::EventLoop,
};

pub fn get_current_screen_resolution() -> (f32, f32) {
  let event_loop = EventLoop::new();
  let primary_monitor = event_loop.primary_monitor().unwrap();
  let size = primary_monitor.size();

  let PhysicalSize {
    width,
    height,
    ..
  } = size;

  (height as f32, width as f32)
}
