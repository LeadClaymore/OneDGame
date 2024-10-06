use std::option;

use rand::Rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
//use raylib::consts::MouseCursor::*;

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const TARGET_FPS: u32 = 30;

// 50, 50 allways worked, 60, 60 sometimes worked

const MAZESIZE_X: u32 = 100;
const MAZESIZE_Y: u32 = 100;

/// This stores what the coords and the left (l), up (u), right (r), and down (d).
/// False is blocked, and true is clear
#[derive(Copy, Clone, Debug, PartialEq)]
struct MazePiece {
    /// If left is clear
    l: bool,
    /// If up is clear
    u: bool,
    /// If right is clear
    r: bool,
    /// If down is clear
    d: bool,
}

// extra functions for the maze piece
impl MazePiece {
    /// creates a new maze piece from the 4 dirrections passed
    fn new(new_l: bool, new_u: bool, new_r: bool, new_d: bool) -> Self {
        MazePiece {
            l: new_l,
            u: new_u,
            r: new_r,
            d: new_d, 
        }
    }

    /// Creates a fully blocked (aka false) MazePiece
    fn default() -> Self {
        MazePiece {
            l: false,
            u: false,
            r: false,
            d: false, 
        }
    }

    /// returns true if all sides are blocked (aka false) which is the default state
    fn unexplored(&self) -> bool {
        // returns true if all are false
        !(self.l || self.u || self.r || self.d)
    }

    /// sets the dirrection given to true (aka open)
    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    fn set_opening(&mut self, dir: u8) {
        match dir {
            0 => self.l = true,
            1 => self.u = true,
            2 => self.r = true,
            3 => self.d = true,
            _ => panic!("Wrong dirrection sent, <{dir}>"),
        }
    }

    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    fn set_opposite_opening(&mut self, dir: u8) {
        match dir {
            0 => self.r = true,
            1 => self.d = true,
            2 => self.l = true,
            3 => self.u = true,
            _ => panic!("Wrong opposit dirrection sent"),
        }
    }

    /// returns what color would match this piece
    fn get_color(&self) -> (Color, Color, Color, Color) {
        match (self.l, self.u, self.r, self.d) {
            (true, true, true, true)        => (Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE),
            (false, true, true, true)       => (Color::BLACK, Color::WHITE, Color::WHITE, Color::WHITE),
            (true, false, true, true)       => (Color::WHITE, Color::BLACK, Color::WHITE, Color::WHITE),
            (true, true, false, true)       => (Color::WHITE, Color::WHITE, Color::BLACK, Color::WHITE),
            (true, true, true, false)       => (Color::WHITE, Color::WHITE, Color::WHITE, Color::BLACK),
            (false, false, true, true)      => (Color::BLACK, Color::BLACK, Color::WHITE, Color::WHITE),
            (true, false, false, true)      => (Color::WHITE, Color::BLACK, Color::BLACK, Color::WHITE),
            (true, true, false, false)      => (Color::WHITE, Color::WHITE, Color::BLACK, Color::BLACK),
            (false, true, true, false)      => (Color::BLACK, Color::WHITE, Color::WHITE, Color::BLACK),
            (false, true, false, true)      => (Color::BLACK, Color::WHITE, Color::BLACK, Color::WHITE),
            (true, false, true, false)      => (Color::WHITE, Color::BLACK, Color::WHITE, Color::BLACK),
            (false, false, false, true)     => (Color::BLACK, Color::BLACK, Color::BLACK, Color::WHITE),
            (true, false, false, false)     => (Color::WHITE, Color::BLACK, Color::BLACK, Color::BLACK),
            (false, true, false, false)     => (Color::BLACK, Color::WHITE, Color::BLACK, Color::BLACK),
            (false, false, true, false)     => (Color::BLACK, Color::BLACK, Color::WHITE, Color::BLACK),
            (false, false, false, false)    => (Color::BLACK, Color::BLACK, Color::BLACK, Color::BLACK),
        }
    }
}

