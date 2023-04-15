mod math;
mod rasterize;
mod win32;

use std::{cell::RefCell, ffi::c_void};

use math::{Mat4, Vec4};
use rasterize::Framebuffer;

#[derive(Default)]
struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct GLState {
    fb: Option<Framebuffer>,
    matrix_mode: MatrixMode,
    matrix_stacks: [Vec<Mat4>; NUM_MATRIX_MODES],
    viewport: Viewport,
    current_primitive: Primitive,
    bmi: win32::BITMAPINFOHEADER,
}

impl Default for GLState {
    fn default() -> Self {
        Self {
            fb: Default::default(),
            matrix_mode: MatrixMode::ModelView,
            matrix_stacks: [vec![Mat4::identity()], vec![Mat4::identity()]],
            viewport: Default::default(),
            current_primitive: Default::default(),
            bmi: Default::default(),
        }
    }
}

thread_local! {
    static GL_STATE: RefCell<GLState> = RefCell::new(Default::default());
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum MatrixMode {
    ModelView,
    Projection,
}

const NUM_MATRIX_MODES: usize = 2;

#[repr(u32)]
#[derive(Copy, Clone, Default, Debug)]
pub enum PrimitiveMode {
    #[default]
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
    QuadStrip,
    Polygon,
}

#[derive(Default)]
struct Primitive {
    mode: PrimitiveMode,
    vertices: Vec<Vec4>,
}

#[no_mangle]
pub extern "system" fn wglCreateContext(hdc: win32::HDC) -> win32::HGLRC {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let mut rect: win32::RECT = Default::default();
        unsafe {
            win32::GetClientRect(win32::WindowFromDC(hdc), &mut rect as _);
        }
        let (width, height) = (rect.right, rect.bottom);
        state.fb = Some(Framebuffer::new(width as usize, height as usize));

        state.bmi = win32::BITMAPINFOHEADER {
            size: std::mem::size_of::<win32::BITMAPINFOHEADER>() as u32,
            width,
            height,
            planes: 1,
            bit_count: 32,
            compression: win32::BI_RGB,
            size_image: 0,
            x_pels_per_meter: 0,
            y_pels_per_meter: 0,
            clr_used: 0,
            clr_important: 0,
        };
    });

    1
}

#[no_mangle]
pub extern "system" fn wglMakeCurrent(_hdc: win32::HDC, _hglrc: win32::HGLRC) -> win32::BOOL {
    true
}

#[no_mangle]
pub extern "system" fn wglGetCurrentContext() -> win32::HGLRC {
    0
}

#[no_mangle]
pub extern "system" fn wglGetCurrentDC() -> win32::HDC {
    0
}

#[no_mangle]
pub extern "system" fn wglDeleteContext(_hglrc: win32::HGLRC) -> win32::BOOL {
    true
}

#[no_mangle]
pub extern "system" fn wglGetProcAddress(_proc: win32::LPCSTR) -> win32::PROC {
    None
}

#[no_mangle]
pub extern "system" fn wglChoosePixelFormat(_hdc: win32::HDC, _ppfd: *const c_void) -> win32::BOOL {
    true
}

#[no_mangle]
pub extern "system" fn wglSetPixelFormat(
    _hdc: win32::HDC,
    _format: i32,
    _ppfd: *const c_void,
) -> win32::BOOL {
    true
}

#[no_mangle]
pub extern "system" fn wglSwapBuffers(hdc: win32::HDC) -> win32::BOOL {
    GL_STATE.with(|state| {
        let state = state.borrow();
        let fb = state.fb.as_ref().unwrap();

        unsafe {
            win32::StretchDIBits(
                hdc,
                0,
                0,
                fb.width as i32,
                fb.height as i32,
                0,
                0,
                fb.width as i32,
                fb.height as i32,
                fb.buffer.as_ptr() as *const c_void,
                &state.bmi as _,
                win32::DIB_RGB_COLORS,
                win32::SRCCOPY,
            );
        }
    });
    true
}

#[no_mangle]
pub extern "system" fn glGetString(_name: u32) -> *const u8 {
    b"asdf\0".as_ptr()
}

#[no_mangle]
pub extern "system" fn glBindTexture(_target: u32, _texture: u32) {}

#[no_mangle]
pub extern "system" fn glClearColor(_red: f32, _green: f32, _blue: f32, _alpha: f32) {}

#[no_mangle]
pub extern "system" fn glClear(_mask: u32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.fb.as_mut().unwrap().clear();
    });
}

#[no_mangle]
pub extern "system" fn glCullFace(_mode: u32) {}

