use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;


use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::game_manager::TextureManager;
use crate::video::gfx::CAM_W;
use crate::video::gfx::CAM_H;
use crate::video::gfx::TILE_SIZE;

const SPEED_LIMIT: i32 = 3;
const ACCEL_RATE: i32 = 1;

//mod crate::video;

pub struct Overworld<'a> {
	wincan: &'a mut WindowCanvas,
	tile_map: [u8; 144], // <- Need to implement
	tile_set: Rc<Texture<'a>>,
	player: Player<'a>,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: &'a mut WindowCanvas)  -> Result<Self, String> {
		let tile_map = [0; 144];
		let tile_set = texture_manager.borrow_mut().load("assets/tile_sheet4x.png")?;
		let player = Player {
			x_pos: 0,
			y_pos: 0,
			x_vel: 0,
			y_vel: 0,
			sprite: texture_manager.borrow_mut().load("assets/player4x.png")?,
		};

		Ok(Overworld{
			wincan,
			tile_map,
			tile_set,
			player,
		})
	}
}

impl Scene for Overworld<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		let mut delta_x = 0;
		let mut delta_y = 0;
		// Matching events, most importantly KeyPress(k)'s
		match event {
			GameEvent::KeyPress(k) => {
				if k.eq(&Keycode::W) {delta_y -= ACCEL_RATE}
				if k.eq(&Keycode::A) {delta_x -= ACCEL_RATE}
				if k.eq(&Keycode::S) {delta_y += ACCEL_RATE}
				if k.eq(&Keycode::D) {delta_x += ACCEL_RATE}
				self.player.x_vel = (self.player.x_vel + delta_x)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				self.player.y_vel = (self.player.y_vel + delta_y)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
			},
			_ => {},
		}
	}

	fn render(&mut self) -> Result<(), String> {
		self.player.update_movement();
		// Draw background
		crate::video::gfx::fill_screen(&mut self.wincan, Color::RGB(0, 128, 128))?;
		// Draw sea tiles
		crate::video::gfx::tile_sprite_from_sheet(&mut self.wincan, &self.tile_set, (0, 0), (40*5, 40), (0, 0), (4, 18))?;
		// Draw player
		crate::video::gfx::draw_sprite(&mut self.wincan, &self.player.sprite, (self.player.x_pos, self.player.y_pos))?;
		
		self.wincan.present();

		Ok(())
	}
}

struct Player<'a> {
	x_pos: i32,
	y_pos: i32,
	x_vel: i32,
	y_vel: i32,
	sprite: Rc<Texture<'a>>,
}

impl<'a> Player<'a> {
	fn update_movement(&mut self) {
		// Check if player will go beyond the bounds of the camera
		// - If yes, set their velocity to 0
		if self.x_pos + self.x_vel > CAM_W as i32 - TILE_SIZE as i32 * 4 || self.x_pos + self.x_vel < 0 {
			self.x_vel = 0;
		}
		if self.y_pos + self.y_vel > CAM_H as i32 - TILE_SIZE as i32 * 4 || self.y_pos + self.y_vel < 0 {
			self.y_vel = 0;
		} 
		// Add velocity to position, clamp to ensure bounds are never exceeded.
		// TILE_SIZE * 4 because the tiles are scaled x4
		self.x_pos = (self.x_pos + self.x_vel).clamp(0, CAM_W as i32 - (TILE_SIZE as i32 * 4));
		self.y_pos = (self.y_pos + self.y_vel).clamp(0, CAM_H as i32 - (TILE_SIZE as i32 * 4));
	}
}
