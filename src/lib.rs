#[no_mangle]
pub extern "stdcall" fn wglCreateContext(_hdc: u32) -> u32 {
    1
}

#[no_mangle]
pub extern "stdcall" fn wglMakeCurrent(_hdc: u32, _hglrc: u32) -> bool {
    true
}

#[no_mangle]
pub extern "stdcall" fn wglGetCurrentContext() -> u32 {
    0
}

#[no_mangle]
pub extern "stdcall" fn wglGetCurrentDC() -> u32 {
    0
}

#[no_mangle]
pub extern "stdcall" fn wglDeleteContext(_hglrc: u32) -> bool {
    true
}

#[no_mangle]
pub extern "stdcall" fn wglGetProcAddress(_name: *const u8) -> u32 {
    0
}

#[no_mangle]
pub extern "stdcall" fn glGetString(_name: u32) -> *const u8 {
    "asdf".as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn glBindTexture(_target: u32, _texture: u32) {}

#[no_mangle]
pub extern "stdcall" fn glClearColor(_red: f32, _green: f32, _blue: f32, _alpha: f32) {}

#[no_mangle]
pub extern "stdcall" fn glCullFace(_mode: u32) {}

#[no_mangle]
pub extern "stdcall" fn glEnable(_cap: u32) {}

#[no_mangle]
pub extern "stdcall" fn glDisable(_cap: u32) {}

#[no_mangle]
pub extern "stdcall" fn glAlphaFunc(_func: u32, _ref: u32) {}

#[no_mangle]
pub extern "stdcall" fn glBlendFunc(_sfactor: u32, _dfactor: u32) {}

#[no_mangle]
pub extern "stdcall" fn glDepthFunc(_func: u32) {}

#[no_mangle]
pub extern "stdcall" fn glDepthRange(_near_val: f64, _far_val: f64) {}

#[no_mangle]
pub extern "stdcall" fn glDepthMask(_flag: bool) {}

#[no_mangle]
pub extern "stdcall" fn glPolygonMode(_face: u32, _mode: u32) {}

#[no_mangle]
pub extern "stdcall" fn glShadeModel(_mode: u32) {}

#[no_mangle]
pub extern "stdcall" fn glTexParameterf(_target: u32, _pname: u32, _param: f32) {}

#[no_mangle]
pub extern "stdcall" fn glTexEnvf(_target: u32, _pname: u32, _param: f32) {}

#[no_mangle]
pub extern "stdcall" fn glTexImage2D(
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
pub extern "stdcall" fn glViewport(_x: u32, _y: u32, _width: u32, _height: u32) {}

#[no_mangle]
pub extern "stdcall" fn glMatrixMode(_mode: u32) {}

#[no_mangle]
pub extern "stdcall" fn glLoadIdentity() {}

#[no_mangle]
pub extern "stdcall" fn glOrtho(
    _left: f64,
    _right: f64,
    _bottom: f64,
    _top: f64,
    _near_val: f64,
    _far_val: f64,
) {
}

#[no_mangle]
pub extern "stdcall" fn glFrustum(
    _left: f64,
    _right: f64,
    _bottom: f64,
    _top: f64,
    _near_val: f64,
    _far_val: f64,
) {
}

#[no_mangle]
pub extern "stdcall" fn glColor3f(_red: f32, _green: f32, _blue: f32) {}

#[no_mangle]
pub extern "stdcall" fn glColor3ubv(_v: *const u8) {}

#[no_mangle]
pub extern "stdcall" fn glColor4f(_red: f32, _green: f32, _blue: f32, _alpha: f32) {}

#[no_mangle]
pub extern "stdcall" fn glColor4fv(_v: *const f32) {}

#[no_mangle]
pub extern "stdcall" fn glBegin(_mode: u32) {}

#[no_mangle]
pub extern "stdcall" fn glEnd() {}

#[no_mangle]
pub extern "stdcall" fn glTexCoord2f(_s: f32, _t: f32) {}

#[no_mangle]
pub extern "stdcall" fn glVertex2f(_x: f32, _y: f32) {}

#[no_mangle]
pub extern "stdcall" fn glVertex3f(_x: f32, _y: f32, _z: f32) {}

#[no_mangle]
pub extern "stdcall" fn glVertex3fv(_v: *const f32) {}

#[no_mangle]
pub extern "stdcall" fn glDrawBuffer(_buf: u32) {}

#[no_mangle]
pub extern "stdcall" fn glRotatef(_angle: f32, _x: f32, _y: f32, _z: f32) {}

#[no_mangle]
pub extern "stdcall" fn glTranslatef(_x: f32, _y: f32, _z: f32) {}

#[no_mangle]
pub extern "stdcall" fn glScalef(_x: f32, _y: f32, _z: f32) {}

#[no_mangle]
pub extern "stdcall" fn glGetFloatv(_pname: u32, _params: *mut f32) {}

#[no_mangle]
pub extern "stdcall" fn glPushMatrix() {}

#[no_mangle]
pub extern "stdcall" fn glPopMatrix() {}

#[no_mangle]
pub extern "stdcall" fn glTexSubImage2D(
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
