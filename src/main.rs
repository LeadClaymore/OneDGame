#[allow(special_module_name)]
use serde_derive::Deserialize;
use std::fs;
//use std::process::exit;
use toml;
use ffi::CloseWindow;
use rand::Rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
mod lib;
use crate::lib::MazePiece;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    config: Config,
}
// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
struct Config {
    width: f32,
    height: f32,
    fps: u32
}

fn main() {
    // code on exporting from toml
    // from https://codingpackets.com/blog/rust-load-a-toml-file/
    let filename = "config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // return with defaults
            "[config]
            width = 1000.0
            height = 1000.0
            fps = 60".to_string()
        }
    };

    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // return defaults
            Data {config: Config {width: 1000.0, height: 1000.0, fps: 60}}
        }
    };
    
    let screen_width: f32 = data.config.width;
    let screen_height: f32 = data.config.height;
    let target_fps: u32 = data.config.fps;

    //stores the size of the maze before becoming the imutable
    let mut temp_size: (u32, u32) = (100, 100);

    // this is the blocked and clear colors for the maze for easy changing
    // blocked = .0, and cleared = .1
    let b_c_color = (Color::BLACK, Color::LIGHTGRAY);

    // setup screen, could be changed to also input the size of the next screen
    let (mut r1, thread) = raylib::init()
        .size(800, 600)
        .title("Startup")
        .build();

    // set up window
    while !r1.window_should_close() {
        // changing the maze size if within size
        if r1.is_key_pressed(KEY_A) { if temp_size.0 - 10 > 0                { temp_size.0 -= 10; } }
        if r1.is_key_pressed(KEY_W) { if temp_size.1 - 10 > 0                { temp_size.1 -= 10; } }
        if r1.is_key_pressed(KEY_D) { if temp_size.0 + 10 < i32::MAX as u32  { temp_size.0 += 10; } }
        if r1.is_key_pressed(KEY_S) { if temp_size.1 + 10 < i32::MAX as u32  { temp_size.1 += 10; } }

        // begin drawing
        let mut d: RaylibDrawHandle<'_> = r1.begin_drawing(&thread);
        // background
        d.clear_background(Color::LIGHTGRAY);

        // printing information
        d.draw_text("Controls:", 20, 20, 20, Color::BLACK);
        d.draw_text("WSAD to move", 30, 40, 20, Color::BLACK);
        d.draw_text("'Z' to enter 1D mode", 30, 60, 20, Color::BLACK);
        d.draw_text("'C' to switch to colors only", 30, 80, 20, Color::BLACK);
        d.draw_text("'F' to turn of the FPS counter", 30, 100, 20, Color::BLACK);
        d.draw_text(&format!("Maze size {} by {} (on this screen WASD to change)", temp_size.0, temp_size.1), 30, 120, 20, Color::BLACK);
        d.draw_text("'ESC' to start (in game 'ESC' to exit)", 30, 140, 20, Color::BLACK);

        let temp = MazePiece::default();
        for ii in 0..16 {
            // printing tile colors
            rec_print_color_piece(
                (20 + ii * 25, 160), 
                20, 
                20, 
                temp.color_list(ii as u8), 
                &mut d
            );
            
            //printing the corresponding tiles
            rec_print_piece(
                (20 + ii * 25, 185), 
                ((20.0 / 3.0) as f32).ceil() as i32, 
                ((20.0 / 3.0) as f32).ceil() as i32, 
                MazePiece::number_to_piece(ii as u8), 
                b_c_color, 
                &mut d
            );
        }
        
    }
    // IDK an safe way of doing this, so meh
    unsafe { CloseWindow(); }
    // this sets the given max maze sizes
    let (max_maze_size_x, max_maze_size_y) = temp_size;

    // getting the size of each rectangle
    let (rec_x, rec_y, subrec_x, subrec_y) = (
        (screen_width / max_maze_size_x as f32).ceil(),
        (screen_height / max_maze_size_y as f32).ceil(),
        (screen_width / (max_maze_size_x as f32 * 3.0)).ceil(),
        (screen_height / (max_maze_size_y as f32 * 3.0)).ceil() );

    // this is the start, end, current color
    // start is .0 and is the start of the maze
    // end is .1 and is the end of the maze
    // current is the part of the maze currently at by c
    let s_e_c_color = (Color::GREEN, Color::RED, Color::BLUE);

    // x y coord, but its used alot so its just c
    let mut c: (usize, usize) = (0, 0);

    // settings for the maze
    let (mut fps_on, mut color_mode, mut zoom_mode) = (true, true, true);

    // where the piece should start, used for un-zoomed printing
    let mut piece_start: (i32, i32);

    //initilize the display
    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("One Dimensional Game")
        .vsync()
        .build();
    // set display max FPS
    rl.set_target_fps(target_fps);
    //initilize the maze
    let mut maze: Vec<Vec<MazePiece>> = Vec::new();
    //for each element of the maze put a fully blocked MazePiece
    for ii in 0..max_maze_size_x as usize {
        maze.push(Vec::new());
        for _jj in 0..max_maze_size_y as usize {
            // add new fully blocked piece
            maze[ii].push(MazePiece::default());
        }
    }
    //initilize the random number generator
    let mut rng = rand::thread_rng();
    // populate the maze with a path
    generate_maze_itterative(0, 0, (max_maze_size_x, max_maze_size_y), &mut maze, &mut rng,);

    //drawing the window
    while !rl.window_should_close() {
        // WASD
        if rl.is_key_pressed(KEY_A) { if maze[c.0][c.1].l && c.0 > 0                                { c.0 -= 1; } }
        if rl.is_key_pressed(KEY_W) { if maze[c.0][c.1].u && c.1 > 0                                { c.1 -= 1; } }
        if rl.is_key_pressed(KEY_D) { if maze[c.0][c.1].r && c.0 < (max_maze_size_x - 1) as usize   { c.0 += 1; } }
        if rl.is_key_pressed(KEY_S) { if maze[c.0][c.1].d && c.1 < (max_maze_size_y - 1) as usize   { c.1 += 1; } }

        //arrow keys
        if rl.is_key_pressed(KEY_LEFT)  { if maze[c.0][c.1].l && c.0 > 0                                { c.0 -= 1; } }
        if rl.is_key_pressed(KEY_UP)    { if maze[c.0][c.1].u && c.1 > 0                                { c.1 -= 1; } }
        if rl.is_key_pressed(KEY_RIGHT) { if maze[c.0][c.1].r && c.0 < (max_maze_size_x - 1) as usize   { c.0 += 1; } }
        if rl.is_key_pressed(KEY_DOWN)  { if maze[c.0][c.1].d && c.1 < (max_maze_size_y - 1) as usize   { c.1 += 1; } }

        // turn on and off settings
        if rl.is_key_pressed(KEY_F)     { fps_on = !fps_on; }
        if rl.is_key_pressed(KEY_C)     { color_mode = !color_mode; }
        if rl.is_key_pressed(KEY_Z)     { zoom_mode = !zoom_mode; }

        // begin drawing
        // I think this needs to be initilized here, or something else, but IDK im like 10 hours into raylib
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(b_c_color.1);

        // this draws the rectangles for the maze piece
        // if in zoom mode just show 1 tile with the current coords
        if zoom_mode {
            // if in color mode, print the color for the current tile
            if color_mode {
                rec_print_color_piece((0, 0), screen_width as i32, screen_height as i32, maze[c.0][c.1].get_color(), &mut d);
                print_coords(c, Color::WHITE, &mut d);
            } else {
                //otherwise print the current piece but larger then normal
                rec_print_piece((0, 0), (screen_width / 3.0) as i32, (screen_height / 3.0) as i32, maze[c.0][c.1], b_c_color, &mut d);
                print_coords(c, Color::WHITE, &mut d);
            }
        // if not then show all of them
        } else {
            // for each tile of the maze where (ii, jj) = (x, y)
            for ii in 0..max_maze_size_x as usize {
                for jj in 0..max_maze_size_y as usize {
                    // where this piece should start on the screen
                    piece_start = ((rec_x * ii as f32) as i32, (rec_y * jj as f32) as i32);
                    // if in color mode just print the color, and black if where the player is
                    if color_mode {
                        if c == (ii, jj) {
                            rec_print_color_piece(piece_start, rec_x as i32, rec_y as i32, Color::BLACK,&mut d);
                        } else {
                            rec_print_color_piece(piece_start, rec_x as i32, rec_y as i32, maze[ii][jj].get_color(), &mut d);
                        }
                    // if in normal mode, print the piece, or tint based on 3 colors of (start, end, current) 
                    } else {
                        if c == (ii, jj) {
                            rec_print_piece(piece_start, subrec_x as i32, subrec_y as i32, maze[ii][jj], (b_c_color.0, s_e_c_color.2), &mut d);
                        } else if (ii, jj) == (0, 0) {
                            rec_print_piece(piece_start, subrec_x as i32, subrec_y as i32, maze[ii][jj], (b_c_color.0, s_e_c_color.0), &mut d);
                        } else if (ii, jj) == ((max_maze_size_x - 1) as usize, (max_maze_size_y - 1) as usize) {
                            rec_print_piece(piece_start, subrec_x as i32, subrec_y as i32, maze[ii][jj], (b_c_color.0, s_e_c_color.1), &mut d);
                        } else {
                            rec_print_piece(piece_start, subrec_x as i32, subrec_y as i32, maze[ii][jj], b_c_color, &mut d);
                        }
                    }
                }
            }
        }
        // if you are at the end of the maze then print the win screen
        if c == ((max_maze_size_x - 1) as usize, (max_maze_size_y - 1) as usize) {
            // double for a drop shadow effect
            d.draw_text("You Win!!!", 20, (screen_height / 2.0) as i32, 100, Color::BLACK);
            d.draw_text("You Win!!!", 22, (screen_height / 2.0) as i32 + 2, 100, Color::WHITE);
        }
        // fps counter
        if fps_on { d.draw_fps(10, 10); }
    }
}

