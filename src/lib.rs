use wasm_bindgen::prelude::*;

const GRAVITY: f32 = -25.0;
const JUMP_SPEED: f32 = 9.0;
const MOVE_SPEED: f32 = 6.0;
const ACCEL: f32 = 40.0;
const GROUND_Y: f32 = 1.0;
const HALF_SIZE: f32 = 25.0;
const RESPAWN_Y: f32 = -20.0;

#[wasm_bindgen]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub grounded: bool,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Player {
        Player { x: 0.0, y: GROUND_Y, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, grounded: true }
    }

    pub fn step(&mut self, move_x: f32, move_z: f32, jump: bool, dt: f32) {
        let dt = dt.clamp(0.0, 0.05);

        let mut dx = move_x;
        let mut dz = move_z;
        let len = (dx * dx + dz * dz).sqrt();
        if len > 1.0 {
            dx /= len;
            dz /= len;
        }
        let max_dv = ACCEL * dt;
        self.vx += (dx * MOVE_SPEED - self.vx).clamp(-max_dv, max_dv);
        self.vz += (dz * MOVE_SPEED - self.vz).clamp(-max_dv, max_dv);

        if jump && self.grounded {
            self.vy = JUMP_SPEED;
            self.grounded = false;
        }
        self.vy += GRAVITY * dt;

        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.z += self.vz * dt;

        let over_plate = self.x.abs() <= HALF_SIZE && self.z.abs() <= HALF_SIZE;
        if over_plate && self.y <= GROUND_Y && self.vy <= 0.0 {
            self.y = GROUND_Y;
            self.vy = 0.0;
            self.grounded = true;
        } else {
            self.grounded = false;
        }

        if self.y < RESPAWN_Y {
            *self = Player::new();
        }
    }
      }