#[no_mangle]
pub extern "system" fn glEnable(_cap: u32) {}

#[no_mangle]
pub extern "system" fn glDisable(_cap: u32) {}

#[no_mangle]
pub extern "system" fn glAlphaFunc(_func: u32, _ref: u32) {}

#[no_mangle]
pub extern "system" fn glBlendFunc(_sfactor: u32, _dfactor: u32) {}

#[no_mangle]
pub extern "system" fn glDepthFunc(_func: u32) {}

#[no_mangle]
pub extern "system" fn glDepthRange(_near_val: f64, _far_val: f64) {}

#[no_mangle]
pub extern "system" fn glDepthMask(_flag: bool) {}

#[no_mangle]
pub extern "system" fn glPolygonMode(_face: u32, _mode: u32) {}

#[no_mangle]
pub extern "system" fn glShadeModel(_mode: u32) {}

#[no_mangle]
pub extern "system" fn glTexParameterf(_target: u32, _pname: u32, _param: f32) {}

#[no_mangle]
pub extern "system" fn glTexEnvf(_target: u32, _pname: u32, _param: f32) {}

#[no_mangle]
pub extern "system" fn glTexImage2D(
    _target: u32,
    _level: u32,
    _internal_format: u32,
    _width: u32,
    _height: u32,
    _border: u32,
    _format: u32,
    _type: u32,
    _data: *const (),
) {
}

#[no_mangle]
pub extern "system" fn glViewport(x: u32, y: u32, width: u32, height: u32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.viewport = Viewport {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        };
    });
}

#[no_mangle]
pub extern "system" fn glMatrixMode(mode: u32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.matrix_mode = match mode {
            0x1700 => MatrixMode::ModelView,
            0x1701 => MatrixMode::Projection,
            _ => todo!(),
        };
    })
}

#[no_mangle]
pub extern "system" fn glLoadIdentity() {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(top) = stack.last_mut() {
            *top = Mat4::identity();
        }
    });
}

#[no_mangle]
pub extern "system" fn glOrtho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near_val: f64,
    far_val: f64,
) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(stack_top) = stack.last_mut() {
            let (left, right) = (left as f32, right as f32);
            let (bottom, top) = (bottom as f32, top as f32);
            let (near_val, far_val) = (near_val as f32, far_val as f32);
            // TODO learn why this is what it is
            let (rpl, rml) = (right + left, right - left);
            let (tpb, tmb) = (top + bottom, top - bottom);
            let (fpn, fmn) = (far_val + near_val, far_val - near_val);
            *stack_top *= Mat4::new(
                [2.0 / rml, 0.0, 0.0, -rpl / rml],
                [0.0, 2.0 / tmb, 0.0, -tpb / tmb],
                [0.0, 0.0, -2.0 / fmn, -fpn / fmn],
                [0.0, 0.0, 0.0, 1.0],
            );
        }
    });
}

#[no_mangle]
pub extern "system" fn glFrustum(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near_val: f64,
    far_val: f64,
) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(stack_top) = stack.last_mut() {
            let (left, right) = (left as f32, right as f32);
            let (bottom, top) = (bottom as f32, top as f32);
            let (near_val, far_val) = (near_val as f32, far_val as f32);
            // TODO review why this is what it is
            let (rpl, rml) = (right + left, right - left);
            let (tpb, tmb) = (top + bottom, top - bottom);
            let (fpn, fmn) = (far_val + near_val, far_val - near_val);
            *stack_top *= Mat4::new(
                [2.0 * near_val / rml, 0.0, rpl / rml, 0.0],
                [0.0, 2.0 * near_val / tmb, tpb / tmb, 0.0],
                [0.0, 0.0, -fpn / fmn, -2.0 * far_val * near_val / fmn],
                [0.0, 0.0, -1.0, 0.0],
            );
        }
    });
}

#[no_mangle]
pub extern "system" fn glColor3f(_red: f32, _green: f32, _blue: f32) {}

#[no_mangle]
pub extern "system" fn glColor3ubv(_v: *const u8) {}

#[no_mangle]
pub extern "system" fn glColor4f(_red: f32, _green: f32, _blue: f32, _alpha: f32) {}

#[no_mangle]
pub extern "system" fn glColor4fv(_v: *const f32) {}

#[no_mangle]
pub extern "system" fn glBegin(mode: PrimitiveMode) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.current_primitive.mode = mode;
        state.current_primitive.vertices.clear();
    });
}

