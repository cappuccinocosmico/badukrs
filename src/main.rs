use bevy::prelude::*;

mod game;
mod random_bot;
mod rendering;

use game::{BadukClassical, BadukMove, Player, Point, StatelessGame};
use random_bot::RandomBot;
use rendering::{setup, BOARD_SIZE, CELL_SIZE};

use crate::random_bot::GameBot;

#[derive(Resource)]
struct GameState {
    game: BadukClassical<19>,
    white_bot: RandomBot<BadukClassical<19>>,
}

#[derive(Component)]
struct Stone {
    row: usize,
    col: usize,
    player: Player,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Baduk/Go Game".to_string(),
                resolution: (800.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState {
            game: BadukClassical::new(),
            white_bot: RandomBot::new(),
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_input, update_board_display, handle_bot_turn),
        )
        .run();
}

fn handle_input(
    mut game_state: ResMut<GameState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if game_state.game.turn != Player::Black || game_state.game.is_game_over() {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            // Convert world position to board coordinates
            let board_x = world_position.x + (BOARD_SIZE - 1) as f32 * CELL_SIZE / 2.0;
            let board_y = world_position.y + (BOARD_SIZE - 1) as f32 * CELL_SIZE / 2.0;

            let col = (board_x / CELL_SIZE).round() as usize;
            let row = (board_y / CELL_SIZE).round() as usize;

            if row < BOARD_SIZE && col < BOARD_SIZE {
                let move_attempt = BadukMove::Play {
                    coordinates: (row, col),
                };
                if game_state.game.is_legal(&move_attempt) {
                    let _ = game_state.game.make_move(move_attempt);
                }
            }
        }
    }
}

fn handle_bot_turn(mut game_state: ResMut<GameState>) {
    if game_state.game.turn == Player::White && !game_state.game.is_game_over() {
        if let Ok(bot_move) = game_state.white_bot.select_move(&game_state.game) {
            let _ = game_state.game.make_move(bot_move);
        }
    }
}

fn update_board_display(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
    stones_query: Query<Entity, With<Stone>>,
) {
    if !game_state.is_changed() {
        return;
    }

    // Remove all existing stones
    for entity in stones_query.iter() {
        commands.entity(entity).despawn();
    }

    // Add stones based on current game state
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(Point::Stone(player)) = game_state.game.board.get_point(row, col) {
                let x = (col as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * CELL_SIZE;
                let y = (row as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * CELL_SIZE;

                let color = match player {
                    Player::Black => Color::BLACK,
                    Player::White => Color::WHITE,
                };

                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(CELL_SIZE * 0.4))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(x, y, 2.0),
                    Stone { row, col, player },
                ));
            }
        }
    }
}