/// sets the changes for exploring a tile and leaving a tile
fn set_exploration(maze: &mut Vec<Vec<MazePiece>>, start: (u32, u32), end: (u32, u32), dir: u8) {
    // if the to and from (aka start and end) are the same just change the direction of it
    // this is used for the start and the end of the maze
    if start == end {
        maze[start.0 as usize][start.1 as usize].set_opening(dir);
        return;
    }
    // otherwise change the MazePeice at start to open in the dirrection given
    maze[start.0 as usize][start.1 as usize].set_opening(dir);

    // and change the MazePiece at end to open in the opposite (aka connecting) dirrection
    maze[end.0 as usize][end.1 as usize].set_opposite_opening(dir);
    return;
}

fn generate_maze_itterative<'a>( start_x: u32, start_y: u32, size: (u32, u32),  maze: &'a mut Vec<Vec<MazePiece>>, rng: &'a mut rand::prelude::ThreadRng) {
    // if bad coords are sent, then it just returns (might do an result with an error later)
    if start_x > size.0 || start_y > size.1 { return }; 

    // initilize needed values
    // x y coords of the current MazePiece for the itteration
    let (mut x, mut y);

    // the x y cords of the planned next piece of the maze
    let (mut next_x, mut next_y);

    // the direction between start and next (0 is left, clockwise)
    let mut dir;

    // an vector of possible routes containing an x y coord and a direction
    let mut possible_route: Vec<(u32, u32, u8)> = Vec::new();

    // an vector of x y cords of each previous spot traversed in this tree
    // this is needed so the maze branches when it hits a dead end
    let mut old_path: Vec<(u32, u32)> = Vec::new();

    // begins by pushing the starting value to the old paths
    // this is because I made the algorithm pop the last value each time see step 5 for why
    old_path.push( (start_x, start_y) );

    // itteration until there are no more paths to take
    while old_path.len() > 0 {
        // step 1 add the top of the old_paths to x and y so they are the coords we look at
        // if we found no more paths for the last x and y this would be the node that lead to here
        (x, y) = old_path.pop().unwrap();

        //clear the possible_routes.
        // this is here so it clears any unused paths from the selection
        possible_route.clear();

        // step 2 consider all dirrections from the MazePiece at x and y and push the valid ones to posible_route
        // if the conjoining MazePieces are A: not off the edge, and B: unexplored they are consitered for the next piece
        // I could make this more convoluted but more effecent if I look at the last dir, and dont even consiter that one
        if x != 0 && maze[(x - 1) as usize][y as usize].unexplored()                { possible_route.push((x - 1, y, 0)) };
        if y != 0 && maze[x as usize][(y - 1) as usize].unexplored()                { possible_route.push((x, y - 1, 1)) };
        if x != (size.0 - 1) && maze[(x + 1) as usize][y as usize].unexplored() { possible_route.push((x + 1, y, 2)) };
        if y != (size.1 - 1) && maze[x as usize][(y + 1) as usize].unexplored() { possible_route.push((x, y + 1, 3)) };

        // if there is a valid option select it
        if possible_route.len() > 0 {
            // step 3 we pick a x, y cord and the direction to from the possible_route vec
            // we get it randomly using the rand library this is [rng.gen_range(range)]
            (next_x, next_y, dir) = possible_route[rng.gen_range(0..possible_route.len())];

            // step 4 set the current and next MazePiece to conjoin
            set_exploration(maze, (x, y), (next_x, next_y), dir);

            // step 5 push the current coord in case there are more paths to take later
            old_path.push((x, y));

            // push the coord of the next MazePiece so we can use it for the next itteration
            old_path.push((next_x, next_y));
        }
        // if there were no valid routes, we already poped the current x, y durring step 1, so we simply go on to consiter the last one
    }
    // this creates the opening at the start and the end of the maze
    set_exploration(maze, (start_x, start_y), (start_x, start_y), 0);
    set_exploration(maze, (size.0 - 1, size.1 - 1), (size.0 - 1, size.1 - 1), 2);
}