fn main() {
    //initilize the display
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Also try Costco hotdog soda combo")
        .vsync()
        .build();

    // set display max FPS
    rl.set_target_fps(TARGET_FPS);

    //initilize the maze
    let mut maze: Vec<Vec<MazePiece>> = Vec::new();

    //for each element of the maze put a fully blocked MazePiece
    for ii in 0..MAZESIZE_X as usize {
        maze.push(Vec::new());
        for _jj in 0..MAZESIZE_Y as usize {
            // add new fully blocked piece
            maze[ii].push(MazePiece::default());
        }
    }

    //initilize the random number generator
    let mut rng = rand::thread_rng();

    // populate the maze with a path
    generate_maze_itterative(
        0,
        0,
        &mut maze,
        &mut rng,
    );

    // make the end thing
    set_exploration(&mut maze, (MAZESIZE_X - 1, MAZESIZE_Y - 1), (MAZESIZE_X - 1, MAZESIZE_Y - 1), 2);

    // getting the size of each rectangle
    let (rec_x, rec_y, subrec_x, subrec_y) = (
        (SCREEN_WIDTH / MAZESIZE_X as f32).ceil(),
        (SCREEN_HEIGHT / MAZESIZE_Y as f32).ceil(),
        (SCREEN_WIDTH / (MAZESIZE_X as f32 * 3.0)).ceil(),
        (SCREEN_HEIGHT / (MAZESIZE_Y as f32 * 3.0)).ceil(),
    );

    //drawing the window
    while !rl.window_should_close() {
        //let dt = rl.get_frame_time();

        //Draw
        //start
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        //ongoing

        // this draws the rectangles for the maze piece
        for ii in 0..MAZESIZE_X as usize {
            for jj in 0..MAZESIZE_Y as usize {
                // top left
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32, 
                    (rec_y * jj as f32) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);
                
                //top right
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 2.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 0.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);
                
                //bottom left
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 2.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 2.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);

                //bottom right
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 0.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 2.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);
                
                //gets the color needed
                let (color_left, color_up, color_right, color_down) = maze[ii][jj].get_color();
                
                // left
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 0.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 1.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_left);
                
                //top (up)
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 1.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 0.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_up);
                
                //right
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 2.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 1.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_right);

                //bottom (down)
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 1.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 2.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_down);

            }
        }
        d.draw_fps(10, 10);
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

fn generate_maze_itterative<'a>( start_x: u32, start_y: u32, maze: &'a mut Vec<Vec<MazePiece>>, rng: &'a mut rand::prelude::ThreadRng) {
    // if bad coords are sent, then it just returns (might do an result with an error later)
    if start_x > MAZESIZE_X || start_y > MAZESIZE_Y { return }; 

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
        if x != (MAZESIZE_X - 1) && maze[(x + 1) as usize][y as usize].unexplored() { possible_route.push((x + 1, y, 2)) };
        if y != (MAZESIZE_Y - 1) && maze[x as usize][(y + 1) as usize].unexplored() { possible_route.push((x, y + 1, 3)) };

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
    set_exploration(maze, (MAZESIZE_X - 1, MAZESIZE_Y - 1), (MAZESIZE_X - 1, MAZESIZE_Y - 1), 2);
}

/// itterate over the maze and print "*" if unexplored, and "#" otherwise
/// for testing
fn print_maze(maze: &mut Vec<Vec<MazePiece>>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexplored() {
        let MP = MazePiece::new(0, 0, false, false, false, false);
        assert_eq!(MP.unexplored(), true);
        let MP2 = MazePiece::new(0, 0, true, true, true, true);
        assert_eq!(MP2.unexplored(), false);
        let MP3 = MazePiece::new(0, 0, true, false, false, false);
        assert_eq!(MP3.unexplored(), false);
        // let MP4 = MazePiece::new(0, 0, false, true, false, false);
        // assert_eq!(MP4.unexplored(), false);
        // let MP5 = MazePiece::new(0, 0, false, false, true, false);
        // assert_eq!(MP5.unexplored(), false);
        // let MP6 = MazePiece::new(0, 0, false, false, false, true);
        // assert_eq!(MP6.unexplored(), false);
    }
}

/* ---------------------------------------------------------------------------------------------------
    TODO traverse the maze:
    make the start block red and track what coord the player is at
    move using wasd or arrow keys
    at each move, move the red dot if the move is possible (use the MazePiece t/f's)
    TODO win screen
    when the player is at the bottom left display end screen and build some functionality to restart
    TODO start screen
    build a screen to display the wsad / arrow key controls in the middle of the screen
    also display other controls, see later
    TODO zoom into 1 square
    have a key switch between full maze and single Piece mode
    in zoomed in mode, you can only see 1 piece
    also test showing the the sourrouning pieces too
    TODO make a color mode
    instead of displaying the 3x3 block for the MazePiece instead display 1 color
    this is mainly for the 1 square mode but you could do it for the full thing later
    TODO setup screen (very large to do)
    On start rather then just making the maze ask for the following things then [Enter] to generate
    1: color only (block switching)
    2: no zoom / unzoom (block switching)
    3: starting zoom / unzoom
    4: starting color / block mode
    
--------------------------------------------------------------------------------------------------- */

