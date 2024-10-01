use anyhow::Result;
use app::Application;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowAttributes};

#[path = "raytracer/app.rs"]
mod app;

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let win_attr = WindowAttributes::default()
        .with_title("Raytracer")
        .with_inner_size(PhysicalSize::new(800, 600));
    let window = event_loop.create_window(win_attr)?;

    let mut application = Application::new(&window)
        .await?;

    event_loop.run_app(&mut application)?;

    Ok(())
}
