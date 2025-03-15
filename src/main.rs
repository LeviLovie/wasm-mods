use anyhow::{Error, Result};
use mod_manager::{ModContext, ModManager};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::Duration;
use tracing::info;
use utils::logging::*;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let sdl_context = sdl2::init().anyhow()?;
    let video_subsystem = sdl_context.video().anyhow()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .anyhow()?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .anyhow()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().anyhow()?;

    let context = ModContext {
        game_version: "1.0".to_string(),
        api_version: "1.0".to_string(),
    };
    let mut manager = ModManager::new("wasm", context)?;
    manager.load_all_mods()?;

    let init_instant = std::time::Instant::now();
    manager.call_init()?;
    info!("Initialized in {}us", init_instant.elapsed().as_micros());

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let update_instant = std::time::Instant::now();
        manager.update_all_mods(1000.0 / 16.0)?;
        info!("Updated in {}us", update_instant.elapsed().as_micros());

        let draw_instant = std::time::Instant::now();
        manager.call_draw()?;
        info!("Drawn in {}us", draw_instant.elapsed().as_micros());

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        {
            let color = {
                let storages_ref = manager.storages();
                let storages = storages_ref.lock().unwrap();
                storages.color.get().clone()
            };
            let storages_ref = manager.storages();
            let mut storages = storages_ref.lock().unwrap();
            let textures = &mut storages.textures;
            for (x, y, w, h) in textures.iter() {
                canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(*x as i32, *y as i32, *w, *h))
                    .anyhow()?;
            }
            storages.clear();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    let unload_instant = std::time::Instant::now();
    manager.unload_all_mods()?;
    info!("Unloaded in {}us", unload_instant.elapsed().as_micros());

    Ok(())
}
