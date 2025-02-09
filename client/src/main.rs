use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]

const BOARD_SIZE: usize = 8;
const TILE_SIZE: f32 = 80.0;
const BOARD_OFFSET: f32 = TILE_SIZE * (BOARD_SIZE as f32) / 2.0;

#[derive(Component)]
struct Pawn {
    is_white: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_pawn)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let is_black = (row + col) % 2 == 1;
            let color = if is_black {
                Color::srgb(0.2, 0.2, 0.2)
            } else {
                Color::srgb(0.9, 0.9, 0.9)
            };

            commands.spawn((
                Sprite::from_color(color, Vec2::splat(TILE_SIZE)),
                Transform::from_xyz(
                    col as f32 * TILE_SIZE - BOARD_OFFSET + TILE_SIZE / 2.0,
                    row as f32 * TILE_SIZE - BOARD_OFFSET + TILE_SIZE / 2.0,
                    0.0,
                ),
            ));
        }
    }

    for col in 0..BOARD_SIZE {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), Vec2::splat(TILE_SIZE * 0.6)),
            Transform::from_xyz(
                col as f32 * TILE_SIZE - BOARD_OFFSET + TILE_SIZE / 2.0,
                6 as f32 * TILE_SIZE - BOARD_OFFSET + TILE_SIZE / 2.0,
                1.0,
            ),
            Pawn { is_white: false },
        ));

        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::splat(TILE_SIZE * 0.6)),
            Transform::from_xyz(
                col as f32 * TILE_SIZE - BOARD_OFFSET + TILE_SIZE / 2.0,
                1 as f32 * TILE_SIZE - BOARD_OFFSET + TILE_SIZE / 2.0,
                1.0,
            ),
            Pawn { is_white: true },
        ));
    }
}

fn move_pawn(
    mut query: Query<(&mut Transform, &Pawn)>,
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            Ok(world_pos) => {
                for (mut transform, pawn) in query.iter_mut() {
                    let pawn_pos = transform.translation.truncate();

                    if pawn_pos.distance(world_pos) < TILE_SIZE / 2.0 {
                        let direction = if pawn.is_white { 1.0 } else { -1.0 };
                        transform.translation.y += TILE_SIZE * direction;
                        break;
                    }
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}
