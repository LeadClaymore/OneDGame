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

/// This represents each square of the maze.
/// Clockwise from the left edge
/// B = blocked,
/// C = cleared.
/// for example BCCC = Only Left Blocked
#[derive(Copy, Clone, Debug, PartialEq)]
enum MazePiece {
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

impl Default for MazePiece {
    fn default() -> Self {
        MazePiece::ERROR
    }
}

// #[derive(Default, Clone)]
// struct Node {
//     left: Option<Box<Node>>,
//     up: Option<Box<Node>>,
//     right: Option<Box<Node>>,
//     down: Option<Box<Node>>,
//     piece: MazePiece,
// }

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
            maze[ii].push(MazePiece::default());
        }
    }

    //for the random path
    let mut rng = rand::thread_rng();
    generate_maze(&maze, 0, 0, MAZESIZE_X - 1, MAZESIZE_Y - 1, &mut rng);
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

fn generate_maze(maze: &Vec<Vec<MazePiece>>, x: u32, y: u32, endx: u32, endy: u32, rng: &mut rand::prelude::ThreadRng) {
    // if bad cords are sent, then it just returns (might do an option later)
    if x < 0 || x > MAZESIZE_X || y < 0 || y > MAZESIZE_Y { return }; 
    let (mut l , mut u, mut r, mut d, mut num) = (false, false, false, false, 0);

    // get current possible routes
    let mut possible_route: Vec<(u32, u32)> = Vec::new();
    if x > 0 && maze[(x - 1) as usize][y as usize] == MazePiece::ERROR { possible_route.push((x - 1, y)) };
    if y > 0 && maze[x as usize][(y - 1) as usize] == MazePiece::ERROR { possible_route.push((x , y - 1)) };
    if x > MAZESIZE_X - 1 && maze[(x + 1) as usize][y as usize] == MazePiece::ERROR { possible_route.push((x + 1, y)) };
    if y > MAZESIZE_Y - 1 && maze[x as usize][(y + 1) as usize] == MazePiece::ERROR { possible_route.push((x, y + 1)) };

    // select route
    if possible_route.len() > 0 {
        let (x2, y2) = possible_route[rng.gen_range(0..possible_route.len())];

    }
    return;
}


/*
TODO:
Change Maze piece into a struct with bool for each dirrection
default of all false
make current maze piece into something to translate shape to color, IDK just comment it out for now
Next using the current generate maze:
    Change the 'get current possible routes' to 1: use the new struct, 2: pass something on to signify that route (some ref to the node)
    Change 'select route' to use this
    Add a update paths to change blocked / unblocked paths
    Think of some way to do regeneration of new paths
*/