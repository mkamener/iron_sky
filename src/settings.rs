pub mod game {
    // Draw collider outlines
    pub const DRAW_DEBUG: bool = false;

    pub const MAX_MISSILES: u32 = 4;
}

pub mod window {
    pub const SIZE: (u32, u32) = (1440, 960);
}

pub mod player {
    pub const SPEED: f64 = 800.0;

    // Collider
    pub const COLLIDER_RADIUS: f64 = 30.0;

    // Explosion Settings
    pub const EXPLOSION_LENGTH: f64 = 1.0;
    pub const EXPLOSION_ZOOM: f64 = 1.5;
}

pub mod missile {
    pub const MAX_SPEED: f64 = 1200.0;
    pub const ACCELERATION: f64 = 3000.0;

    // Collider
    pub const COLLIDER_RADIUS: f64 = 20.0;

    // Explosion Settings
    pub const EXPLOSION_LENGTH: f64 = 0.5;
    pub const EXPLOSION_ZOOM: f64 = 1.0;
}

pub mod missile_generator {
    pub const SPAWN_RADIUS: f64 = 1000.0;
    pub const TIME_TO_APPEAR: f64 = 5.0;
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
