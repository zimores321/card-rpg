use crate::game_manager::TextureManager;
use crate::video::sdl_core::SDLCore;
use crate::game_manager::GameManager;

mod game_manager;
mod video;
mod events;
mod scenes;

fn main() -> Result<(), String>{
    let sdl_core = SDLCore::init()?;
    let mut texture_manager = TextureManager::new(&sdl_core.texture_creator);
    let mut game_manager = GameManager::init(&sdl_core.sdl_context, sdl_core.wincan, &mut texture_manager)?;
    game_manager.start_state_machine();

    Ok(())
}
