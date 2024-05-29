use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Logic {
    pub ball_x: usize,
    pub ball_y: usize,
    pub radius: usize,
    pub dy: isize,
    pub mass: usize,
    pub gravity: usize,
    pub frame: usize,
    version: &'static str,
    display_height: usize,
}

#[wasm_bindgen]
impl Logic {
    pub fn new(display_width: usize, display_height: usize) -> Self {
        Self {
            ball_x: display_width / 2,
            ball_y: display_height / 2,
            radius: 20,
            dy: 2,
            mass: 1,
            gravity: 1,
            frame: 0,
            version: "1.1",
            display_height,
        }
    }

    pub fn tick(&mut self) {
        self.dy += (self.gravity * self.mass) as isize;
        self.ball_y += self.dy as usize;
        if self.ball_y + self.radius > self.display_height {
            self.ball_y = self.display_height - self.radius;
            self.dy = -self.dy;
        }
        self.frame = if self.frame == 60 { 0 } else { self.frame + 1 };
    }

    pub fn version(&self) -> String {
        self.version.to_string()
    }
}
