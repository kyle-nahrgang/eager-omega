use macroquad::prelude::*;

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
enum Action {
    ATTACK,
    AXE,
    CARRY,
    CASTING,
    CAUGHT,
    DEATH,
    DIG,
    DOING,
    HAMMERING,
    HURT,
    IDLE,
    JUMP,
    MINING,
    REELING,
    ROLL,
    RUN,
    SWIMMING,
    WAITING,
    WALKING,
    WATERING,
}

const FRAME_TIME: f32 = 0.15;

impl Action {
    pub fn dirname(&self) -> &'static str {
        match self {
            Action::ATTACK => "ATTACK",
            Action::AXE => "AXE",
            Action::CARRY => "CARRY",
            Action::CASTING => "CASTING",
            Action::CAUGHT => "CAUGHT",
            Action::DEATH => "DEATH",
            Action::DIG => "DIG",
            Action::DOING => "DOING",
            Action::HAMMERING => "HAMMERING",
            Action::HURT => "HURT",
            Action::IDLE => "IDLE",
            Action::JUMP => "JUMP",
            Action::MINING => "MINING",
            Action::REELING => "REELING",
            Action::ROLL => "ROLL",
            Action::RUN => "RUN",
            Action::SWIMMING => "SWIMMING",
            Action::WAITING => "WAITING",
            Action::WALKING => "WALKING",
            Action::WATERING => "WATERING",
        }
    }

    pub fn file_component(&self) -> &'static str {
        match self {
            Action::ATTACK => "ATTACK",
            Action::AXE => "AXE",
            Action::CARRY => "CARRY",
            Action::CASTING => "CASTING",
            Action::CAUGHT => "CAUGHT",
            Action::DEATH => "DEATH",
            Action::DIG => "DIG",
            Action::DOING => "DOING",
            Action::HAMMERING => "HAMMERING",
            Action::HURT => "HURT",
            Action::IDLE => "idle",
            Action::JUMP => "JUMP",
            Action::MINING => "MINING",
            Action::REELING => "REELING",
            Action::ROLL => "roll",
            Action::RUN => "run",
            Action::SWIMMING => "SWIMMING",
            Action::WAITING => "WAITING",
            Action::WALKING => "walk",
            Action::WATERING => "WATERING",
        }
    }

    pub fn frame_count(&self) -> u16 {
        match self {
            Action::IDLE => 9,
            Action::WALKING => 8,
            Action::RUN => 8,
            Action::JUMP => 9,
            Action::ATTACK => 10,
            Action::ROLL => 10,
            _ => 1, // Default to 1 frame for unimplemented actions
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
}

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub hair_style: HairStyle,
    textures: std::collections::HashMap<(HairStyle, Action), Texture2D>,
    frame_timer: f32,
    current_frame: u16,
    current_action: Action,
    flip_x: bool,
}

impl Player {
    pub async fn new(position: Vec2, speed: f32, hair_style: HairStyle) -> Self {
        let mut textures = std::collections::HashMap::new();
        for action in [Action::IDLE, Action::WALKING] {
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
            speed,
            hair_style,
            textures,
            frame_timer: 0.0,
            current_frame: 0,
            current_action: Action::IDLE,
            flip_x: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut direction = vec2(0.0, 0.0);
        let prev_action = self.current_action;

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
            self.flip_x = true;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
            self.flip_x = false;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }

        if direction.length() > 0.0 {
            self.velocity = direction.normalize() * self.speed;
            self.current_action = Action::ROLL;
        } else {
            self.velocity = Vec2::ZERO;
            self.current_action = Action::IDLE;
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
            self.position.x,
            self.position.y,
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
                self.position.x,
                self.position.y,
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
