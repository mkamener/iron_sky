pub type KeyFrame = (f64, f64); // (frac, value)

pub struct Tween {
    keyframes: Vec<KeyFrame>,
    length: f64,
    duration: f64,
    looped: bool,
    playing: bool,
}

impl Tween {
    pub fn new(keyframes: Vec<KeyFrame>, length: f64, looped: bool, playing: bool) -> Tween {
        assert!(length != 0.0);
        assert!(keyframes.len() >= 2);
        Tween {
            keyframes,
            length,
            duration: 0.0,
            looped,
            playing,
        }
    }

    pub fn update(&mut self, dt: f64) -> () {
        if self.playing == false {
            return;
        };
        self.duration += dt;
        if self.duration > self.length {
            self.playing = false;
            self.duration = self.length;
        }
    }

    pub fn get_val(&self) -> f64 {
        if self.playing {
            let total_frac = self.duration / self.length;

            // Find the index of the first keyframe that is greater than total_frac
            let maybe_idx = self.keyframes
                .iter()
                .position(|&keyframe| keyframe.0 >= total_frac);

            match maybe_idx {
                Some(idx) if idx > 0 => {
                    let from = self.keyframes[idx - 1];
                    let to = self.keyframes[idx];
                    let frac = (total_frac - from.0) / (to.0 - from.0);

                    // Calculate the current interpolated value
                    frac * (to.1 - from.1)
                }
                Some(_) => self.keyframes[0].1,
                None => self.keyframes.last().unwrap().1,
            }
        } else {
            // Keyframes must have at least one value or else it will panic
            self.keyframes.last().unwrap().0
        }
    }

    pub fn reset(&mut self) -> () {
        self.playing = true;
        self.duration = 0.0;
    }

    pub fn stop(&mut self) -> () {
        self.playing = false;
        self.duration = self.length;
    }
}
