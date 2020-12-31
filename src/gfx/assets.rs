use process_path::get_executable_path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::path::PathBuf;

const AGENT_SPRITE_PATH: &str = "assets/agent.png";
const AGENT_EYES_SPRITE_PATH: &str = "assets/agent_eyes.png";

/// Struct for keeping track of SDL2 assets.
pub struct Assets<'a> {
    pub agent_sprite: Texture<'a>,
    pub agent_eyes_sprite: Texture<'a>,
}

impl<'a> Assets<'a> {
    /// Load assets from disk.
    /// NOTE: The executable must be three layers deeper than the "assets" directory,
    ///       for example ./target/debug/mini-universe next to ./assets.
    pub fn load(creator: &'a mut TextureCreator<WindowContext>) -> Assets<'a> {
        let assets_dir_pathbuf = PathBuf::from(
            get_executable_path()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        );

        let mut agent_sprite_pathbuf = assets_dir_pathbuf.clone();
        agent_sprite_pathbuf.push(AGENT_SPRITE_PATH);
        let agent_sprite = creator
            .load_texture(agent_sprite_pathbuf.as_path())
            .unwrap();

        let mut agent_eyes_sprite_pathbuf = assets_dir_pathbuf.clone();
        agent_eyes_sprite_pathbuf.push(AGENT_EYES_SPRITE_PATH);
        let agent_eyes_sprite = creator
            .load_texture(agent_eyes_sprite_pathbuf.as_path())
            .unwrap();

        Assets {
            agent_sprite,
            agent_eyes_sprite,
        }
    }
}
