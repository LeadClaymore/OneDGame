use rand::Rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
//use raylib::consts::MouseCursor::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const TARGET_FPS: u32 = 240;
const MAZESIZE_X: u32 = 10;
const MAZESIZE_Y: u32 = 10;

// #[derive(Default, Clone)]
// struct Node {
//     left: Option<Box<Node>>,
//     up: Option<Box<Node>>,
//     right: Option<Box<Node>>,
//     down: Option<Box<Node>>,
//     piece: MazePiece,
// }

/// This represents each square of the maze.
/// Clockwise from the left edge
/// B = blocked,
/// C = cleared.
/// for example BCCC = Only Left Blocked
    // ERROR,
    // BCCC,
    // CBCC,
    // CCBC,
    // CCCB,
    // BBCC,
    // CBBC,
    // CCBB,
    // BCCB,
    // BCBC,
    // CBCB,
    // BBBC,
    // CBBB,
    // BCBB,
    // BBCB,

/// This stores what the coords and 
#[derive(Copy, Clone, Debug, PartialEq)]
struct MazePiece {
    x: u32,
    y: u32,
    l: bool,
    u: bool,
    r: bool,
    d: bool,
}

impl MazePiece {
    fn new(new_x: u32, new_y: u32) -> Self {
        MazePiece {
            x: new_x,
            y: new_y,
            l: false,
            u: false,
            r: false,
            d: false, 
        }
    }

    fn default() -> Self {
        MazePiece {
            x: 0,
            y: 0,
            l: false,
            u: false,
            r: false,
            d: false, 
        }
    }

    /// returns true if all sides are blocked (false)
    fn unexplored(&self) -> bool {
        !(self.l || self.u || self.r || self.d)
    }

    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    fn set_opening(&mut self, dir: u8) {
        match dir {
            0 => self.l = true,
            1 => self.u = true,
            2 => self.r = true,
            3 => self.d = true,
            _ => panic!("Wrong dirrection sent"),
        }
    }

    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    fn set_oposite_opening(&mut self, dir: u8) {
        match dir {
            0 => self.r = true,
            1 => self.d = true,
            2 => self.l = true,
            3 => self.u = true,
            _ => panic!("Wrong opposit dirrection sent"),
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Also try Costco hotdog soda combo")
        .vsync()
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut maze: Vec<Vec<MazePiece>> = Vec::new();
    for ii in 0..MAZESIZE_X as usize {
        maze.push(Vec::new());
        for jj in 0..MAZESIZE_Y as usize {
            println!("[{ii}][{jj}]");
            maze[ii].push(MazePiece::new(ii as u32, jj as u32));
        }
    }

    //for the random path
    let mut rng = rand::thread_rng();
    generate_maze(&mut maze, 0, 0, MAZESIZE_X - 1, MAZESIZE_Y - 1, &mut rng);

    for ii in 0..MAZESIZE_X as usize {
        for jj in 0..MAZESIZE_Y as usize {
            if maze[ii][jj].
            println!("");
        }
    }

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        //Draw
        //start
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        //ongoing

        d.draw_fps(10, 10);
    }
}

fn generate_maze(maze: &mut Vec<Vec<MazePiece>>, x: u32, y: u32, endx: u32, endy: u32, rng: &mut rand::prelude::ThreadRng) {
    // if bad cords are sent, then it just returns (might do an option later)
    if x < 0 || x > MAZESIZE_X || y < 0 || y > MAZESIZE_Y { return }; 
    //let (mut l , mut u, mut r, mut d, mut num) = (false, false, false, false, 0);

    // get current possible routes
    let mut possible_route: Vec<(u32, u32, u8)> = Vec::new();
    if x > 0 && maze[(x - 1) as usize][y as usize].unexplored() { possible_route.push((x - 1, y, 0)) };
    if y > 0 && maze[x as usize][(y - 1) as usize].unexplored() { possible_route.push((x, y - 1, 1)) };
    if x > MAZESIZE_X - 1 && maze[(x + 1) as usize][y as usize].unexplored() { possible_route.push((x + 1, y, 2)) };
    if y > MAZESIZE_Y - 1 && maze[x as usize][(y + 1) as usize].unexplored() { possible_route.push((x, y + 1, 3)) };

    // select route
    if possible_route.len() > 0 {
        let (x2, y2, dir) = possible_route[rng.gen_range(0..possible_route.len())];
        maze[x2 as usize][y2 as usize].set_oposite_opening(dir);
        maze[x as usize][y as usize].set_opening(dir);
    }
    return;
}


/*
TODO:
Change Maze piece into a struct with bool for each dirrection //DONE
default of all false //DONE
make current maze piece into something to translate shape to color, IDK just comment it out for now //DONE
Next using the current generate maze:
    Change the 'get current possible routes' to 1: use the new struct, 2: pass something on to signify that route (some ref to the node)
    Change 'select route' to use this
    Add a update paths to change blocked / unblocked paths
    Think of some way to do regeneration of new paths
*/