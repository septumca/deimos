use ggez::{graphics};

pub struct Blip {
    frames: Vec<graphics::Rect>,
    act_frame: usize,
    position: (f32, f32)
}

impl Blip {
    pub fn new(position: (f32, f32)) -> Blip {
        let frame_indexes: Vec<(f32, f32)> = vec![
            (0.0, 1.0),
            (1.0, 1.0),
            (2.0, 1.0),
            (3.0, 1.0),
        ];
        let frames = super::points_to_frames(frame_indexes);

        Blip {
            frames,
            act_frame: 0,
            position
        }
    }

    pub fn update_animation(&mut self) {
        self.act_frame = (self.act_frame + 1) % self.frames.len();
    }

    pub fn get_position(&self) -> &(f32, f32) {
        &self.position
    }
}

impl super::WithFrame for Blip {
    fn get_frame(&self) -> &graphics::Rect {
        &self.frames[self.act_frame]
    }
}