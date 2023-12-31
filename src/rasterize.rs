use crate::math::{Vec2, Vec3, Vec4};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
    pub z_buffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height * 4],
            z_buffer: vec![0.0; width * height * 4],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.z_buffer.fill(1.0);
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            let index = (x as usize + y as usize * self.width) * 4;
            self.buffer[index] = color[2];
            self.buffer[index + 1] = color[1];
            self.buffer[index + 2] = color[0];
        }
    }

    pub fn draw_triangle<F>(&mut self, verts: [Vec4; 3], shader: F)
    where
        F: Fn(Vec3) -> Vec3,
    {
        let (mut min_x, mut min_y) = (self.width as i32, self.height as i32);
        let (mut max_x, mut max_y) = (0, 0);
        for vert in verts {
            min_x = min_x.min(vert.x as i32);
            min_y = min_y.min(vert.y as i32);
            max_x = max_x.max(vert.x as i32);
            max_y = max_y.max(vert.y as i32);
        }
        min_x = (min_x - 1).clamp(0, self.width as i32);
        min_y = (min_y - 1).clamp(0, self.height as i32);
        max_x = (max_x + 1).clamp(0, self.width as i32);
        max_y = (max_y + 1).clamp(0, self.height as i32);

        let (ooza, oozb, oozc) = (1.0 / verts[0].z, 1.0 / verts[1].z, 1.0 / verts[2].z);
        let (oowa, oowb, oowc) = (1.0 / verts[0].w, 1.0 / verts[1].w, 1.0 / verts[2].w);
        let (a, b, c) = (verts[0].xy(), verts[1].xy(), verts[2].xy());
        let invarea = 1.0 / (b - a).perp().dot(c - a);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vec2::new(x as f32, y as f32);
                let bary = Vec3::new(
                    (b - p).perp().dot(c - p) * invarea,
                    (c - p).perp().dot(a - p) * invarea,
                    (a - p).perp().dot(b - p) * invarea,
                );
                if bary[0] >= 0.0 && bary[1] >= 0.0 && bary[2] >= 0.0 {
                    let z = 1.0 / (bary[0] * ooza + bary[1] * oozb + bary[2] * oozc);
                    let w = 1.0 / (bary[0] * oowa + bary[1] * oowb + bary[2] * oowc);
                    if z < self.z_buffer[x as usize + y as usize * self.width] {
                        self.z_buffer[x as usize + y as usize * self.width] = z;

                        // correct for perspective
                        // TODO understand this better
                        // TODO why does doing this break menus?
                        let bary = Vec3::new(bary[0] * oowa, bary[1] * oowb, bary[2] * oowc) * w;

                        let color = shader(bary) * 255.0;
                        self.draw_pixel(x, y, [color[0] as u8, color[1] as u8, color[2] as u8]);
                    }
                }
            }
        }
    }
}
