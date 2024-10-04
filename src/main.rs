//use raylib::ffi::Rectangle;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
//use raylib::consts::MouseCursor::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const TARGET_FPS: u32 = 240;
const MAZESIZE_X: u32 = 10;
const MAZESIZE_Y: u32 = 10;

//There will be a color for each piece
#[derive(Default, Copy, Clone)]
enum MazePiece {
    #[default]
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

#[derive(Default, Clone)]
struct Node {
    left: Option<Box<Node>>,
    up: Option<Box<Node>>,
    right: Option<Box<Node>>,
    down: Option<Box<Node>>,
    piece: MazePiece,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Also try Costco hotdog soda combo")
        .vsync()
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut maze: Vec<Vec<Node>>;
    for ii in 0..MAZESIZE_X as usize {
        for jj in 0..MAZESIZE_Y as usize {
            println!("[{ii}][{jj}]");
            if ii != 0 {
                //maze[ii][jj].left = Some(maze[ii - 1][jj]);
            }
            if ii + 1 != MAZESIZE_X as usize {

            }
            if jj != 0 {
                
            }
            if jj + 1 != MAZESIZE_Y as usize {

            }
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

fn generate_maze(curr: &Node) {
    
}