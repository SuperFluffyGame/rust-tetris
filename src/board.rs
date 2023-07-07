use super::renderer::data::Vertex;
use super::tetrimino::{Color, Tetrimino};

#[derive(Clone)]
pub struct Block {
    pub ty: Color,
}
impl Block {
    pub fn is_empty(&self) -> bool {
        matches!(self.ty, Color::Empty)
    }
    pub fn to_vertices(&self, pos_x: f32, pos_y: f32, block_size: f32) -> Option<Vec<Vertex>> {
        let min_uv: [f32; 2] = match self.ty {
            Color::Blue => [0.0, 0.0],
            Color::Red => [0.125, 0.0],
            Color::Green => [0.25, 0.0],
            Color::Orange => [0.375, 0.0],
            Color::Purple => [0.5, 0.0],
            Color::Yellow => [0.625, 0.0],
            Color::LightBlue => [0.75, 0.0],
            _ => [0.875, 0.0],
        };

        let min_pos = [pos_x, pos_y, 0.0];

        Some(vec![
            Vertex {
                position: min_pos,
                uv: min_uv,
            },
            Vertex {
                position: [min_pos[0] + block_size, min_pos[1], min_pos[2]],
                uv: [min_uv[0] + 0.125, min_uv[1]],
            },
            Vertex {
                position: [min_pos[0] + block_size, min_pos[1] + block_size, min_pos[2]],
                uv: [min_uv[0] + 0.125, min_uv[1] + 1.0],
            },
            Vertex {
                position: min_pos,
                uv: min_uv,
            },
            Vertex {
                position: [min_pos[0], min_pos[1] + block_size, min_pos[2]],
                uv: [min_uv[0], min_uv[1] + 1.0],
            },
            Vertex {
                position: [min_pos[0] + block_size, min_pos[1] + block_size, min_pos[2]],
                uv: [min_uv[0] + 0.125, min_uv[1] + 1.0],
            },
        ])
    }
}

pub struct Board {
    width: usize,
    height: usize,
    block_size: f32,
    offset_x: f32,
    offset_y: f32,
    data: Vec<Block>,

    pub current_tetrimino: Option<Tetrimino>,
    pub current_tetrimino_pos_x: i32,
    pub current_tetrimino_pos_y: i32,
    pub current_tetrimino_rotation: i8,
}

impl Board {
    pub fn new(width: usize, height: usize, block_size: f32, offset_x: f32, offset_y: f32) -> Self {
        let mut data = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            data.push(Block { ty: Color::Empty });
        }

