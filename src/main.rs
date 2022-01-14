use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_bouncer)
        .add_system(animate_translation)
        .add_system(handle_input)
        .run();
}

#[derive(Component)]
struct Bouncer {
    dx: f32,
    dy: f32,
}

impl Bouncer {
    pub fn flip_x(&mut self) {
        self.dx *= -1.0;
    }

    pub fn flip_y(&mut self) {
        self.dy *= -1.0;
    }
}

fn random_color() -> Color {
    let mut rnd = rand::thread_rng();

    let r = rnd.gen_range(0.0..1.0);
    let g = rnd.gen_range(0.0..1.0);
    let b = rnd.gen_range(0.0..1.0);

    Color::rgb(r, g, b)
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_bouncer (
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>
) {
    let texture_handle = asset_server.load("dvd_logo.png");

    let window = windows.get_primary().unwrap();

    // dimensions of our texture
    let size = Vec2::new(173.5, 100.0);

    let ceiling = window.height() / 2. - size.y / 2.;
    let ground = -(window.height() / 2.) + size.y / 2.;

    let wall_left = -(window.width() / 2.) + size.x / 2.;
    let wall_right = window.width() / 2. - size.x / 2.;

    let mut rnd = rand::thread_rng();

    let x_range = std::ops::Range{ start: wall_left, end: wall_right};
    let y_range = std::ops::Range{ start: ground, end: ceiling};

    let sprite_location = Vec3::new(
        rnd.gen_range(x_range),
        rnd.gen_range(y_range),
        0.0,
    );

    let dx = match rnd.gen_bool(0.5) {
        true => 100.0,
        false => -100.0,
    };
    let dy = match rnd.gen_bool(0.5) {
        true => 100.0,
        false => -100.0,
    };

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: random_color(),
            custom_size: Some(size),
            ..Default::default()
        },
        texture: texture_handle,
        transform: Transform {
            translation: sprite_location,
            ..Default::default()
        },
        ..Default::default()
    }).insert(Bouncer { dx, dy });
}

fn handle_input(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::Q) {
        info!("exiting.");
        std::process::exit(0);
    }
}

fn animate_translation(
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Bouncer, &mut Sprite), (With<Sprite>, With<Bouncer>)>,
) {
    let window = windows.get_primary().unwrap();

    let delta = time.delta_seconds();

    for (mut transform, mut bouncer, mut sprite) in query.iter_mut() {
        let size = sprite.custom_size.unwrap();

        let ceiling = window.height() / 2. - size.y / 2.;
        let ground = -(window.height() / 2.) + size.y / 2.;

        let wall_left = -(window.width() / 2.) + size.x / 2.;
        let wall_right = window.width() / 2. - size.x / 2.;

        transform.translation.x += delta * bouncer.dx;
        transform.translation.y += delta * bouncer.dy;

        if transform.translation.y > ceiling {
            transform.translation.y = ceiling;
            bouncer.flip_y();
            sprite.color = random_color();
        } else if transform.translation.y < ground {
            transform.translation.y = ground;
            bouncer.flip_y();
            sprite.color = random_color();
        }

        if transform.translation.x > wall_right {
            transform.translation.x = wall_right;
            bouncer.flip_x();
            sprite.color = random_color();
        } else if transform.translation.x < wall_left {
            transform.translation.x = wall_left;
            bouncer.flip_x();
            sprite.color = random_color();
        }
    }
}
