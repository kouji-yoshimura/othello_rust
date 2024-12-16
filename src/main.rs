use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use bevy::input::common_conditions::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Component, Default)]
enum WhiteOrBlack {
    #[default]
    White,
    Black,
}

#[derive(Component, Default, Copy, Clone, PartialEq)]
enum CellState {
    #[default]
    Empty,
    White,
    Black,
}

#[derive(Component)]
struct LabelText;

#[derive(Resource, Default)]
struct GameState {
    board: [[CellState; 8]; 8],
    current_turn: WhiteOrBlack,
    white_score: u8,
    black_score: u8,
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            (
                handle_click,
                calculate_score,
                check_game_over,
                render,
            ).chain()
                .run_if(input_just_pressed(MouseButton::Left)),
            (
                handle_press_space,
                calculate_score,
                render,
            ).chain()
                .run_if(input_just_pressed(KeyCode::Space)),
            (handle_press_key_s, render).chain()
                .run_if(input_just_pressed(KeyCode::KeyS)),
        ))
        .run();
}

fn reset_game_state(
    game_state: &mut ResMut<GameState>,
) {
    for row in game_state.board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = CellState::Empty;
        }
    }
    game_state.white_score = 2;
    game_state.black_score = 2;
    game_state.current_turn = WhiteOrBlack::White;
    game_state.board[3][3] = CellState::White;
    game_state.board[4][4] = CellState::White;
    game_state.board[3][4] = CellState::Black;
    game_state.board[4][3] = CellState::Black;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    // Camera
    commands.spawn(Camera2d);

    reset_game_state(&mut game_state);
    for row in 0..8 {
        for column in 0..8 {
            let position = Vec3::new(40.0 * (row as f32 - 4.0), 40.0 * (column as f32 - 4.0), 0.0);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(GREEN))),
                Transform::from_translation(position)
                    .with_scale(Vec3::new(39., 39., 0.)),
            ));

            let cell = game_state.board[row][column];
            let color = match cell {
                CellState::White => Color::from(WHITE),
                CellState::Black => Color::from(BLACK),
                _ => Color::from(GREEN),
            };

            commands.spawn((
                Cell,
                Position {
                    row: row,
                    column: column,
                },
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(position)
                    .with_scale(Vec3::new(34.0, 34.0, 0.0)),
            ));
        }
    }

    commands.spawn((
        Text::new(format!("white: {}", game_state.white_score)),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        LabelText,
        WhiteOrBlack::White,
    ));

    commands.spawn((
        Text::new(format!("black: {}", game_state.black_score)),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            right: Val::Px(5.0),
            ..default()
        },
        LabelText,
        WhiteOrBlack::Black,
    ));
}

fn render(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: ResMut<GameState>,
    mut query: Query<(Entity, &Position), With<Cell>>,
) {
    for (entity, position) in query.iter_mut() {
        let cell = game_state.board[position.row][position.column];
        match cell {
            CellState::White => commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(WHITE)))),
            CellState::Black => commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(BLACK)))),
            _ =>  commands.entity(entity).insert(MeshMaterial2d(materials.add(Color::from(GREEN)))),
        };
    }
}

fn calculate_score(
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut Text, &WhiteOrBlack), With<LabelText>>
) {
    let mut white_score = 0;
    let mut black_score = 0;
    for row in game_state.board.iter_mut() {
        for cell in row.iter_mut() {
            match cell {
                CellState::White => {
                    white_score += 1;
                },
                CellState::Black => {
                    black_score += 1;
                },
                _ => {},
            };
        }
    }
    game_state.white_score = white_score;
    game_state.black_score = black_score;

    for (mut text, white_or_black) in query.iter_mut() {
        match white_or_black {
            WhiteOrBlack::White => {
                text.clear();
                text.push_str(format!("white: {}", game_state.white_score).as_str());
            },
            WhiteOrBlack::Black => {
                text.clear();
                text.push_str(format!("black: {}", game_state.black_score).as_str());
            },
        };
    }
}

fn check_game_over(
    mut game_state: ResMut<GameState>,
) {
    let mut exist_white = false;
    let mut exist_black = false;
    let mut full_filled = true;
    for row in game_state.board.iter_mut() {
        for cell in row.iter_mut() {
            match cell {
                CellState::White => {
                    exist_white = true;
                },
                CellState::Black => {
                    exist_black = true;
                },
                _ => {
                    full_filled = false;
                },
            };
        }
    }
    if !exist_white || !exist_black || full_filled {
        println!("Game Over");
    }
}

fn handle_click(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut game_state: ResMut<GameState>,
) {
    let current = match game_state.current_turn {
        WhiteOrBlack::White => CellState::White,
        WhiteOrBlack::Black => CellState::Black,
    };
    let target = match game_state.current_turn {
        WhiteOrBlack::White => CellState::Black,
        WhiteOrBlack::Black => CellState::White,
    };

    if let Some(position) = q_windows.single().cursor_position() {
        if position.x < 460. || position.x >= 780. || position.y < 220. || position.y >= 540. {
            return
        }

        let size = q_windows.single().size();
        let row: usize = ((position.x - (size.x / 2.) + 20.) / 40. + 4.0) as usize;
        let column: usize = (8.0 - ((position.y - (size.y / 2.) - 20.) / 40. + 4.0)) as usize;

        if game_state.board[row][column] != CellState::Empty {
            return
        }

        let mut found = false;
        for seek_row_dir in -1..=1 {
            for seek_column_dir in -1..=1 {
                if seek_row_dir == 0 && seek_column_dir == 0 {
                    continue
                }
                let mut seek_row = row as isize;
                let mut seek_column = column as isize;
                let mut lock_on = false;
                let mut ignite = false;

                while seek_row >= 0 && seek_row < 8 && seek_column >= 0 && seek_column < 8 {
                    seek_row += seek_row_dir;
                    seek_column += seek_column_dir;
                    if seek_row < 0 || seek_row >= 8 || seek_column < 0 || seek_column >= 8 {
                        break
                    }

                    if !lock_on && game_state.board[seek_row as usize][seek_column as usize] != target {
                        break
                    }
                    lock_on = true;

                    if lock_on && game_state.board[seek_row as usize][seek_column as usize] == current {
                        ignite = true;
                        found = true;
                        break
                    }
                }

                if ignite {
                    while seek_row != row as isize || seek_column != column as isize {
                        game_state.board[seek_row as usize][seek_column as usize] = current;
                        seek_row -= seek_row_dir;
                        seek_column -= seek_column_dir;
                    }
                }
            }
        }
        if !found {
            return
        }
        game_state.board[row][column] = current;

        game_state.current_turn = match game_state.current_turn {
            WhiteOrBlack::White => WhiteOrBlack::Black,
            WhiteOrBlack::Black => WhiteOrBlack::White,
        };
    }
}

fn handle_press_space(
    mut game_state: ResMut<GameState>,
) {
    reset_game_state(&mut game_state);
}

fn handle_press_key_s(
    mut game_state: ResMut<GameState>,
) {
    game_state.current_turn = match game_state.current_turn {
        WhiteOrBlack::White => WhiteOrBlack::Black,
        WhiteOrBlack::Black => WhiteOrBlack::White,
    };
}