        Self {
            width,
            height,
            data,
            block_size,
            offset_x,
            offset_y,
            current_tetrimino: None,
            current_tetrimino_pos_x: 0,
            current_tetrimino_pos_y: 0,
            current_tetrimino_rotation: 0,
        }
    }

    pub fn to_vertices(&self) -> Vec<Vertex> {
        let mut all_verts = Vec::new();

        for i in 0..self.width * self.height {
            let x = (i % self.width) as f32 * self.block_size;
            let y = (i / self.width) as f32 * self.block_size;
            let block = &self.data[i];
            if let Some(verts) =
                block.to_vertices(x - self.offset_x, y - self.offset_y, self.block_size)
            {
                all_verts.extend(&verts);
            }
        }

        if let Some(tetrimino) = &self.current_tetrimino {
            let len = tetrimino.get_length();
            for i in 0..len * len {
                let x = (i % len) as f32 + self.current_tetrimino_pos_x as f32;
                let y = (0 - (i / len) + len) as f32 + self.current_tetrimino_pos_y as f32;
                let is_block = tetrimino.get_blocks(self.current_tetrimino_rotation)[i as usize];
                if is_block == 1 {
                    let block = Block {
                        ty: tetrimino.get_color(),
                    };
                    let verts = block.to_vertices(
                        x * self.block_size - self.offset_x,
                        y * self.block_size - self.offset_y,
                        self.block_size,
                    );
                    if let Some(verts) = verts {
                        all_verts.extend(verts);
                    }
                }
            }
        }

        all_verts
    }

    pub fn num_vertices(&self) -> usize {
        self.width * self.height * 6
    }

    pub fn get_block_at(&self, x: usize, y: usize) -> Option<&Block> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.data[y * self.width + x])
    }

    pub fn tetrimino_valid(&self, tetrimino: &Tetrimino, x: i32, y: i32, rot: i8) -> bool {
        let mut valid = true;

        let len = tetrimino.get_length();
        for i in 0..len * len {
            let block_x = i % len;
            let block_y = 0 - (i / len) + len;
            let t_block = tetrimino.get_blocks(rot)[i as usize];
            let board_block = self.get_block_at((x + block_x) as usize, (y + block_y) as usize);

            if let Some(b) = board_block {
                if let Color::Empty = b.ty {
                    continue;
                } else if t_block == 0 {
                    continue;
                } else {
                    valid = false;
                    break;
                }
            } else if t_block == 0 {
                continue;
            } else {
                valid = false;
                break;
            }
        }

        valid
    }

    pub fn place_tetrimino(
        &mut self,
        tetrimino: &Tetrimino,
        x: i32,
        y: i32,
        tetrimino_rotation: i8,
    ) -> bool {
        let valid = self.tetrimino_valid(tetrimino, x, y, tetrimino_rotation);

        if valid {
            let len = tetrimino.get_length();
            for i in 0..len * len {
                let b_x = i % len;
                let b_y = 0 - (i / len) + len;
                let t_block = tetrimino.get_blocks(tetrimino_rotation)[i as usize];

                let combined_x = x + b_x;
                let combined_y = y + b_y;
                if t_block == 1 {
                    self.data[(combined_y * self.width as i32 + combined_x) as usize] = Block {
                        ty: tetrimino.get_color(),
                    };
                }
            }
        }

        valid
    }

    pub fn set_current_tetrimino(
        &mut self,
        tetrimino: &Tetrimino,
        pos_x: i32,
        pos_y: i32,
        rotation: i8,
    ) {
        self.current_tetrimino = Some(*tetrimino);
        self.current_tetrimino_pos_x = pos_x;
        self.current_tetrimino_pos_y = pos_y;
        self.current_tetrimino_rotation = rotation;
    }

    pub fn rotate_left(&mut self) -> bool {
        let mut next_rotation = self.current_tetrimino_rotation - 1;
        if next_rotation < 0 {
            next_rotation = 3;
        }

        let valid = self.current_tetrimino_valid(0, 0, next_rotation);

        println!(
            "{:?}, {:?}",
            valid,
            (self.current_tetrimino_rotation - 1) % 4
        );
        if valid {
            self.current_tetrimino_rotation = next_rotation;
        }
        valid
    }
    pub fn rotate_right(&mut self) -> bool {
        let mut next_rotation = self.current_tetrimino_rotation + 1;
        if next_rotation > 3 {
            next_rotation = 0;
        }

        let valid = self.current_tetrimino_valid(0, 0, next_rotation);
        if valid {
            self.current_tetrimino_rotation = next_rotation
        }
        valid
    }

    pub fn place_current_tetrimino(&mut self) -> bool {
        let placed = self.place_tetrimino(
            &self.current_tetrimino.unwrap(),
            self.current_tetrimino_pos_x,
            self.current_tetrimino_pos_y,
            self.current_tetrimino_rotation,
        );
        if !placed {
            panic!("Tetrimino not placed!")
        }

        self.current_tetrimino = Some(Tetrimino::random());
        self.current_tetrimino_pos_x = 3;
        self.current_tetrimino_pos_y = 15;

        placed
    }
    pub fn current_tetrimino_valid(&self, off_x: i32, off_y: i32, rot: i8) -> bool {
        self.current_tetrimino.is_some()
            && self.tetrimino_valid(
                &self.current_tetrimino.unwrap(),
                self.current_tetrimino_pos_x + off_x,
                self.current_tetrimino_pos_y + off_y,
                rot,
            )
    }

    pub fn move_left(&mut self) -> bool {
        let valid = self.current_tetrimino_valid(-1, 0, self.current_tetrimino_rotation);
        if valid {
            self.current_tetrimino_pos_x -= 1;
        }
        valid
    }
    pub fn move_right(&mut self) -> bool {
        let valid = self.current_tetrimino_valid(1, 0, self.current_tetrimino_rotation);
        if valid {
            self.current_tetrimino_pos_x += 1;
        }
        valid
    }
    pub fn move_down(&mut self) -> bool {
        let valid = self.current_tetrimino_valid(0, -1, self.current_tetrimino_rotation);
        if valid {
            self.current_tetrimino_pos_y -= 1;
        } else {
            self.place_current_tetrimino();
            self.check_and_delete_rows();
        }

        valid
    }

    pub fn get_row(&self, row: usize) -> &[Block] {
        if row >= self.height {
            panic!("Row is greater than height.");
        }
        &self.data[(row * self.width)..(row * self.width + self.width)]
    }
    pub fn is_row_full(row: &[Block]) -> bool {
        row.iter().all(|block| !block.is_empty())
    }
    pub fn get_full_rows(&mut self) -> Vec<usize> {
        let mut full_rows = Vec::new();
        for i in 0..self.height {
            let row = self.get_row(i);
            let full = Self::is_row_full(row);
            if full {
                full_rows.push(i);
            }
        }
        full_rows
    }

    pub fn delete_row(&mut self, row: usize) {
        self.data
            .drain((row * self.width)..(row * self.width + self.width));

        let mut new_data = Vec::new();
        for _ in 0..self.width {
            new_data.push(Block { ty: Color::Empty })
        }

        self.data.append(&mut new_data);
    }

    pub fn check_and_delete_rows(&mut self) {
        let mut full_rows = self.get_full_rows();
        full_rows.sort();
        full_rows.reverse();

        for full_row in full_rows {
            self.delete_row(full_row);
        }
    }

    pub fn update(&mut self) {
        let start = std::time::Instant::now();
        self.move_down();
        let end = std::time::Instant::now();

        let dur = end - start;

        println!("{:?}", dur.as_micros());
    }
}