#[no_mangle]
pub extern "system" fn glEnd() {
    GL_STATE.with(|state| {
        let state = &mut *state.borrow_mut();

        // match state.current_primitive.mode {
        //     PrimitiveMode::Quads => todo!(),
        //     PrimitiveMode::Triangles => todo!(),
        //     PrimitiveMode::TriangleStrip => todo!(),
        //     PrimitiveMode::TriangleFan => todo!(),
        //     PrimitiveMode::Polygon => todo!()
        //     _ => todo!(),
        // }

        for vert in &mut state.current_primitive.vertices {
            *vert = *state.matrix_stacks[MatrixMode::Projection as usize]
                .last()
                .unwrap()
                * *state.matrix_stacks[MatrixMode::ModelView as usize]
                    .last()
                    .unwrap()
                * *vert;

            // perspective divide
            if vert.0[3] > 0.0 {
                vert.0[0] /= vert.0[3];
                vert.0[1] /= vert.0[3];
                vert.0[2] /= vert.0[3];
            }

            vert.0[0] = (vert.0[0] + 1.0) * state.viewport.width * 0.5 + state.viewport.x;
            vert.0[1] = (vert.0[1] + 1.0) * state.viewport.height * 0.5 + state.viewport.y;
        }

        let verts = &state.current_primitive.vertices;
        for i in 0..verts.len() {
            let j = (i + 1) % verts.len();

            // world's shittiest clipping algorithm
            if verts[i].0[3] <= 0.1 || verts[j].0[3] <= 0.1 {
                continue;
            }

            state.fb.as_mut().unwrap().draw_line(
                verts[i].0[0],
                verts[i].0[1],
                verts[j].0[0],
                verts[j].0[1],
            );
        }
    })
}

#[no_mangle]
pub extern "system" fn glTexCoord2f(_s: f32, _t: f32) {}

#[no_mangle]
pub extern "system" fn glVertex2f(x: f32, y: f32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state
            .current_primitive
            .vertices
            .push(Vec4::new(x, y, 0.0, 1.0));
    });
}

#[no_mangle]
pub extern "system" fn glVertex3f(x: f32, y: f32, z: f32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state
            .current_primitive
            .vertices
            .push(Vec4::new(x, y, z, 1.0));
    });
}

#[no_mangle]
pub extern "system" fn glVertex3fv(v: &[f32; 3]) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state
            .current_primitive
            .vertices
            .push(Vec4::new(v[0], v[1], v[2], 1.0));
    });
}

#[no_mangle]
pub extern "system" fn glDrawBuffer(_buf: u32) {}

#[no_mangle]
pub extern "system" fn glRotatef(angle: f32, x: f32, y: f32, z: f32) {
    // TODO maybe make sure i can derive this
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(top) = stack.last_mut() {
            let norm = (x * x + y * y + z * z).sqrt();
            let (x, y, z) = (x / norm, y / norm, z / norm);
            let c = angle.to_radians().cos();
            let s = angle.to_radians().sin();
            *top *= Mat4::new(
                [
                    x * x * (1.0 - c) + c,
                    x * y * (1.0 - c) - z * s,
                    x * z * (1.0 - c) + y * s,
                    0.0,
                ],
                [
                    y * x * (1.0 - c) + z * s,
                    y * y * (1.0 - c) + c,
                    y * z * (1.0 - c) - x * s,
                    0.0,
                ],
                [
                    z * x * (1.0 - c) - y * s,
                    z * y * (1.0 - c) + x * s,
                    z * z * (1.0 - c) + c,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            );
        }
    });
}

#[no_mangle]
pub extern "system" fn glTranslatef(x: f32, y: f32, z: f32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(top) = stack.last_mut() {
            *top *= Mat4::new(
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            );
        }
    });
}

#[no_mangle]
pub extern "system" fn glScalef(x: f32, y: f32, z: f32) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(top) = stack.last_mut() {
            *top *= Mat4::new(
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            );
        }
    });
}

#[no_mangle]
pub extern "system" fn glGetFloatv(_pname: u32, _params: *mut f32) {}

#[no_mangle]
pub extern "system" fn glPushMatrix() {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        if let Some(top) = stack.last() {
            stack.push(*top);
        }
    });
}

#[no_mangle]
pub extern "system" fn glPopMatrix() {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();

        let index = state.matrix_mode as usize;
        let stack = &mut state.matrix_stacks[index];
        stack.pop();
    });
}

#[no_mangle]
pub extern "system" fn glTexSubImage2D(
    _target: u32,
    _level: u32,
    _xoffset: u32,
    _yoffset: u32,
    _width: u32,
    _height: u32,
    _format: u32,
    _type: u32,
    _pixels: *const (),
) {
}
