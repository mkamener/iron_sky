pub mod game {
    // Draw collider outlines
    pub const DRAW_DEBUG: bool = false;

    pub const MAX_MISSILES: u32 = 6;
    pub const MAX_PICKUPS: u32 = 4;

    // Score
    pub const POINTS_PER_MISSILE: u32 = 100;
    pub const POINTS_PER_PICKUP: u32 = 500;
}

pub mod window {
    pub const SIZE: (u32, u32) = (1440, 720);
}

pub mod player {
    pub const SPEED: f64 = 800.0;
    pub const ANGULAR_VELOCITY: f64 = 240.0; // Degrees per second

    // Collider
    pub const COLLIDER_RADIUS: f64 = 30.0;

    // Explosion Settings
    pub const EXPLOSION_LENGTH: f64 = 1.0;
    pub const EXPLOSION_ZOOM: f64 = 1.5;
}

pub mod missile {
    pub const MAX_SPEED: f64 = 1500.0;
    pub const ACCELERATION: f64 = 3500.0;

    // Collider
    pub const COLLIDER_RADIUS: f64 = 20.0;

    // Explosion Settings
    pub const EXPLOSION_LENGTH: f64 = 0.5;
    pub const EXPLOSION_ZOOM: f64 = 1.0;
}

pub mod missile_generator {
    pub const SPAWN_RADIUS: f64 = 1200.0;
    pub const TIME_TO_APPEAR: f64 = 5.0;
}

pub mod pickup {
    pub const ROTATION_PERIOD: f64 = 4.0;
    pub const MAX_TIME: f64 = 10.0;

    // Collider
    pub const COLLIDER_RADIUS: f64 = 20.0;
}

pub mod pickup_generator {
    pub const MIN_SPAWN_RADIUS: f64 = 500.0;
    pub const MAX_SPAWN_RADIUS: f64 = 1000.0;
    pub const TIME_TO_APPEAR: f64 = 4.0;
}

pub mod offscreen_pointer {
    pub const SCALE: f64 = 0.3;
    pub const OFFSET: f64 = 60.0;

    pub const OBJ_SCALE: f64 = 0.4;
}

pub mod ui {
    pub const SHADOW_OFFSET: f64 = 2.0;
    pub const SHADOW_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    pub const SCORE_V_OFFSET: f64 = 120.0;
    pub const SCORE_H_OFFSET: f64 = 95.0;
    pub const SCORE_COLOR: [f32; 4] = [0.17, 0.74, 0.18, 1.0];
    pub const SCORE_FONT_SIZE: u32 = 32;

    pub mod game_over {
        pub const GAME_OVER_V_OFFSET: f64 = 350.0;
        pub const GAME_OVER_H_OFFSET: f64 = 455.0;
        pub const GAME_OVER_COLOR: [f32; 4] = [0.17, 0.74, 0.18, 1.0];
        pub const GAME_OVER_FONT_SIZE: u32 = 72;

        pub const FADE_IN_LENGTH: f64 = 0.5;

        pub const RESTART_V_OFFSET: f64 = 540.0;
        pub const RESTART_H_OFFSET: f64 = 500.0;
        pub const RESTART_COLOR: [f32; 4] = [0.17, 0.74, 0.18, 1.0];
        pub const RESTART_FONT_SIZE: u32 = 24;

        pub const FADE_IN_OUT_LENGTH: f64 = 1.5;
    }
}

pub mod background {
    pub const FILES: &'static [(&str, f64)] = &[
        ("bkgd_0.png", 0.0),
        ("bkgd_1.png", 0.01),
        ("bkgd_2.png", 0.02),
        ("bkgd_3.png", 0.03),
        ("bkgd_4.png", 0.04),
        ("bkgd_5.png", 0.05),
        ("bkgd_6.png", 0.5),
        ("bkgd_7.png", 1.0),
    ];
}
