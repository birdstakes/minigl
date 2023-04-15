pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height * 4],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32) {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            let index = (x as usize + y as usize * self.width) * 4;
            self.buffer[index] = 0xe0;
            self.buffer[index + 1] = 0xe0;
            self.buffer[index + 2] = 0xe0;
        }
    }

    pub fn draw_line(&mut self, mut sx: f32, mut sy: f32, mut ex: f32, mut ey: f32) {
        let (dx, dy) = (ex - sx, ey - sy);
        if dx.abs() > dy.abs() {
            if dx < 0.0 {
                std::mem::swap(&mut sx, &mut ex);
                std::mem::swap(&mut sy, &mut ey);
            }
            let mut y = sy;
            for x in sx as i32..ex as i32 {
                self.draw_pixel(x, y as i32);
                y += (ey - sy) / (ex - sx);
            }
        } else {
            if dy < 0.0 {
                std::mem::swap(&mut sx, &mut ex);
                std::mem::swap(&mut sy, &mut ey);
            }
            let mut x = sx;
            for y in sy as i32..ey as i32 {
                self.draw_pixel(x as i32, y);
                x += (ex - sx) / (ey - sy);
            }
        }
    }
}
