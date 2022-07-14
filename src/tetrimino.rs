#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Color {
    Empty,
    LightBlue,
    Blue,
    Red,
    Orange,
    Yellow,
    Purple,
    Green,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Tetrimino {
    L,
    J,
    O,
    T,
    Z,
    S,
    I,
}

impl Tetrimino {
    // always 4x4
    pub fn get_blocks(&self, rotation: u8) -> Vec<u8> {
        match self {
            Tetrimino::I => {
                return match rotation {
                    1 => vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                    2 => vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
                    3 => vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
                    _ => vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                }
            }
            Tetrimino::J => {
                return match rotation {
                    1 => vec![0, 1, 1, 0, 1, 0, 0, 1, 0],
                    2 => vec![0, 0, 0, 1, 1, 1, 0, 0, 1],
                    3 => vec![0, 1, 0, 0, 1, 0, 1, 1, 0],
                    _ => vec![1, 0, 0, 1, 1, 1, 0, 0, 0],
                }
            }
            Tetrimino::L => {
                return match rotation {
                    1 => vec![0, 1, 0, 0, 1, 0, 0, 1, 1],
                    2 => vec![0, 0, 0, 1, 1, 1, 1, 0, 0],
                    3 => vec![1, 1, 0, 0, 1, 0, 0, 1, 0],
                    _ => vec![0, 0, 1, 1, 1, 1, 0, 0, 0],
                }
            }
            Tetrimino::O => {
                return vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0];
            }
            Tetrimino::S => {
                return match rotation {
                    1 => vec![0, 1, 0, 0, 1, 1, 0, 0, 1],
                    2 => vec![0, 0, 0, 0, 1, 1, 1, 1, 0],
                    3 => vec![1, 0, 0, 1, 1, 0, 0, 1, 0],
                    _ => vec![0, 1, 1, 1, 1, 0, 0, 0, 0],
                };
            }
            Tetrimino::T => {
                return match rotation {
                    1 => vec![0, 1, 0, 0, 1, 1, 0, 1, 0],
                    2 => vec![0, 0, 0, 1, 1, 1, 0, 1, 0],
                    3 => vec![0, 1, 0, 1, 1, 0, 0, 1, 0],
                    _ => vec![0, 1, 0, 1, 1, 1, 0, 0, 0],
                };
            }
            Tetrimino::Z => {
                return match rotation {
                    1 => vec![0, 0, 1, 0, 1, 1, 0, 1, 0],
                    2 => vec![0, 0, 0, 1, 1, 0, 0, 1, 1],
                    3 => vec![0, 1, 0, 1, 1, 0, 1, 0, 0],
                    _ => vec![1, 1, 0, 0, 1, 1, 0, 0, 0],
                };
            }
        }
    }

    pub fn get_length(&self) -> i32 {
        match self {
            Tetrimino::I => 4,
            Tetrimino::J => 3,
            Tetrimino::L => 3,
            Tetrimino::O => 4,
            Tetrimino::S => 3,
            Tetrimino::T => 3,
            Tetrimino::Z => 3,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Tetrimino::I => Color::LightBlue,
            Tetrimino::J => Color::Blue,
            Tetrimino::L => Color::Orange,
            Tetrimino::O => Color::Yellow,
            Tetrimino::S => Color::Green,
            Tetrimino::T => Color::Purple,
            Tetrimino::Z => Color::Red,
        }
    }
}
