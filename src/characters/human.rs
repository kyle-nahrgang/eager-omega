use macroquad::prelude::*;
use macroquad_tiled::Map;

use crate::world_map::TileIndex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum HairStyle {
    Base,
    Bowl,
    Curly,
    Long,
    Mop,
    Short,
    Spikey,
}

impl HairStyle {
    pub fn to_str(&self) -> &'static str {
        match self {
            HairStyle::Base => "base",
            HairStyle::Bowl => "bowlhair",
            HairStyle::Curly => "curlyhair",
            HairStyle::Long => "longhair",
            HairStyle::Mop => "mophair",
            HairStyle::Short => "shorthair",
            HairStyle::Spikey => "spikeyhair",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CharacterAction {
    IDLE,
    ROLL,
    WALKING,
}

const IDLE_ACTION: CharacterAction = CharacterAction::IDLE;
const MOVE_ACTION: CharacterAction = CharacterAction::WALKING;
const FRAME_TIME: f32 = 0.15;

impl CharacterAction {
    pub fn dirname(&self) -> &'static str {
        match self {
            CharacterAction::IDLE => "IDLE",
            CharacterAction::ROLL => "ROLL",
            CharacterAction::WALKING => "WALKING",
        }
    }

    pub fn file_component(&self) -> &'static str {
        match self {
            CharacterAction::IDLE => "idle",
            CharacterAction::ROLL => "roll",
            CharacterAction::WALKING => "walk",
        }
    }

    pub fn frame_count(&self) -> u16 {
        match self {
            CharacterAction::IDLE => 9,
            CharacterAction::WALKING => 8,
            CharacterAction::ROLL => 10,
        }
    }

    pub fn get_path(&self, hair_style: HairStyle) -> String {
        format!(
            "assets/Characters/Human/{}/{}_{}_strip{}.png",
            self.dirname(),
            hair_style.to_str(),
            self.file_component(),
            self.frame_count(),
        )
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            CharacterAction::IDLE => 0.0,
            CharacterAction::WALKING => 60.0,
            CharacterAction::ROLL => 120.0,
        }
    }
}

pub struct Human {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub hair_style: HairStyle,
    textures: std::collections::HashMap<(HairStyle, CharacterAction), Texture2D>,
    frame_timer: f32,
    current_frame: u16,
    current_action: CharacterAction,
    flip_x: bool,
}

impl Human {
    pub async fn new(position: Vec2, hair_style: HairStyle) -> Self {
        let mut textures = std::collections::HashMap::new();
        for action in [IDLE_ACTION, MOVE_ACTION, CharacterAction::ROLL] {
            if hair_style != HairStyle::Base {
                let texture_path = action.get_path(HairStyle::Base);
                let texture = load_texture(&texture_path).await.unwrap();
                texture.set_filter(FilterMode::Nearest);
                textures.insert((HairStyle::Base, action), texture);
            }

            let texture_path = action.get_path(hair_style);
            let texture = load_texture(&texture_path).await.unwrap();
            texture.set_filter(FilterMode::Nearest);
            textures.insert((hair_style, action), texture);
        }

        Self {
            position,
            velocity: Vec2::ZERO,
            speed: MOVE_ACTION.get_speed(),
            hair_style,
            textures,
            frame_timer: 0.0,
            current_frame: 0,
            current_action: IDLE_ACTION,
            flip_x: false,
        }
    }

    pub fn update(&mut self, dt: f32, tiled_map: &Map) {
        let mut direction = vec2(0.0, 0.0);
        let prev_action = self.current_action;

        let mut is_rolling = self.current_action == CharacterAction::ROLL
            && self.current_frame != CharacterAction::ROLL.frame_count() - 1;

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) || (is_rolling && self.flip_x) {
            direction.x -= 1.0;
            self.flip_x = true;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) || (is_rolling && !self.flip_x) {
            direction.x += 1.0;
            self.flip_x = false;
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }

        if is_key_down(KeyCode::Space) && !is_rolling {
            is_rolling = true;
            self.current_action = CharacterAction::ROLL;
            self.current_frame = 0;
            self.frame_timer = 0.0;
            self.speed = CharacterAction::ROLL.get_speed();
        }

        if !is_rolling {
            self.speed = MOVE_ACTION.get_speed();
        }

        let new_velocity = if direction.length() > 0.0 {
            direction.normalize() * self.speed
        } else {
            Vec2::ZERO
        };

        let new_position = self.position + new_velocity * dt;

        let new_tile_x = (new_position.x / 16.0).floor() as u32;
        let new_tile_y = (new_position.y / 16.0).floor() as u32;

        let is_walkable = match tiled_map.get_tile("ground", new_tile_x, new_tile_y) {
            Some(t) => true,
            None => match tiled_map.get_tile("ladders", new_tile_x, new_tile_y) {
                Some(_) => true,
                None => false,
            },
        };

        if direction.length() > 0.0 && is_walkable {
            self.velocity = new_velocity;
            self.current_action = if is_rolling {
                CharacterAction::ROLL
            } else {
                MOVE_ACTION
            };
        } else {
            self.velocity = Vec2::ZERO;
            self.current_action = if is_rolling {
                CharacterAction::ROLL
            } else {
                IDLE_ACTION
            };
        }

        if prev_action != self.current_action {
            self.current_frame = 0;
            self.frame_timer = 0.0;
        }

        self.position += self.velocity * dt;

        // --- Animation ---
        self.frame_timer += dt;
        if self.frame_timer >= FRAME_TIME {
            self.frame_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % self.current_action.frame_count();
        }
    }

    pub fn draw(&self) {
        let base_texture = self
            .textures
            .get(&(HairStyle::Base, self.current_action))
            .unwrap();
        draw_texture_ex(
            &base_texture,
            self.position.x - 24.0,
            self.position.y - 16.0,
            WHITE,
            DrawTextureParams {
                source: Some(self.player_uv(self.current_frame)),
                dest_size: Some(vec2(48.0, 32.0)),
                flip_x: self.flip_x,
                ..Default::default()
            },
        );
        if self.hair_style != HairStyle::Base {
            let hair_texture = self
                .textures
                .get(&(self.hair_style, self.current_action))
                .unwrap();
            draw_texture_ex(
                &hair_texture,
                self.position.x - 24.0,
                self.position.y - 16.0,
                WHITE,
                DrawTextureParams {
                    source: Some(self.player_uv(self.current_frame)),
                    dest_size: Some(vec2(48.0, 32.0)),
                    flip_x: self.flip_x,
                    ..Default::default()
                },
            );
        }
    }

    fn player_uv(&self, frame: u16) -> Rect {
        let frame_width = 96.0;
        let frame_height = 64.0;

        Rect::new(frame as f32 * frame_width, 0.0, frame_width, frame_height)
    }
}
