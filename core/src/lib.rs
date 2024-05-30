use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Logic {
    pub ball_x: usize,
    pub radius: usize,
    pub mass: usize,
    pub gravity: usize,

    ball_y: RefCell<usize>,
    frame: RefCell<usize>,
    dy: RefCell<isize>,
    version: &'static str,
    display_height: usize,
}

#[wasm_bindgen]
impl Logic {
    pub fn new(display_width: usize, display_height: usize) -> Self {
        Self {
            ball_x: display_width / 2,
            ball_y: RefCell::new(display_height / 2),
            radius: 20,
            dy: RefCell::new(2),
            mass: 1,
            gravity: 1,
            frame: RefCell::new(0),
            version: "1.5",
            display_height,
        }
    }

    pub fn tick(&self) {
        let mut dy = self.dy.borrow_mut();
        let mut ball_y = self.ball_y.borrow_mut();
        let mut frame = self.frame.borrow_mut();

        *dy += (self.gravity * self.mass) as isize;
        *ball_y += *dy as usize;
        if *ball_y + self.radius > self.display_height {
            *ball_y = self.display_height - self.radius;
            *dy = -*dy;
        }
        *frame = if *frame == 60 { 0 } else { *frame + 1 };
    }

    pub fn version(&self) -> String {
        self.version.to_string()
    }

    pub fn dy(&self) -> isize {
        self.dy.clone().take()
    }

    pub fn ball_y(&self) -> usize {
        self.ball_y.clone().take()
    }

    pub fn frame(&self) -> usize {
        self.frame.clone().take()
    }
}