/* Old Code
    /// This represents each square of the maze.
    /// Clockwise from the left edge
    /// B = blocked,
    /// C = cleared.
    /// for example BCCC = Only Left Blocked
    enum Configuration {
        ERROR,
        BCCC,
        CBCC,
        CCBC,
        CCCB,
        BBCC,
        CBBC,
        CCBB,
        BCCB,
        BCBC,
        CBCB,
        BBBC,
        CBBB,
        BCBB,
        BBCB,
    }

    fn get_route<'a>(x: u32, y: u32, maze: &'a mut Vec<Vec<MazePiece>>, rng: &'a mut rand::prelude::ThreadRng) 
    -> (Option<(u32, u32, u8)>, &'a mut Vec<Vec<MazePiece>>, &'a mut rand::prelude::ThreadRng) {
    //initilize vec of routes
    let mut possible_route: Vec<(u32, u32, u8)> = Vec::new();

    //add routes to the vec
    if x != 0 && maze[(x - 1) as usize][y as usize].unexplored() { possible_route.push((x - 1, y, 0)) };
    if y != 0 && maze[x as usize][(y - 1) as usize].unexplored() { possible_route.push((x, y - 1, 1)) };
    if x != (MAZESIZE_X - 1) && maze[(x + 1) as usize][y as usize].unexplored() { possible_route.push((x + 1, y, 2)) };
    if y != (MAZESIZE_Y - 1) && maze[x as usize][(y + 1) as usize].unexplored() { possible_route.push((x, y + 1, 3)) };

    //if there are routes then return them otherwise none
    if possible_route.len() != 0 {
        return (Some(possible_route[rng.gen_range(0..possible_route.len())]), maze, rng);
    } else {
        return (None, maze, rng);
    }
}

    /// (x, y) is from |
    /// (args.0.1, args.0.1) is  |
    /// args.0.2 is from > to direction |
    /// args.1 is the maze |
    /// args.2 is the random thead |
    fn generate_maze_recursive<'a>( x: u32, y: u32, args: (Option<( u32, u32, u8 )>, &'a mut Vec<Vec<MazePiece>>, &'a mut rand::prelude::ThreadRng) ) {
        // if bad cords are sent, then it just returns (might do an option later)
        if x > MAZESIZE_X || y > MAZESIZE_Y || args.0.is_none() { return }; 

        // testing print maze
        //print_maze(args.1);

        set_exploration(args.1, (x, y), (args.0.unwrap().0, args.0.unwrap().1), args.0.unwrap().2);

        // repeat this up to 3 times for conjoning paths, 
        generate_maze_recursive(args.0.unwrap().0, args.0.unwrap().1, get_route(args.0.unwrap().0, args.0.unwrap().1, args.1, args.2));
        generate_maze_recursive(args.0.unwrap().0, args.0.unwrap().1, get_route(args.0.unwrap().0, args.0.unwrap().1, args.1, args.2));
        generate_maze_recursive(args.0.unwrap().0, args.0.unwrap().1, get_route(args.0.unwrap().0, args.0.unwrap().1, args.1, args.2));
    }

    impl MazePiece {
        /// returns what enum would match this piece
        fn get_configuration(&self) -> Configuration {
            match (self.l, self.u, self.r, self.d) {
                (false, true, true, true)  => Configuration::BCCC,
                (true, false, true, true) => Configuration::CBCC,
                (true, true, false, true) => Configuration::CCBC,
                (true, true, true, false) => Configuration::CCCB,
                (false, false, true, true) => Configuration::BBCC,
                (true, false, false, true) => Configuration::CBBC,
                (true, true, false, false) => Configuration::CCBB,
                (false, true, true, false) => Configuration::BCCB,
                (false, true, false, true) => Configuration::BCBC,
                (true, false, true, false) => Configuration::CBCB,
                (false, false, false, true) => Configuration::BBBC,
                (true, false, false, false) => Configuration::CBBB,
                (false, true, false, false) => Configuration::BCBB,
                (false, false, true, false) => Configuration::BBCB,
                _ => Configuration::ERROR,
            }
        }

        /// sets the changes for exploring a tile and leaving a tile
        fn set_exploration(&mut self, next_tile: &mut MazePiece, dir: u8) {
            self.set_opening(dir);
            next_tile.set_opposite_opening(dir);
        }
    }

*/