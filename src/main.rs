use bevy::prelude::*;

const BOARD_SIZE: usize = 19;
const GRID_SIZE: f32 = 600.0;
const CELL_SIZE: f32 = GRID_SIZE / (BOARD_SIZE - 1) as f32;

#[derive(Component)]
struct Board;

#[derive(Component)]
struct GridLine;

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Create board background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.7, 0.4), // Wood-like color
            custom_size: Some(Vec2::new(GRID_SIZE + 40.0, GRID_SIZE + 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        Board,
    ));

    // Create grid lines
    let line_material = materials.add(Color::BLACK);

    // Horizontal lines
    for i in 0..BOARD_SIZE {
        let y = (i as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * CELL_SIZE;
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(GRID_SIZE, 1.0))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(0.0, y, 0.0),
            GridLine,
        ));
    }

    // Vertical lines
    for i in 0..BOARD_SIZE {
        let x = (i as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * CELL_SIZE;
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, GRID_SIZE))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(x, 0.0, 0.0),
            GridLine,
        ));
    }

    // Add star points (handicap points)
    let star_points = [
        (3, 3), (3, 9), (3, 15),
        (9, 3), (9, 9), (9, 15),
        (15, 3), (15, 9), (15, 15),
    ];

    let star_material = materials.add(Color::BLACK);

    for (row, col) in star_points {
        let x = (col as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * CELL_SIZE;
        let y = (row as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * CELL_SIZE;

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(star_material.clone()),
            Transform::from_xyz(x, y, 1.0),
        ));
    }
}