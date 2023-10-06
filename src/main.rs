/**
 * --Adapted from--
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Edited: Isak Larsson <isaklar@kth.se>
 * Last updated: 2022-09-28
 * URL: https://github.com/IndaPlus23/Turbofishes/tree/master/chess-gui/chess-gui-templates/ggez-template
 */
use sagakar_chess_lib::{Color, Game, Piece};

use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use std::{collections::HashMap, env, path};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: graphics::Color =
    graphics::Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);
const GREEN: graphics::Color = 
    graphics::Color::new(85.0 / 255.0, 205.0 / 255.0 , 70.0 / 255.0, 0.5);

/// GUI logic and event implementation structure.
struct AppState {
    sprites: HashMap<(Color, Piece), graphics::Image>,
    // Example board representation.
    board: [[Option<(Color, Piece)>; 8]; 8],
    // Imported game representation.
    game: Game,
    move_start: Option<(u8, u8)>,
    possible_moves: Option<Vec<String>>
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut state = AppState {
            sprites: AppState::load_sprites(ctx),
            game: Game::new(),
            board: [[None, None, None, None, None, None, None, None]; 8],
            move_start: None,
            possible_moves: None,
        };
        state.board_from_game();
        Ok(state)
    }
    #[rustfmt::skip] // Skips formatting on this function (not recommended)
                     /// Loads chess piese images into hashmap, for ease of use.
    fn load_sprites(ctx: &mut Context) -> HashMap<(Color, Piece), graphics::Image> {
         [
             ((Color::Black, Piece::King), "/black_king.png".to_string()),
             ((Color::Black, Piece::Queen), "/black_queen.png".to_string()),
             ((Color::Black, Piece::Rook), "/black_rook.png".to_string()),
             ((Color::Black, Piece::Pawn), "/black_pawn.png".to_string()),
             ((Color::Black, Piece::Bishop), "/black_bishop.png".to_string()),
             ((Color::Black, Piece::Knight), "/black_knight.png".to_string()),
             ((Color::White, Piece::King), "/white_king.png".to_string()),
             ((Color::White, Piece::Queen), "/white_queen.png".to_string()),
             ((Color::White, Piece::Rook), "/white_rook.png".to_string()),
             ((Color::White, Piece::Pawn), "/white_pawn.png".to_string()),
             ((Color::White, Piece::Bishop), "/white_bishop.png".to_string()),
             ((Color::White, Piece::Knight), "/white_knight.png".to_string())
         ]
             .iter()
             .map(|(piece, path)| {
                 (*piece, graphics::Image::new(ctx, path).unwrap())
             })
             .collect::<HashMap<(Color, Piece), graphics::Image>>()
     }

    fn board_from_game(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                let square = self.game.get_board()[y][x];
                if square != None {
                    self.board[y][x] = Some((self.game.get_color_at(x, y).unwrap(), square.unwrap()));
                }
                else {
                    self.board[y][x] = None;
                }
            }
        }
    }
}

// This is where we implement the functions that ggez requires to function
impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    /// It won't be necessary to touch this unless you are implementing something that's not triggered by the user, like a clock
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background Color
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state()))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                text_dimensions.w as f32 + 16.0,
                text_dimensions.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default())
            .expect("Failed to draw background.");

        // draw grid
        for row in 0..8 {
            for col in 0..8 {
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        col * GRID_CELL_SIZE.0 as i32,
                        row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match (col + 1) % 2 {
                        0 => {
                            if row % 2 == 0 {
                                WHITE
                            } else {
                                BLACK
                            }
                        }
                        _ => {
                            if row % 2 == 0 {
                                BLACK
                            } else {
                                WHITE
                            }
                        }
                    },
                )
                .expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");

                // draw piece
                if let Some(piece) = self.board[row as usize][col as usize] {
                    graphics::draw(
                        ctx,
                        self.sprites.get(&piece).unwrap(),
                        graphics::DrawParam::default()
                            .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                            .dest([
                                col as f32 * GRID_CELL_SIZE.0 as f32,
                                row as f32 * GRID_CELL_SIZE.1 as f32,
                            ]),
                    )
                    .expect("Failed to draw piece.");
                }
            }
        }

        // Highlight possible moves
        if self.possible_moves != None {
            for tile in self.possible_moves.as_ref().unwrap() {
                let center_x = chess_to_numerical(tile).0 as f32 * GRID_CELL_SIZE.0 as f32 + GRID_CELL_SIZE.0 as f32 / 2.0;
                let center_y = chess_to_numerical(tile).1 as f32 * GRID_CELL_SIZE.0 as f32 + GRID_CELL_SIZE.0 as f32 / 2.0;
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [center_x, center_y],
                    GRID_CELL_SIZE.0 as f32 / 2.0,
                    0.1,
                    GREEN
                )
                .expect("Failed to create circle");
                graphics::draw(ctx, &circle, graphics::DrawParam::default())
                .expect("Failed to draw circle");
            }
        }

        // draw text with dark gray Coloring and center position
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    // * Now THIS is what it's all about!!!
    /// Update game on mouse click
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button != event::MouseButton::Left {
            return;
        }
        // cell_x, cell_y are guaranteed to be within [0, 7], so they can safely be cast to a u8
        let cell_x = (x / f32::from(GRID_CELL_SIZE.0)).floor() as u8;
        let cell_y = (y / f32::from(GRID_CELL_SIZE.1)).floor() as u8;
        let string_coordinates = numerical_to_chess(cell_x, cell_y);
        
        if self.move_start == None {
            let possible_moves = self.game.get_possible_moves(&string_coordinates);
            // If there are no moves, return
            if possible_moves == None {
                return
            }
            let cell_color = self.board[cell_y as usize][cell_x as usize].unwrap().0;
            // If attempring to move out of turn, return
            if cell_color != self.game.get_player() {
                return
            }
            self.move_start = Some((cell_x, cell_y));
            self.possible_moves = possible_moves;

        }
        else {
            // Reset move_start, possible_moves and return if the move is illegal
            if self.possible_moves.as_ref().unwrap().contains(&string_coordinates) {
                let start = self.move_start.as_ref().unwrap();
                if self.game.make_move(&numerical_to_chess(start.0, start.1), &string_coordinates) != None {
                    self.board_from_game();
                }
            }
            self.move_start = None;
            self.possible_moves = None;
            println!("{:?}", self.board)
        }
    }
}

// Converts numerical board coordinates to chess coordinates
fn numerical_to_chess (x : u8, y : u8) -> (String) {
    let chess_x = ((x + ('A' as u8)) as char).to_string();
    let chess_y = (8 - y).to_string();
    return chess_x + &chess_y;

}

fn chess_to_numerical (coordinates : &str) -> ((u8, u8)) {
    let coordinates = coordinates.chars().collect::<Vec<char>>();
    let x = (coordinates[0].clone() as u8) - ('A' as u8);
    let y = 8 - ((coordinates[1].clone() as u8) - ('0' as u8));
    return (x, y);
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "viola")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Schack") // Set window title "Schack"
                .icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        );
    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}
