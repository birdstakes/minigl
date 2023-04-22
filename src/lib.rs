mod math;
mod rasterize;
mod win32;

use std::{cell::RefCell, ffi::c_void};

use math::{Mat4, Vec4};
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

const GL_TEXTURE_2D: GLenum = 0x0de1;
const GL_UNSIGNED_BYTE: GLenum = 0x1401;
const GL_MODELVIEW: GLenum = 0x1700;
const GL_PROJECTION: GLenum = 0x1701;
const GL_RGB: GLenum = 0x1907;
const GL_RGBA: GLenum = 0x1908;
const GL_LUMINANCE: GLenum = 0x1909;
const GL_LUMINANCE_ALPHA: GLenum = 0x190a;

#[derive(Default)]
struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Default)]
struct Texture {
    width: usize,
    height: usize,
    data: Vec<Vec4>,
}

struct GLState {
    fb: Option<Framebuffer>,
    matrix_mode: MatrixMode,
    matrix_stacks: [Vec<Mat4>; NUM_MATRIX_MODES],
    viewport: Viewport,
    primitive: Primitive,
    tex_coord: Vec4,
    bound_texture: usize,
    textures: Vec<Texture>,
    bmi: win32::BITMAPINFOHEADER,
}

impl Default for GLState {
    fn default() -> Self {
        Self {
            fb: Default::default(),
            matrix_mode: MatrixMode::ModelView,
            matrix_stacks: [vec![Mat4::identity()], vec![Mat4::identity()]],
            viewport: Default::default(),
            primitive: Default::default(),
            tex_coord: Vec4::new(0.0, 0.0, 0.0, 1.0),
            bound_texture: 0,
            textures: vec![Default::default()],
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
    vertices: Vec<Vertex>,
}

#[derive(Clone, Copy)]
struct Vertex {
    position: Vec4,
    tex_coord: Vec4,
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
pub extern "system" fn glBindTexture(_target: GLenum, texture: GLuint) {
    GL_STATE.with(|state| {
        let state = &mut *state.borrow_mut();
        state.bound_texture = texture as usize;
        if state.bound_texture >= state.textures.len() {
            state
                .textures
                .resize_with(state.bound_texture + 1, Default::default)
        }
    });
}

#[no_mangle]
pub extern "system" fn glTexImage2D(
    target: GLenum,
    level: GLint,
    internal_format: GLint,
    width: GLsizei,
    height: GLsizei,
    _border: GLint,
    format: GLenum,
    type_: GLenum,
    data: *const GLvoid,
) {
    GL_STATE.with(|state| {
        let state = &mut *state.borrow_mut();

        let internal_format = match internal_format {
            1 => GL_LUMINANCE,
            2 => GL_LUMINANCE_ALPHA,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => internal_format as GLenum,
        };

        // we only care about what quake uses (for now, at least)
        assert!(target == GL_TEXTURE_2D);
        assert!(
            internal_format == GL_RGB
                || internal_format == GL_RGBA
                || internal_format == GL_LUMINANCE
        );
        assert!(format == GL_RGBA || format == GL_LUMINANCE);
        assert_eq!(type_, GL_UNSIGNED_BYTE);

        if format != GL_RGBA || level != 0 {
            return;
        }

        let texture = &mut state.textures[state.bound_texture];
        texture.width = width as usize;
        texture.height = height as usize;
        texture
            .data
            .resize_with(texture.width * texture.height, Default::default);

        if !data.is_null() {
            let data = unsafe {
                std::slice::from_raw_parts(
                    data as *const GLubyte,
                    width as usize * height as usize * 4,
                )
            };

            for y in 0..height {
                for x in 0..width {
                    let index = ((x + y * width) * 4) as usize;
                    let color = Vec4::new(
                        data[index] as f32 / 256.0,
                        data[index + 1] as f32 / 256.0,
                        data[index + 2] as f32 / 256.0,
                        data[index + 3] as f32 / 256.0,
                    );
                    texture.data[(x + y * width) as usize] = color;
                }
            }
        }
    });
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
        state.primitive.mode = mode;
        state.primitive.vertices.clear();
    });
}

#[no_mangle]
pub extern "system" fn glEnd() {
    GL_STATE.with(|state| {
        let state = &mut *state.borrow_mut();
        let fb = state.fb.as_mut().unwrap();
        let verts = &mut state.primitive.vertices;

        for vert in verts.iter_mut() {
            vert.position = *state.matrix_stacks[MatrixMode::Projection as usize]
                .last()
                .unwrap()
                * *state.matrix_stacks[MatrixMode::ModelView as usize]
                    .last()
                    .unwrap()
                * vert.position;

            if vert.position.w > 0.0 {
                vert.position.x /= vert.position.w;
                vert.position.y /= vert.position.w;
                vert.position.z /= vert.position.w;
            }

            vert.position.x =
                (vert.position.x + 1.0) * state.viewport.width * 0.5 + state.viewport.x;
            vert.position.y =
                (vert.position.y + 1.0) * state.viewport.height * 0.5 + state.viewport.y;
        }

        let mut tris = vec![];

        match state.primitive.mode {
            PrimitiveMode::Triangles => {
                for i in (0..verts.len()).step_by(3) {
                    tris.push([verts[i], verts[i + 1], verts[i + 2]]);
                }
            }

            PrimitiveMode::Quads => {
                for i in (0..verts.len()).step_by(4) {
                    tris.push([verts[i], verts[i + 1], verts[i + 2]]);
                    tris.push([verts[i + 2], verts[i + 3], verts[i]]);
                }
            }

            PrimitiveMode::TriangleStrip => {
                for i in 0..verts.len() - 2 {
                    if i % 2 == 0 {
                        tris.push([verts[i], verts[i + 1], verts[i + 2]]);
                    } else {
                        tris.push([verts[i + 1], verts[i], verts[i + 2]]);
                    }
                }
            }

            PrimitiveMode::TriangleFan | PrimitiveMode::Polygon => {
                for i in 1..verts.len() - 1 {
                    tris.push([verts[0], verts[i], verts[i + 1]]);
                }
            }

            _ => todo!(),
        }

        let texture = &state.textures[state.bound_texture];

        for tri in &tris {
            fb.draw_triangle(
                [tri[0].position, tri[1].position, tri[2].position],
                |bary| {
                    if texture.width == 0 || texture.height == 0 {
                        return bary;
                    }
                    let uv = bary[0] * tri[0].tex_coord
                        + bary[1] * tri[1].tex_coord
                        + bary[2] * tri[2].tex_coord;
                    let x = (uv[0].rem_euclid(1.0) * texture.width as f32) as usize % texture.width;
                    let y =
                        (uv[1].rem_euclid(1.0) * texture.height as f32) as usize % texture.height;
                    texture.data[x + y * texture.width].xyz()
                },
            );
        }
    })
}

#[no_mangle]
pub extern "system" fn glTexCoord2f(s: GLfloat, t: GLfloat) {
    GL_STATE.with(|state| {
        let state = &mut *state.borrow_mut();
        state.tex_coord = Vec4::new(s, t, 0.0, 1.0);
    })
}

#[no_mangle]
pub extern "system" fn glVertex2f(x: GLfloat, y: GLfloat) {
    glVertex4f(x, y, 0.0, 1.0);
}

#[no_mangle]
pub extern "system" fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat) {
    glVertex4f(x, y, z, 1.0);
}

#[no_mangle]
pub extern "system" fn glVertex3fv(v: &[GLfloat; 3]) {
    glVertex4f(v[0], v[1], v[2], 1.0);
}

#[no_mangle]
pub extern "system" fn glVertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    GL_STATE.with(|state| {
        let state = &mut *state.borrow_mut();
        state.primitive.vertices.push(Vertex {
            position: Vec4::new(x, y, z, w),
            tex_coord: state.tex_coord,
        });
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
