mod math;
mod rasterize;
mod win32;

use std::{cell::RefCell, ffi::c_void};

use math::{Mat4, Vec3, Vec4};
use rasterize::Framebuffer;

type GLenum = std::ffi::c_uint;
type GLboolean = std::ffi::c_uchar;
type GLbitfield = std::ffi::c_uint;
type GLint = std::ffi::c_int;
type GLsizei = std::ffi::c_int;
type GLubyte = std::ffi::c_uchar;
type GLuint = std::ffi::c_uint;
type GLfloat = std::ffi::c_float;
type GLclampf = std::ffi::c_float;
type GLdouble = std::ffi::c_double;
type GLvoid = std::ffi::c_void;

const GL_MODELVIEW: u32 = 0x1700;
const GL_PROJECTION: u32 = 0x1701;

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
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
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
pub extern "system" fn glGetString(_name: GLenum) -> *const GLubyte {
    b"asdf\0".as_ptr()
}

#[no_mangle]
pub extern "system" fn glBindTexture(_target: GLenum, _texture: GLuint) {}

#[no_mangle]
pub extern "system" fn glClearColor(
    _red: GLclampf,
    _green: GLclampf,
    _blue: GLclampf,
    _alpha: GLclampf,
) {
}

#[no_mangle]
pub extern "system" fn glClear(_mask: GLbitfield) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.fb.as_mut().unwrap().clear();
    });
}

#[no_mangle]
pub extern "system" fn glCullFace(_mode: GLenum) {}

#[no_mangle]
pub extern "system" fn glEnable(_cap: GLenum) {}

#[no_mangle]
pub extern "system" fn glDisable(_cap: GLenum) {}

#[no_mangle]
pub extern "system" fn glAlphaFunc(_func: GLenum, _ref: GLclampf) {}

#[no_mangle]
pub extern "system" fn glBlendFunc(_sfactor: GLenum, _dfactor: GLenum) {}

#[no_mangle]
pub extern "system" fn glDepthFunc(_func: GLenum) {}

#[no_mangle]
pub extern "system" fn glDepthRange(_near_val: GLdouble, _far_val: GLdouble) {}

#[no_mangle]
pub extern "system" fn glDepthMask(_flag: GLboolean) {}

#[no_mangle]
pub extern "system" fn glPolygonMode(_face: GLenum, _mode: GLenum) {}

#[no_mangle]
pub extern "system" fn glShadeModel(_mode: GLenum) {}

#[no_mangle]
pub extern "system" fn glTexParameterf(_target: GLenum, _pname: GLenum, _param: GLfloat) {}

#[no_mangle]
pub extern "system" fn glTexEnvf(_target: GLenum, _pname: GLenum, _param: GLfloat) {}

#[no_mangle]
pub extern "system" fn glTexImage2D(
    _target: GLenum,
    _level: GLint,
    _internal_format: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _border: GLint,
    _format: GLenum,
    _type: GLenum,
    _data: *const GLvoid,
) {
}

#[no_mangle]
pub extern "system" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
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
pub extern "system" fn glMatrixMode(mode: GLenum) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.matrix_mode = match mode {
            GL_MODELVIEW => MatrixMode::ModelView,
            GL_PROJECTION => MatrixMode::Projection,
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
    left: GLdouble,
    right: GLdouble,
    bottom: GLdouble,
    top: GLdouble,
    near_val: GLdouble,
    far_val: GLdouble,
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
    left: GLdouble,
    right: GLdouble,
    bottom: GLdouble,
    top: GLdouble,
    near_val: GLdouble,
    far_val: GLdouble,
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
pub extern "system" fn glColor3f(_red: GLfloat, _green: GLfloat, _blue: GLfloat) {}

#[no_mangle]
pub extern "system" fn glColor3ubv(_v: *const GLubyte) {}

#[no_mangle]
pub extern "system" fn glColor4f(_red: GLfloat, _green: GLfloat, _blue: GLfloat, _alpha: GLfloat) {}

#[no_mangle]
pub extern "system" fn glColor4fv(_v: *const GLfloat) {}

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
        let fb = state.fb.as_mut().unwrap();
        let verts = &mut state.current_primitive.vertices;

        for vert in verts.iter_mut() {
            *vert = *state.matrix_stacks[MatrixMode::Projection as usize]
                .last()
                .unwrap()
                * *state.matrix_stacks[MatrixMode::ModelView as usize]
                    .last()
                    .unwrap()
                * *vert;

            if vert.w > 0.0 {
                vert.x /= vert.w;
                vert.y /= vert.w;
                vert.z /= vert.w;
            }

            vert.x = (vert.x + 1.0) * state.viewport.width * 0.5 + state.viewport.x;
            vert.y = (vert.y + 1.0) * state.viewport.height * 0.5 + state.viewport.y;
        }

        let shader = |bary: Vec3| bary;

        match state.current_primitive.mode {
            PrimitiveMode::Triangles => {
                for i in (0..verts.len()).step_by(3) {
                    let tri = &verts[i..i + 3];
                    fb.draw_triangle([tri[0], tri[1], tri[2]], shader)
                }
            }
            PrimitiveMode::Quads => {
                for i in (0..verts.len()).step_by(4) {
                    let quad = &verts[i..i + 4];
                    fb.draw_triangle([quad[0], quad[1], quad[2]], shader);
                    fb.draw_triangle([quad[2], quad[3], quad[0]], shader);
                }
            }
            PrimitiveMode::TriangleStrip => {
                for i in 0..verts.len() - 2 {
                    if i % 2 == 0 {
                        fb.draw_triangle([verts[i], verts[i + 1], verts[i + 2]], shader);
                    } else {
                        fb.draw_triangle([verts[i + 1], verts[i], verts[i + 2]], shader);
                    }
                }
            }
            PrimitiveMode::TriangleFan | PrimitiveMode::Polygon => {
                for i in 1..verts.len() - 1 {
                    fb.draw_triangle([verts[0], verts[i], verts[i + 1]], shader);
                }
            }
            _ => todo!(),
        }
    })
}

#[no_mangle]
pub extern "system" fn glTexCoord2f(_s: GLfloat, _t: GLfloat) {}

#[no_mangle]
pub extern "system" fn glVertex2f(x: GLfloat, y: GLfloat) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state
            .current_primitive
            .vertices
            .push(Vec4::new(x, y, 0.0, 1.0));
    });
}

#[no_mangle]
pub extern "system" fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state
            .current_primitive
            .vertices
            .push(Vec4::new(x, y, z, 1.0));
    });
}

#[no_mangle]
pub extern "system" fn glVertex3fv(v: &[GLfloat; 3]) {
    GL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state
            .current_primitive
            .vertices
            .push(Vec4::new(v[0], v[1], v[2], 1.0));
    });
}

#[no_mangle]
pub extern "system" fn glDrawBuffer(_buf: GLenum) {}

#[no_mangle]
pub extern "system" fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) {
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
pub extern "system" fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat) {
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
pub extern "system" fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat) {
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
pub extern "system" fn glGetFloatv(_pname: GLenum, _params: *mut GLfloat) {}

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
    _target: GLenum,
    _level: GLint,
    _xoffset: GLint,
    _yoffset: GLint,
    _width: GLsizei,
    _height: GLsizei,
    _format: GLenum,
    _type: GLenum,
    _pixels: *const c_void,
) {
}
