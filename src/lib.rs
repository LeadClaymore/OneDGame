use raylib::prelude::*;

/// This stores what the coords and the left (l), up (u), right (r), and down (d).
/// False is blocked, and true is clear
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazePiece {
    /// If left is clear
    pub l: bool,
    /// If up is clear
    pub u: bool,
    /// If right is clear
    pub r: bool,
    /// If down is clear
    pub d: bool,
}

// extra functions for the maze piece
impl MazePiece {
    /// creates a new maze piece from the 4 dirrections passed
    #[allow(dead_code)]
    pub fn new(new_l: bool, new_u: bool, new_r: bool, new_d: bool) -> Self {
        MazePiece {
            l: new_l,
            u: new_u,
            r: new_r,
            d: new_d, 
        }
    }

    /// Creates a fully blocked (aka false) MazePiece
    pub fn default() -> Self {
        MazePiece {
            l: false,
            u: false,
            r: false,
            d: false, 
        }
    }

    /// returns true if all sides are blocked (aka false) which is the default state
    pub fn unexplored(&self) -> bool {
        // returns true if all are false
        !(self.l || self.u || self.r || self.d)
    }

    /// sets the dirrection given to true (aka open)
    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    pub fn set_opening(&mut self, dir: u8) {
        match dir {
            0 => self.l = true,
            1 => self.u = true,
            2 => self.r = true,
            3 => self.d = true,
            _ => panic!("Wrong dirrection sent, <{dir}>"),
        }
    }

    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    pub fn set_opposite_opening(&mut self, dir: u8) {
        match dir {
            0 => self.r = true,
            1 => self.d = true,
            2 => self.l = true,
            3 => self.u = true,
            _ => panic!("Wrong opposit dirrection sent"),
        }
    }

    /// returns what color would match this piece
    pub fn get_color(&self) -> Color {
        let mut ret_num: u8 = 0;
        if self.l { ret_num += 1 }
        if self.u { ret_num += 2 }
        if self.r { ret_num += 4 }
        if self.d { ret_num += 8 }
        self.color_list(ret_num)
    }

    // returns the number that would correspond to an tile
    pub fn get_number(&self) -> u8 {
        let mut ret_num: u8 = 0;
        if self.l { ret_num += 1 }
        if self.u { ret_num += 2 }
        if self.r { ret_num += 4 }
        if self.d { ret_num += 8 }
        ret_num
    }

    // creates a MazePiece from a number
    pub fn number_to_piece(mut num: u8) -> MazePiece {
        let mut ret = MazePiece::default();
        if num >= 8 { ret.d = true; num -= 8; }
        if num >= 4 { ret.r = true; num -= 4; }
        if num >= 2 { ret.u = true; num -= 2; }
        if num >= 1 { ret.l = true; }
        ret
    }

    //list for used colors
    pub fn color_list(&self, num: u8) -> Color{
        match num {
            0 => Color::RED,
            1 => Color::BLUE,
            2 => Color::YELLOW,
            3 => Color::ORANGE,
            4 => Color::GREEN,
            5 => Color::PURPLE,
            6 => Color::PLUM,
            7 => Color::TOMATO,
            8 => Color::FUCHSIA,
            9 => Color::AQUAMARINE,
            10 => Color::INDIGO,
            11 => Color::HOTPINK,
            12 => Color::LIGHTBLUE,
            13 => Color::DARKGOLDENROD,
            14 => Color::DARKCYAN,
            15 => Color::LIME,
            _ => panic!("invalid color!"),
        }
    }
}