/// itterate over the maze and print "*" if unexplored, and "#" otherwise
/// for testing
fn _print_maze(maze: &mut Vec<Vec<MazePiece>>) {
    println!("\n------------------------------------");
    for ii in maze.iter() {
        println!();
        for jj in ii.iter() {
            if jj.unexplored() {
                print!("*");
            } else {
                print!("#");
            }
        }
    }
}

fn rec_print_piece(piece_start: (i32, i32), subrec_x: i32, subrec_y: i32, 
        piece: MazePiece, b_c_color: (Color, Color), d: &mut RaylibDrawHandle<'_>) {

    let mut temp_color;

    // ii is the x axis
    for ii in 0..3 {
        // jj is the y axis
        for jj in 0..3 {
            // if ii and jj are % 2 = 0, then its a corner piece
            // because every corner piece is blocked for all valid pieces se set it to the blocked color of b_c_color.0
            if ii % 2 == 0 && jj % 2 == 0 {
                temp_color = b_c_color.0;
            // if (x, y) is (1, 1) then its the middle
            // all valid pieces have a clear middle so this must be set to b_c_color.1
            } else if (ii, jj) == (1, 1) {
                temp_color = b_c_color.1;
            //otherwise this is one of the pieces that depend on the blocked parts
            } else {
                // piece._ is wether the direction (l, u, r, d) is clear
                // (ii, jj) == (_, _) is wether or not that piece is the corrosponding one
                // if both are true then it must be clear
                // if none are true then it must be blocked because it would be clear or corrospond
                // there for it needs to be set to b_c_color.1
                if  piece.l && (ii, jj) == (0, 1) || 
                    piece.u && (ii, jj) == (1, 0) || 
                    piece.d && (ii, jj) == (1, 2) || 
                    piece.r && (ii, jj) == (2, 1) {
                    temp_color = b_c_color.1;
                // otherwise it bust be blocked
                // and therefor set to b_c_color.0
                } else {
                    temp_color = b_c_color.0;
                }
            }
            // this draws the rectangle in the spot designated by ii and jj and the color determined above
            d.draw_rectangle(
                piece_start.0 + (subrec_x * ii), 
                piece_start.1 + (subrec_y * jj), 
                subrec_x, 
                subrec_y, 
                temp_color
            );
        }
    }
    // if you ask yourself why is this not just 9 draw_rectangle functions with some small logic to determin color,
    // a friend of mine grilled me on the size of it, so I shortened it 
    // but as all things are it must be more convoluted and therefor use more space for comments then saved
}

// could delete this for simply the draw rec, but it used to be important and now its not
fn rec_print_color_piece(piece_start: (i32, i32), rec_x: i32, rec_y: i32, 
color: Color, d: &mut RaylibDrawHandle<'_>) {
    d.draw_rectangle(
        piece_start.0, 
        piece_start.1, 
        rec_x, 
        rec_y, 
        color
    );
}

fn print_coords(coords: (usize, usize), color: Color, d: &mut RaylibDrawHandle<'_>) {
    d.draw_text(&format!("(x: {}, y: {})", coords.0, coords.1) , 12, 32, 24, color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexplored() {
        let maze_piece = MazePiece::new(false, false, false, false);
        assert_eq!(maze_piece.unexplored(), true);
        let maze_piece2 = MazePiece::new(true, true, true, true);
        assert_eq!(maze_piece2.unexplored(), false);
        let maze_piece3 = MazePiece::new(true, false, false, false);
        assert_eq!(maze_piece3.unexplored(), false);
    }
}