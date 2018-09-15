pub type KeyFrame = (f64, f64); // (frac, value)

#[allow(dead_code)]
pub enum Easing {
    EaseIn,
    EaseOut,
    EaseInOut,
}

type EasingFn = fn(f64) -> f64;

fn ease_in(x: f64) -> f64 {
    x.powi(3)
}

fn ease_out(x: f64) -> f64 {
    (x - 1.0).powi(3) + 1.0
}

fn ease_in_out(x: f64) -> f64 {
    if x < 0.5 {
        (x * 2.0).powi(3) * 0.5
    } else {
        (x * 2.0 - 2.0).powi(3) * 0.5 + 1.0
    }
}

fn get_easing_fn(easing: Easing) -> EasingFn {
    match easing {
        Easing::EaseIn => ease_in,
        Easing::EaseOut => ease_out,
        Easing::EaseInOut => ease_in_out,
    }
}

pub struct Tween {
    keyframes: Vec<KeyFrame>,
    length: f64,
    duration: f64,
    easing_fn: EasingFn,
    looped: bool,
    playing: bool,
}

impl Tween {
    pub fn new(keyframes: Vec<KeyFrame>, length: f64, easing: Easing, looped: bool) -> Tween {
        assert!(length != 0.0);
        assert!(keyframes.len() >= 2);
        Tween {
            keyframes,
            length,
            duration: 0.0,
            easing_fn: get_easing_fn(easing),
            looped,
            playing: false,
        }
    }

    pub fn update(&mut self, dt: f64) -> () {
        if self.playing == false {
            return;
        };
        self.duration += dt;
        if self.duration > self.length {
            match self.looped {
                true => {
                    self.duration -= self.length;
                }
                false => {
                    self.playing = false;
                    self.duration = self.length;
                }
            }
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

                    // Calculate the current interpolated value using the desired easing
                    (self.easing_fn)(frac) * (to.1 - from.1) + from.1
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_easing_function(input: Vec<f64>, expected_output: Vec<f64>, f: EasingFn) {
        input
            .into_iter()
            .map(f)
            .zip(expected_output.into_iter())
            .for_each(|(input, expected_output)| assert_approx_eq!(input, expected_output, 1e-4));
    }

    #[test]
    fn it_should_calcualate_correct_ease_in() {
        let input = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let expected_output = vec![0.0, 0.0156, 0.125, 0.4219, 1.0];
        test_easing_function(input, expected_output, ease_in);
    }

    #[test]
    fn it_should_calcualate_correct_ease_out() {
        let input = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let expected_output = vec![0.0, 0.5781, 0.875, 0.9844, 1.0];
        test_easing_function(input, expected_output, ease_out);
    }

    #[test]
    fn it_should_calcualate_correct_ease_in_out() {
        let input = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let expected_output = vec![0.0, 0.0625, 0.5, 0.9375, 1.0];
        test_easing_function(input, expected_output, ease_in_out);
    }
}
