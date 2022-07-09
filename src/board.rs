use super::renderer::data::Vertex;

#[derive(Clone)]
#[allow(dead_code)]
pub enum BlockType {
    Empty,
    LightBlue,
    Blue,
    Red,
    Orange,
    Yellow,
    Purple,
    Green,
}

#[derive(Clone)]
pub struct Block {
    pub ty: BlockType,
}
impl Block {
    pub fn to_vertices(&self, pos_x: f32, pos_y: f32, block_size: f32) -> Option<Vec<Vertex>> {
        if let BlockType::Empty = self.ty {
            return None;
        }

        let min_uv: [f32; 2] = match self.ty {
            BlockType::Blue => [0.0, 0.0],
            BlockType::Red => [0.125, 0.0],
            BlockType::Green => [0.25, 0.0],
            BlockType::Orange => [0.375, 0.0],
            BlockType::Purple => [0.5, 0.0],
            BlockType::Yellow => [0.625, 0.0],
            BlockType::LightBlue => [0.75, 0.0],
            _ => [0.0, 0.0],
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
}

impl Board {
    pub fn new(width: u32, height: u32, block_size: f32, offset_x: f32, offset_y: f32) -> Self {
        let data = vec![
            Block {
                ty: BlockType::Blue,
            };
            (width * height) as usize
        ];

        Self {
            width,
            height,
            data,
            block_size,
            offset_x,
            offset_y,
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
                all_verts.extend_from_slice(&verts);
            }
        }

        all_verts
    }

    pub fn num_vertices(&self) -> u64 {
        (self.width * self.height * 6).into()
    }
}
