use super::renderer::data::Vertex;
use super::tetrimino::{Color, Tetrimino};

#[derive(Clone)]
pub struct Block {
    pub ty: Color,
}
impl Block {
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

        return Some(vec![
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
        ]);
    }
}

pub struct Board {
    width: u32,
    height: u32,
    block_size: f32,
    offset_x: f32,
    offset_y: f32,
    data: Vec<Block>,

    pub current_tetrimino: Option<Tetrimino>,
    pub current_tetrimino_pos_x: u32,
    pub current_tetrimino_pos_y: u32,
    pub current_tetrimino_rotation: u8,
}

impl Board {
    pub fn new(width: u32, height: u32, block_size: f32, offset_x: f32, offset_y: f32) -> Self {
        let mut data = Vec::with_capacity((width * height) as usize);

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
            let block = &self.data[i as usize];
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

    pub fn num_vertices(&self) -> u64 {
        (self.width * self.height * 6).into()
    }

    pub fn get_block_at(&self, x: u32, y: u32) -> Option<&Block> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.data[(y * self.width + x) as usize])
    }

    pub fn tetrimino_valid(&self, tetrimino: &Tetrimino, x: i32, y: i32) -> bool {
        let mut valid = true;

        let len = tetrimino.get_length();
        for i in 0..len * len {
            let block_x = i as i32 % len;
            let block_y = 0 - (i as i32 / len) + len;
            let t_block = tetrimino.get_blocks(self.current_tetrimino_rotation)[i as usize];
            let board_block = self.get_block_at((x + block_x) as u32, (y + block_y) as u32);

            if let Some(b) = board_block {
                if let Color::Empty = b.ty {
                    continue;
                } else {
                    if t_block == 0 {
                        continue;
                    } else {
                        valid = false;
                        break;
                    }
                }
            } else {
                if t_block == 0 {
                    continue;
                } else {
                    valid = false;
                    break;
                }
            }
        }

        valid
    }

    pub fn place_tetrimino(
        &mut self,
        tetrimino: &Tetrimino,
        x: i32,
        y: i32,
        tetrimino_rotation: u8,
    ) -> bool {
        let valid = self.tetrimino_valid(tetrimino, x, y);

        if valid {
            let len = tetrimino.get_length();
            for i in 0..len * len {
                let b_x = i as i32 % len;
                let b_y = 0 - (i as i32 / len) + len;
                let t_block = tetrimino.get_blocks(tetrimino_rotation)[i as usize];

                let combined_x = x as i32 + b_x;
                let combined_y = y as i32 + b_y;
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
        pos_x: u32,
        pos_y: u32,
        rotation: u8,
    ) {
        self.current_tetrimino = Some(*tetrimino);
        self.current_tetrimino_pos_x = pos_x;
        self.current_tetrimino_pos_y = pos_y;
        self.current_tetrimino_rotation = rotation;
    }

    pub fn rotate_current_tetrimino(&mut self, rotation: u8) {
        self.current_tetrimino_rotation = (self.current_tetrimino_rotation + rotation) % 4;
    }

    pub fn move_current_tetrimino(&mut self, x: i32, y: i32) {
        self.current_tetrimino_pos_x = self.current_tetrimino_pos_x + x as u32;
        self.current_tetrimino_pos_y = self.current_tetrimino_pos_y + y as u32;
    }
}
