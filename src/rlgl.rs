use std::ffi::CStr;
use std::os::raw::c_void;

use crate::ffi;
use crate::math::Matrix;

pub use crate::ffi::{
    rlBlendMode, rlCullMode, rlDrawCall, rlFramebufferAttachTextureType, rlFramebufferAttachType,
    rlGlVersion, rlPixelFormat, rlRenderBatch, rlShaderAttributeDataType, rlShaderLocationIndex,
    rlShaderUniformDataType, rlTextureFilter, rlTraceLogLevel, rlVertexBuffer,
};

// Texture parameters (equivalent to OpenGL defines)
//----------------------------------------------------------------------------------
/// GL_TEXTURE_WRAP_S
pub const TEXTURE_WRAP_S: i32 = 0x2802;
/// GL_TEXTURE_WRAP_T
pub const TEXTURE_WRAP_T: i32 = 0x2803;
/// GL_TEXTURE_MAG_FILTER
pub const TEXTURE_MAG_FILTER: i32 = 0x2800;
/// GL_TEXTURE_MIN_FILTER
pub const TEXTURE_MIN_FILTER: i32 = 0x2801;

/// GL_NEAREST
pub const TEXTURE_FILTER_NEAREST: i32 = 0x2600;
/// GL_LINEAR
pub const TEXTURE_FILTER_LINEAR: i32 = 0x2601;
/// GL_NEAREST_MIPMAP_NEAREST
pub const TEXTURE_FILTER_MIP_NEAREST: i32 = 0x2700;
/// GL_NEAREST_MIPMAP_LINEAR
pub const TEXTURE_FILTER_NEAREST_MIP_LINEAR: i32 = 0x2702;
/// GL_LINEAR_MIPMAP_NEAREST
pub const TEXTURE_FILTER_LINEAR_MIP_NEAREST: i32 = 0x2701;
/// GL_LINEAR_MIPMAP_LINEAR
pub const TEXTURE_FILTER_MIP_LINEAR: i32 = 0x2703;
/// Anisotropic filter (custom identifier)
pub const TEXTURE_FILTER_ANISOTROPIC: i32 = 0x3000;
/// Texture mipmap bias, percentage ratio (custom identifier)
pub const TEXTURE_MIPMAP_BIAS_RATIO: i32 = 0x4000;

/// GL_REPEAT
pub const TEXTURE_WRAP_REPEAT: i32 = 0x2901;
/// GL_CLAMP_TO_EDGE
pub const TEXTURE_WRAP_CLAMP: i32 = 0x812F;
/// GL_MIRRORED_REPEAT
pub const TEXTURE_WRAP_MIRROR_REPEAT: i32 = 0x8370;
/// GL_MIRROR_CLAMP_EXT
pub const TEXTURE_WRAP_MIRROR_CLAMP: i32 = 0x8742;

// Matrix modes (equivalent to OpenGL)
//----------------------------------------------------------------------------------
/// GL_MODELVIEW
pub const MODELVIEW: i32 = 0x1700;
/// GL_PROJECTION
pub const PROJECTION: i32 = 0x1701;
/// GL_TEXTURE
pub const TEXTURE: i32 = 0x1702;

// Primitive assembly draw modes
//----------------------------------------------------------------------------------
/// GL_LINES
pub const RL_LINES: i32 = 0x0001;
/// GL_TRIANGLES
pub const RL_TRIANGLES: i32 = 0x0004;
/// GL_QUADS
pub const RL_QUADS: i32 = 0x0007;

// GL equivalent data types
//----------------------------------------------------------------------------------
/// GL_UNSIGNED_BYTE
pub const UNSIGNED_BYTE: i32 = 0x1401;
/// GL_FLOAT
pub const FLOAT: i32 = 0x1406;

// GL buffer usage hint
//----------------------------------------------------------------------------------
/// GL_STREAM_DRAW
pub const STREAM_DRAW: i32 = 0x88E0;
/// GL_STREAM_READ
pub const STREAM_READ: i32 = 0x88E1;
/// GL_STREAM_COPY
pub const STREAM_COPY: i32 = 0x88E2;
/// GL_STATIC_DRAW
pub const STATIC_DRAW: i32 = 0x88E4;
/// GL_STATIC_READ
pub const STATIC_READ: i32 = 0x88E5;
/// GL_STATIC_COPY
pub const STATIC_COPY: i32 = 0x88E6;
/// GL_DYNAMIC_DRAW
pub const DYNAMIC_DRAW: i32 = 0x88E8;
/// GL_DYNAMIC_READ
pub const DYNAMIC_READ: i32 = 0x88E9;
/// GL_DYNAMIC_COPY
pub const DYNAMIC_COPY: i32 = 0x88EA;

// GL Shader type
//----------------------------------------------------------------------------------
/// GL_FRAGMENT_SHADER
pub const FRAGMENT_SHADER: i32 = 0x8B30;
/// GL_VERTEX_SHADER
pub const VERTEX_SHADER: i32 = 0x8B31;
/// GL_COMPUTE_SHADER
pub const COMPUTE_SHADER: i32 = 0x91B9;

// GL blending factors
//----------------------------------------------------------------------------------
/// GL_ZERO
pub const ZERO: i32 = 0;
/// RL_ONE
pub const RL_ONE: i32 = 1;
/// GL_SRC_COLOR
pub const SRC_COLOR: i32 = 0x0300;
/// GL_ONE_MINUS_SRC_COLOR
pub const RL_ONE_MINUS_SRC_COLOR: i32 = 0x0301;
/// GL_SRC_ALPHA
pub const RL_SRC_ALPHA: i32 = 0x0302;
/// GL_ONE_MINUS_SRC_ALPHA
pub const RL_ONE_MINUS_SRC_ALPHA: i32 = 0x0303;
/// GL_DST_ALPHA
pub const DST_ALPHA: i32 = 0x0304;
/// GL_ONE_MINUS_DST_ALPHA
pub const RL_ONE_MINUS_DST_ALPHA: i32 = 0x0305;
/// GL_DST_COLOR
pub const DST_COLOR: i32 = 0x0306;
/// GL_ONE_MINUS_DST_COLOR
pub const ONE_MINUS_DST_COLOR: i32 = 0x0307;
/// GL_SRC_ALPHA_SATURATE
pub const SRC_ALPHA_SATURATE: i32 = 0x0308;
/// GL_CONSTANT_COLOR
pub const CONSTANT_COLOR: i32 = 0x8001;
/// GL_ONE_MINUS_CONSTANT_COLOR
pub const ONE_MINUS_CONSTANT_COLOR: i32 = 0x8002;
/// GL_CONSTANT_ALPHA
pub const CONSTANT_ALPHA: i32 = 0x8003;
/// GL_ONE_MINUS_CONSTANT_ALPHA
pub const ONE_MINUS_CONSTANT_ALPHA: i32 = 0x8004;

// GL blending functions/equations
//----------------------------------------------------------------------------------
/// GL_FUNC_ADD
pub const RL_FUNC_ADD: i32 = 0x8006;
/// GL_MIN
pub const MIN: i32 = 0x8007;
/// GL_MAX
pub const MAX: i32 = 0x8008;
/// GL_FUNC_SUBTRACT
pub const FUNC_SUBTRACT: i32 = 0x800A;
/// GL_FUNC_REVERSE_SUBTRACT
pub const FUNC_REVERSE_SUBTRACT: i32 = 0x800B;
/// GL_BLEND_EQUATION
pub const BLEND_EQUATION: i32 = 0x8009;
/// GL_BLEND_EQUATION_RGB (Same as BLEND_EQUATION)
pub const BLEND_EQUATION_RGB: i32 = 0x8009;
/// GL_BLEND_EQUATION_ALPHA
pub const BLEND_EQUATION_ALPHA: i32 = 0x883D;
/// GL_BLEND_DST_RGB
pub const BLEND_DST_RGB: i32 = 0x80C8;
/// GL_BLEND_SRC_RGB
pub const BLEND_SRC_RGB: i32 = 0x80C9;
/// GL_BLEND_DST_ALPHA
pub const BLEND_DST_ALPHA: i32 = 0x80CA;
/// GL_BLEND_SRC_ALPHA
pub const BLEND_SRC_ALPHA: i32 = 0x80CB;
/// GL_BLEND_COLOR
pub const BLEND_COLOR: i32 = 0x8005;

// GL framebuffer targets
//----------------------------------------------------------------------------------
/// GL_READ_FRAMEBUFFER
pub const READ_FRAMEBUFFER: i32 = 0x8CA8;
/// GL_DRAW_FRAMEBUFFER
pub const DRAW_FRAMEBUFFER: i32 = 0x8CA9;
//------------------------------------------------------------------------------------
// Functions Declaration - Matrix operations
//------------------------------------------------------------------------------------

/// Choose the current matrix to be transformed
pub fn rlMatrixMode(mode: i32) {
    unsafe {
        ffi::rlMatrixMode(mode);
    }
}

/// Push the current matrix to stack
pub fn rlPushMatrix() {
    unsafe {
        ffi::rlPushMatrix();
    }
}

/// Pop latest inserted matrix from stack
pub fn rlPopMatrix() {
    unsafe {
        ffi::rlPopMatrix();
    }
}

/// Reset current matrix to identity matrix
pub fn rlLoadIdentity() {
    unsafe {
        ffi::rlLoadIdentity();
    }
}

/// Multiply the current matrix by a translation matrix
pub fn rlTranslatef(x: f32, y: f32, z: f32) {
    unsafe {
        ffi::rlTranslatef(x, y, z);
    }
}

/// Multiply the current matrix by a rotation matrix
pub fn rlRotatef(angle: f32, x: f32, y: f32, z: f32) {
    unsafe {
        ffi::rlRotatef(angle, x, y, z);
    }
}

/// Multiply the current matrix by a scaling matrix
pub fn rlScalef(x: f32, y: f32, z: f32) {
    unsafe {
        ffi::rlScalef(x, y, z);
    }
}

/// Multiply the current matrix by another matrix (a 4x4 matrix)
pub fn rlMultMatrixf(matf: &[f32; 16]) {
    unsafe {
        // The C function expects a pointer to the first float.
        // .as_ptr() safely provides this from our fixed-size array reference.
        ffi::rlMultMatrixf(matf.as_ptr());
    }
}

/// Multiply the current matrix by a perspective matrix
pub fn rlFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe {
        ffi::rlFrustum(left, right, bottom, top, znear, zfar);
    }
}

/// Multiply the current matrix by an orthographic matrix
pub fn rlOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe {
        ffi::rlOrtho(left, right, bottom, top, znear, zfar);
    }
}

/// Set the viewport area
pub fn rlViewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        ffi::rlViewport(x, y, width, height);
    }
}

/// Set clip planes distances
pub fn rlSetClipPlanes(near_plane: f64, far_plane: f64) {
    unsafe {
        ffi::rlSetClipPlanes(near_plane, far_plane);
    }
}

/// Get cull plane distance near
pub fn rlGetCullDistanceNear() -> f64 {
    unsafe { ffi::rlGetCullDistanceNear() }
}

/// Get cull plane distance far
pub fn rlGetCullDistanceFar() -> f64 {
    unsafe { ffi::rlGetCullDistanceFar() }
}

//------------------------------------------------------------------------------------
// Functions Declaration - Vertex level operations
//------------------------------------------------------------------------------------

/// Initialize drawing mode (how to organize vertex)
pub fn rlBegin(mode: i32) {
    unsafe {
        ffi::rlBegin(mode);
    }
}

/// Finish vertex providing
pub fn rlEnd() {
    unsafe {
        ffi::rlEnd();
    }
}

/// Define one vertex (position) - 2 int
pub fn rlVertex2i(x: i32, y: i32) {
    unsafe {
        ffi::rlVertex2i(x, y);
    }
}

/// Define one vertex (position) - 2 float
pub fn rlVertex2f(x: f32, y: f32) {
    unsafe {
        ffi::rlVertex2f(x, y);
    }
}

/// Define one vertex (position) - 3 float
pub fn rlVertex3f(x: f32, y: f32, z: f32) {
    unsafe {
        ffi::rlVertex3f(x, y, z);
    }
}

/// Define one vertex (texture coordinate) - 2 float
pub fn rlTexCoord2f(x: f32, y: f32) {
    unsafe {
        ffi::rlTexCoord2f(x, y);
    }
}

/// Define one vertex (normal) - 3 float
pub fn rlNormal3f(x: f32, y: f32, z: f32) {
    unsafe {
        ffi::rlNormal3f(x, y, z);
    }
}

/// Define one vertex (color) - 4 byte
pub fn rlColor4ub(r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        ffi::rlColor4ub(r, g, b, a);
    }
}

/// Define one vertex (color) - 3 float
pub fn rlColor3f(x: f32, y: f32, z: f32) {
    unsafe {
        ffi::rlColor3f(x, y, z);
    }
}

/// Define one vertex (color) - 4 float
pub fn rlColor4f(x: f32, y: f32, z: f32, w: f32) {
    unsafe {
        ffi::rlColor4f(x, y, z, w);
    }
}

//---------------------------------
// Vertex buffers state
//---------------------------------

/// Enable vertex array (VAO, if supported). Returns true if enabled.
pub fn enable_vertex_array(vao_id: u32) -> bool {
    unsafe { ffi::rlEnableVertexArray(vao_id) }
}

/// Disable vertex array (VAO, if supported)
pub fn disable_vertex_array() {
    unsafe { ffi::rlDisableVertexArray() }
}

/// Enable vertex buffer (VBO)
pub fn enable_vertex_buffer(id: u32) {
    unsafe { ffi::rlEnableVertexBuffer(id) }
}

/// Disable vertex buffer (VBO)
pub fn disable_vertex_buffer() {
    unsafe { ffi::rlDisableVertexBuffer() }
}

/// Enable vertex buffer element (VBO element)
pub fn enable_vertex_buffer_element(id: u32) {
    unsafe { ffi::rlEnableVertexBufferElement(id) }
}

/// Disable vertex buffer element (VBO element)
pub fn disable_vertex_buffer_element() {
    unsafe { ffi::rlDisableVertexBufferElement() }
}

/// Enable vertex attribute index
pub fn enable_vertex_attribute(index: u32) {
    unsafe { ffi::rlEnableVertexAttribute(index) }
}

/// Disable vertex attribute index
pub fn disable_vertex_attribute(index: u32) {
    unsafe { ffi::rlDisableVertexAttribute(index) }
}

/// Enable attribute state pointer (only for OpenGL 1.1)
#[cfg(feature = "opengl_11")]
pub fn enable_state_pointer(vertex_attrib_type: i32, buffer: *mut c_void) {
    unsafe { ffi::rlEnableStatePointer(vertex_attrib_type, buffer) }
}

/// Disable attribute state pointer (only for OpenGL 1.1)
#[cfg(feature = "opengl_11")]
pub fn disable_state_pointer(vertex_attrib_type: i32) {
    unsafe { ffi::rlDisableStatePointer(vertex_attrib_type) }
}

// Textures state
//---------------------------------

/// Select and active a texture slot
pub fn active_texture_slot(slot: i32) {
    unsafe { ffi::rlActiveTextureSlot(slot) }
}

/// Enable texture
pub fn enable_texture(id: u32) {
    unsafe { ffi::rlEnableTexture(id) }
}

/// Disable texture
pub fn disable_texture() {
    unsafe { ffi::rlDisableTexture() }
}

/// Enable texture cubemap
pub fn enable_texture_cubemap(id: u32) {
    unsafe { ffi::rlEnableTextureCubemap(id) }
}

/// Disable texture cubemap
pub fn disable_texture_cubemap() {
    unsafe { ffi::rlDisableTextureCubemap() }
}

/// Set texture parameters (filter, wrap)
pub fn texture_parameters(id: u32, param: i32, value: i32) {
    unsafe { ffi::rlTextureParameters(id, param, value) }
}

/// Set cubemap parameters (filter, wrap)
pub fn cubemap_parameters(id: u32, param: i32, value: i32) {
    unsafe { ffi::rlCubemapParameters(id, param, value) }
}

// Shader state
//---------------------------------

/// Enable shader program
pub fn rlEnableShader(id: u32) {
    unsafe { ffi::rlEnableShader(id) }
}

/// Disable shader program
pub fn rlDisableShader() {
    unsafe { ffi::rlDisableShader() }
}

// Framebuffer state
//---------------------------------

/// Enable render texture (fbo)
pub fn enable_framebuffer(id: u32) {
    unsafe { ffi::rlEnableFramebuffer(id) }
}

/// Disable render texture (fbo), return to default framebuffer
pub fn disable_framebuffer() {
    unsafe { ffi::rlDisableFramebuffer() }
}

/// Get the currently active render texture (fbo), 0 for default framebuffer
pub fn get_active_framebuffer() -> u32 {
    unsafe { ffi::rlGetActiveFramebuffer() }
}

/// Activate multiple draw color buffers
pub fn active_draw_buffers(count: i32) {
    unsafe { ffi::rlActiveDrawBuffers(count) }
}

/// Blit active framebuffer to main framebuffer
pub fn blit_framebuffer(
    src_x: i32,
    src_y: i32,
    src_width: i32,
    src_height: i32,
    dst_x: i32,
    dst_y: i32,
    dst_width: i32,
    dst_height: i32,
    buffer_mask: i32,
) {
    unsafe {
        ffi::rlBlitFramebuffer(
            src_x,
            src_y,
            src_width,
            src_height,
            dst_x,
            dst_y,
            dst_width,
            dst_height,
            buffer_mask,
        )
    }
}

/// Bind framebuffer (FBO)
pub fn bind_framebuffer(target: u32, framebuffer: u32) {
    unsafe { ffi::rlBindFramebuffer(target, framebuffer) }
}

// General render state
//---------------------------------

/// Enable color blending
pub fn enable_color_blend() {
    unsafe { ffi::rlEnableColorBlend() }
}

/// Disable color blending
pub fn disable_color_blend() {
    unsafe { ffi::rlDisableColorBlend() }
}

/// Enable depth test
pub fn enable_depth_test() {
    unsafe { ffi::rlEnableDepthTest() }
}

/// Disable depth test
pub fn rlDisableDepthTest() {
    unsafe { ffi::rlDisableDepthTest() }
}

/// Enable depth write
pub fn enable_depth_mask() {
    unsafe { ffi::rlEnableDepthMask() }
}

/// Disable depth write
pub fn disable_depth_mask() {
    unsafe { ffi::rlDisableDepthMask() }
}

/// Enable backface culling
pub fn rlEnableBackfaceCulling() {
    unsafe { ffi::rlEnableBackfaceCulling() }
}

/// Disable backface culling
pub fn rlDisableBackfaceCulling() {
    unsafe { ffi::rlDisableBackfaceCulling() }
}

/// Color mask control
pub fn rlColorMask(r: bool, g: bool, b: bool, a: bool) {
    unsafe { ffi::rlColorMask(r, g, b, a) }
}

/// Set face culling mode
pub fn set_cull_face(mode: i32) {
    unsafe { ffi::rlSetCullFace(mode) }
}

/// Enable scissor test
pub fn rlEnableScissorTest() {
    unsafe { ffi::rlEnableScissorTest() }
}

/// Disable scissor test
pub fn rlDisableScissorTest() {
    unsafe { ffi::rlDisableScissorTest() }
}

/// Scissor test
pub fn scissor(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::rlScissor(x, y, width, height) }
}

/// Enable wire mode
pub fn enable_wire_mode() {
    unsafe { ffi::rlEnableWireMode() }
}

/// Disable wire mode
pub fn disable_wire_mode() {
    unsafe { ffi::rlDisableWireMode() }
}

/// Set the line drawing width
pub fn set_line_width(width: f32) {
    unsafe { ffi::rlSetLineWidth(width) }
}

/// Get the line drawing width
pub fn get_line_width() -> f32 {
    unsafe { ffi::rlGetLineWidth() }
}

/// Enable line aliasing
pub fn enable_smooth_lines() {
    unsafe { ffi::rlEnableSmoothLines() }
}

/// Disable line aliasing
pub fn disable_smooth_lines() {
    unsafe { ffi::rlDisableSmoothLines() }
}

/// Enable stereo rendering
pub fn enable_stereo_render() {
    unsafe { ffi::rlEnableStereoRender() }
}

/// Disable stereo rendering
pub fn disable_stereo_render() {
    unsafe { ffi::rlDisableStereoRender() }
}

/// Check if stereo render is enabled
pub fn is_stereo_render_enabled() -> bool {
    unsafe { ffi::rlIsStereoRenderEnabled() }
}

/// Clear color buffer with color
pub fn clear_color(r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::rlClearColor(r, g, b, a) }
}

/// Clear used screen buffers (color and depth)
pub fn clear_screen_buffers() {
    unsafe { ffi::rlClearScreenBuffers() }
}

/// Check and log OpenGL error codes
pub fn check_errors() {
    unsafe { ffi::rlCheckErrors() }
}

/// Set blending mode
pub fn rlSetBlendMode(mode: i32) {
    unsafe { ffi::rlSetBlendMode(mode) }
}

/// Set blending mode factor and equation (using OpenGL factors)
pub fn rlSetBlendFactors(gl_src_factor: i32, gl_dst_factor: i32, gl_equation: i32) {
    unsafe { ffi::rlSetBlendFactors(gl_src_factor, gl_dst_factor, gl_equation) }
}

/// Set blending mode factors and equations separately (using OpenGL factors)
pub fn set_blend_factors_separate(
    gl_src_rgb: i32,
    gl_dst_rgb: i32,
    gl_src_alpha: i32,
    gl_dst_alpha: i32,
    gl_eq_rgb: i32,
    gl_eq_alpha: i32,
) {
    unsafe {
        ffi::rlSetBlendFactorsSeparate(
            gl_src_rgb,
            gl_dst_rgb,
            gl_src_alpha,
            gl_dst_alpha,
            gl_eq_rgb,
            gl_eq_alpha,
        )
    }
}

//------------------------------------------------------------------------------------
// Functions Declaration - rlgl functionality
//------------------------------------------------------------------------------------

/// Initialize rlgl (buffers, shaders, textures, states)
pub fn rlgl_init(width: i32, height: i32) {
    unsafe { ffi::rlglInit(width, height) }
}

/// De-initialize rlgl (buffers, shaders, textures)
pub fn rlgl_close() {
    unsafe { ffi::rlglClose() }
}

/// Load OpenGL extensions (loader function required)
pub fn load_extensions(loader: *mut c_void) {
    unsafe { ffi::rlLoadExtensions(loader) }
}

// TODO: / Get OpenGL procedure address
// pub fn get_proc_address(proc_name: &CStr) -> *mut c_void {
//     unsafe { ffi::rlGetProcAddress(proc_name.as_ptr()) }
// }

/// Get current OpenGL version
pub fn get_version() -> i32 {
    unsafe { ffi::rlGetVersion() }
}

/// Set current framebuffer width
pub fn set_framebuffer_width(width: i32) {
    unsafe { ffi::rlSetFramebufferWidth(width) }
}

/// Get default framebuffer width
pub fn get_framebuffer_width() -> i32 {
    unsafe { ffi::rlGetFramebufferWidth() }
}

/// Set current framebuffer height
pub fn set_framebuffer_height(height: i32) {
    unsafe { ffi::rlSetFramebufferHeight(height) }
}

/// Get default framebuffer height
pub fn get_framebuffer_height() -> i32 {
    unsafe { ffi::rlGetFramebufferHeight() }
}

/// Get default texture id
pub fn rlGetTextureIdDefault() -> u32 {
    unsafe { ffi::rlGetTextureIdDefault() }
}

/// Get default shader id
pub fn get_shader_id_default() -> u32 {
    unsafe { ffi::rlGetShaderIdDefault() }
}

/// Get default shader locations
/// NOTE: Returns a raw pointer to a C array. Caller must handle lifetime and access safely.
pub fn get_shader_locs_default() -> *mut i32 {
    unsafe { ffi::rlGetShaderLocsDefault() }
}

// Render batch management
//---------------------------------

/// Load a render batch system
pub fn load_render_batch(num_buffers: i32, buffer_elements: i32) -> rlRenderBatch {
    unsafe { ffi::rlLoadRenderBatch(num_buffers, buffer_elements) }
}

/// Unload render batch system
pub fn unload_render_batch(batch: rlRenderBatch) {
    unsafe { ffi::rlUnloadRenderBatch(batch) }
}

/// Draw render batch data (Update->Draw->Reset)
pub fn draw_render_batch(batch: &mut rlRenderBatch) {
    unsafe { ffi::rlDrawRenderBatch(batch) }
}

/// Set the active render batch for rlgl (pass None for default internal)
pub fn set_render_batch_active(batch: Option<&mut rlRenderBatch>) {
    let ptr = batch.map_or(std::ptr::null_mut(), |b| b as *mut _);
    unsafe { ffi::rlSetRenderBatchActive(ptr) }
}

/// Update and draw internal render batch
pub fn rlDrawRenderBatchActive() {
    unsafe { ffi::rlDrawRenderBatchActive() }
}

/// Check internal buffer overflow for a given number of vertex
pub fn rlCheckRenderBatchLimit(v_count: i32) -> bool {
    unsafe { ffi::rlCheckRenderBatchLimit(v_count) }
}

/// Set current texture for render batch and check buffers limits
pub fn rlSetTexture(id: u32) {
    unsafe { ffi::rlSetTexture(id) }
}

// Vertex buffers management
//---------------------------------

/// Load vertex array (vao) if supported
pub fn load_vertex_array() -> u32 {
    unsafe { ffi::rlLoadVertexArray() }
}

/// Load a vertex buffer object from a slice of data
pub fn load_vertex_buffer<T>(buffer: &[T], dynamic: bool) -> u32 {
    let size = (buffer.len() * std::mem::size_of::<T>()) as i32;
    unsafe { ffi::rlLoadVertexBuffer(buffer.as_ptr() as *const c_void, size, dynamic) }
}

/// Load vertex buffer elements object from a slice of data
pub fn load_vertex_buffer_element<T>(buffer: &[T], dynamic: bool) -> u32 {
    let size = (buffer.len() * std::mem::size_of::<T>()) as i32;
    unsafe { ffi::rlLoadVertexBufferElement(buffer.as_ptr() as *const c_void, size, dynamic) }
}

/// Update vertex buffer object data on GPU buffer
pub fn update_vertex_buffer<T>(buffer_id: u32, data: &[T], offset: i32) {
    let data_size = (data.len() * std::mem::size_of::<T>()) as i32;
    unsafe {
        ffi::rlUpdateVertexBuffer(buffer_id, data.as_ptr() as *const c_void, data_size, offset)
    }
}

/// Update vertex buffer elements data on GPU buffer
pub fn update_vertex_buffer_elements<T>(id: u32, data: &[T], offset: i32) {
    let data_size = (data.len() * std::mem::size_of::<T>()) as i32;
    unsafe {
        ffi::rlUpdateVertexBufferElements(id, data.as_ptr() as *const c_void, data_size, offset)
    }
}

/// Unload vertex array (vao)
pub fn unload_vertex_array(vao_id: u32) {
    unsafe { ffi::rlUnloadVertexArray(vao_id) }
}

/// Unload vertex buffer object
pub fn unload_vertex_buffer(vbo_id: u32) {
    unsafe { ffi::rlUnloadVertexBuffer(vbo_id) }
}

// TODO: / Set vertex attribute data configuration
// pub fn set_vertex_attribute(
//     index: u32,
//     comp_size: i32,
//     attrib_type: i32,
//     normalized: bool,
//     stride: i32,
//     offset: i32,
// ) {
//     // The C API uses an integer as an offset, which can be interpreted as a pointer.
//     // In Rust, we pass the offset directly.
//     let ptr_offset = offset as *const c_void;
//     unsafe {
//         ffi::rlSetVertexAttribute(
//             index,
//             comp_size,
//             attrib_type,
//             normalized,
//             stride,
//             ptr_offset,
//         )
//     }
// }

/// Set vertex attribute data divisor
pub fn set_vertex_attribute_divisor(index: u32, divisor: i32) {
    unsafe { ffi::rlSetVertexAttributeDivisor(index, divisor) }
}

/// Set vertex attribute default value, when attribute is not provided
pub fn set_vertex_attribute_default<T>(loc_index: i32, value: &[T], attrib_type: i32, count: i32) {
    unsafe {
        ffi::rlSetVertexAttributeDefault(
            loc_index,
            value.as_ptr() as *const c_void,
            attrib_type,
            count,
        )
    }
}

/// Draw vertex array (currently active vao)
pub fn draw_vertex_array(offset: i32, count: i32) {
    unsafe { ffi::rlDrawVertexArray(offset, count) }
}

/// Draw vertex array elements
pub fn draw_vertex_array_elements(offset: i32, count: i32, buffer: *const c_void) {
    unsafe { ffi::rlDrawVertexArrayElements(offset, count, buffer) }
}

/// Draw vertex array (currently active vao) with instancing
pub fn draw_vertex_array_instanced(offset: i32, count: i32, instances: i32) {
    unsafe { ffi::rlDrawVertexArrayInstanced(offset, count, instances) }
}

/// Draw vertex array elements with instancing
pub fn draw_vertex_array_elements_instanced(
    offset: i32,
    count: i32,
    buffer: *const c_void,
    instances: i32,
) {
    unsafe { ffi::rlDrawVertexArrayElementsInstanced(offset, count, buffer, instances) }
}

// Textures management
//---------------------------------

/// Load texture data from a slice
pub fn load_texture(data: &[u8], width: i32, height: i32, format: i32, mipmap_count: i32) -> u32 {
    unsafe {
        ffi::rlLoadTexture(
            data.as_ptr() as *const c_void,
            width,
            height,
            format,
            mipmap_count,
        )
    }
}

/// Load depth texture/renderbuffer (to be attached to fbo)
pub fn load_texture_depth(width: i32, height: i32, use_render_buffer: bool) -> u32 {
    unsafe { ffi::rlLoadTextureDepth(width, height, use_render_buffer) }
}

/// Load texture cubemap data from a slice
pub fn load_texture_cubemap(data: &[u8], size: i32, format: i32, mipmap_count: i32) -> u32 {
    unsafe { ffi::rlLoadTextureCubemap(data.as_ptr() as *const c_void, size, format, mipmap_count) }
}

/// Update texture with new data on GPU
pub fn update_texture(
    id: u32,
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32,
    format: i32,
    data: &[u8],
) {
    unsafe {
        ffi::rlUpdateTexture(
            id,
            offset_x,
            offset_y,
            width,
            height,
            format,
            data.as_ptr() as *const c_void,
        )
    }
}

/// Get OpenGL internal formats
pub fn get_gl_texture_formats(format: i32) -> (u32, u32, u32) {
    let mut gl_internal_format = 0;
    let mut gl_format = 0;
    let mut gl_type = 0;
    unsafe {
        ffi::rlGetGlTextureFormats(
            format,
            &mut gl_internal_format,
            &mut gl_format,
            &mut gl_type,
        );
    }
    (gl_internal_format, gl_format, gl_type)
}

/// Get name string for pixel format
pub fn get_pixel_format_name(format: u32) -> &'static CStr {
    unsafe { CStr::from_ptr(ffi::rlGetPixelFormatName(format)) }
}

/// Unload texture from GPU memory
pub fn unload_texture(id: u32) {
    unsafe { ffi::rlUnloadTexture(id) }
}

/// Generate mipmap data for selected texture
pub fn gen_texture_mipmaps(id: u32, width: i32, height: i32, format: i32, mipmaps: &mut i32) {
    unsafe { ffi::rlGenTextureMipmaps(id, width, height, format, mipmaps) }
}

/// Read texture pixel data into a Vec
pub fn read_texture_pixels(id: u32, width: i32, height: i32, format: i32) -> Option<Vec<u8>> {
    // This is complex because rlReadTexturePixels allocates memory that must be freed.
    // We'll copy it to a Rust-managed Vec and free the C-pointer immediately.
    let data_ptr = unsafe { ffi::rlReadTexturePixels(id, width, height, format) };
    if data_ptr.is_null() {
        return None;
    }
    // Calculate size based on format - this is a simplification.
    // A robust solution would use a helper function like raylib's GetPixelDataSize.
    // For now, let's assume we can calculate it or we get it from an Image struct.
    // Let's assume you have a way to get the size. For this example, I'll hardcode a dummy size.
    // In a real binding, you'd call GetPixelDataSize(width, height, format).
    // let size = GetPixelDataSize(width, height, format);
    let size = (width * height * 4) as usize; // Example for a 32-bit format
    let data_slice = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size) };
    let vec = data_slice.to_vec();
    unsafe { ffi::MemFree(data_ptr) };
    Some(vec)
}

/// Read screen pixel data (color buffer) into a Vec
pub fn read_screen_pixels(width: i32, height: i32) -> Vec<u8> {
    let data_ptr = unsafe { ffi::rlReadScreenPixels(width, height) };
    let size = (width * height * 4) as usize; // Screen is always RGBA8888
    let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, size) };
    let vec = data_slice.to_vec();
    unsafe { ffi::MemFree(data_ptr as *mut c_void) };
    vec
}

// Framebuffer management (fbo)
//---------------------------------

/// Load an empty framebuffer
pub fn load_framebuffer() -> u32 {
    unsafe { ffi::rlLoadFramebuffer() }
}

/// Attach texture/renderbuffer to a framebuffer
pub fn framebuffer_attach(
    fbo_id: u32,
    tex_id: u32,
    attach_type: i32,
    tex_type: i32,
    mip_level: i32,
) {
    unsafe { ffi::rlFramebufferAttach(fbo_id, tex_id, attach_type, tex_type, mip_level) }
}

/// Verify framebuffer is complete. Returns true if complete.
pub fn framebuffer_complete(id: u32) -> bool {
    unsafe { ffi::rlFramebufferComplete(id) }
}

/// Delete framebuffer from GPU
pub fn unload_framebuffer(id: u32) {
    unsafe { ffi::rlUnloadFramebuffer(id) }
}

// Shaders management
//---------------------------------

/// Load shader from code strings
pub fn load_shader_code(vs_code: &CStr, fs_code: &CStr) -> u32 {
    unsafe { ffi::rlLoadShaderCode(vs_code.as_ptr(), fs_code.as_ptr()) }
}

/// Compile custom shader and return shader id
pub fn compile_shader(shader_code: &CStr, shader_type: i32) -> u32 {
    unsafe { ffi::rlCompileShader(shader_code.as_ptr(), shader_type) }
}

/// Load custom shader program
pub fn load_shader_program(v_shader_id: u32, f_shader_id: u32) -> u32 {
    unsafe { ffi::rlLoadShaderProgram(v_shader_id, f_shader_id) }
}

/// Unload shader program
pub fn unload_shader_program(id: u32) {
    unsafe { ffi::rlUnloadShaderProgram(id) }
}

/// Get shader location uniform
pub fn get_location_uniform(shader_id: u32, uniform_name: &CStr) -> i32 {
    unsafe { ffi::rlGetLocationUniform(shader_id, uniform_name.as_ptr()) }
}

/// Get shader location attribute
pub fn get_location_attrib(shader_id: u32, attrib_name: &CStr) -> i32 {
    unsafe { ffi::rlGetLocationAttrib(shader_id, attrib_name.as_ptr()) }
}

/// Set shader value uniform
pub fn set_uniform<T>(loc_index: i32, value: &[T], uniform_type: i32) {
    let count = value.len() as i32;
    unsafe {
        ffi::rlSetUniform(
            loc_index,
            value.as_ptr() as *const c_void,
            uniform_type,
            count,
        )
    }
}

/// Set shader value matrix
pub fn set_uniform_matrix(loc_index: i32, mat: Matrix) {
    unsafe { ffi::rlSetUniformMatrix(loc_index, mat) }
}

/// Set shader value matrices
pub fn set_uniform_matrices(loc_index: i32, mat: &[Matrix]) {
    let count = mat.len() as i32;
    unsafe { ffi::rlSetUniformMatrices(loc_index, mat.as_ptr(), count) }
}

/// Set shader value sampler
pub fn set_uniform_sampler(loc_index: i32, texture_id: u32) {
    unsafe { ffi::rlSetUniformSampler(loc_index, texture_id) }
}

/// Set shader currently active (id and locations)
pub fn set_shader(id: u32, locs: &[i32]) {
    unsafe { ffi::rlSetShader(id, locs.as_ptr() as *mut i32) }
}

// Compute shader management
//---------------------------------

/// Load compute shader program
pub fn load_compute_shader_program(shader_id: u32) -> u32 {
    unsafe { ffi::rlLoadComputeShaderProgram(shader_id) }
}

/// Dispatch compute shader
pub fn compute_shader_dispatch(group_x: u32, group_y: u32, group_z: u32) {
    unsafe { ffi::rlComputeShaderDispatch(group_x, group_y, group_z) }
}

// Shader buffer storage object management (ssbo)
//---------------------------------

/// Load shader storage buffer object (SSBO) from a slice of data.
pub fn load_shader_buffer<T>(data: &[T], usage_hint: i32) -> u32 {
    let size = (data.len() * std::mem::size_of::<T>()) as u32;
    unsafe { ffi::rlLoadShaderBuffer(size, data.as_ptr() as *const c_void, usage_hint) }
}

/// Unload shader storage buffer object (SSBO)
pub fn unload_shader_buffer(ssbo_id: u32) {
    unsafe { ffi::rlUnloadShaderBuffer(ssbo_id) }
}

/// Update SSBO buffer data
pub fn update_shader_buffer<T>(id: u32, data: &[T], offset: u32) {
    let data_size = (data.len() * std::mem::size_of::<T>()) as u32;
    unsafe { ffi::rlUpdateShaderBuffer(id, data.as_ptr() as *const c_void, data_size, offset) }
}

/// Bind SSBO buffer
pub fn bind_shader_buffer(id: u32, index: u32) {
    unsafe { ffi::rlBindShaderBuffer(id, index) }
}

/// Read SSBO buffer data (GPU->CPU) into a mutable slice.
pub fn read_shader_buffer<T>(id: u32, dest: &mut [T], offset: u32) {
    let count = (dest.len() * std::mem::size_of::<T>()) as u32;
    unsafe { ffi::rlReadShaderBuffer(id, dest.as_mut_ptr() as *mut c_void, count, offset) }
}

/// Copy SSBO data between buffers
pub fn copy_shader_buffer(
    dest_id: u32,
    src_id: u32,
    dest_offset: u32,
    src_offset: u32,
    count: u32,
) {
    unsafe { ffi::rlCopyShaderBuffer(dest_id, src_id, dest_offset, src_offset, count) }
}

/// Get SSBO buffer size
pub fn get_shader_buffer_size(id: u32) -> u32 {
    unsafe { ffi::rlGetShaderBufferSize(id) }
}

// Buffer management
//---------------------------------

/// Bind image texture
pub fn bind_image_texture(id: u32, index: u32, format: i32, readonly: bool) {
    unsafe { ffi::rlBindImageTexture(id, index, format, readonly) }
}

// Matrix state management
//---------------------------------

/// Get internal modelview matrix
pub fn get_matrix_modelview() -> Matrix {
    unsafe { ffi::rlGetMatrixModelview() }
}

/// Get internal projection matrix
pub fn get_matrix_projection() -> Matrix {
    unsafe { ffi::rlGetMatrixProjection() }
}

/// Get internal accumulated transform matrix
pub fn get_matrix_transform() -> Matrix {
    unsafe { ffi::rlGetMatrixTransform() }
}

/// Get internal projection matrix for stereo render (selected eye)
pub fn get_matrix_projection_stereo(eye: i32) -> Matrix {
    unsafe { ffi::rlGetMatrixProjectionStereo(eye) }
}

/// Get internal view offset matrix for stereo render (selected eye)
pub fn get_matrix_view_offset_stereo(eye: i32) -> Matrix {
    unsafe { ffi::rlGetMatrixViewOffsetStereo(eye) }
}

/// Set a custom projection matrix
pub fn set_matrix_projection(proj: Matrix) {
    unsafe { ffi::rlSetMatrixProjection(proj) }
}

/// Set a custom modelview matrix
pub fn set_matrix_modelview(view: Matrix) {
    unsafe { ffi::rlSetMatrixModelview(view) }
}

/// Set eyes projection matrices for stereo rendering
pub fn set_matrix_projection_stereo(right: Matrix, left: Matrix) {
    unsafe { ffi::rlSetMatrixProjectionStereo(right, left) }
}

/// Set eyes view offsets matrices for stereo rendering
pub fn set_matrix_view_offset_stereo(right: Matrix, left: Matrix) {
    unsafe { ffi::rlSetMatrixViewOffsetStereo(right, left) }
}

// Quick and dirty cube/quad buffers load->draw->unload
//---------------------------------

/// Load and draw a cube
pub fn load_draw_cube() {
    unsafe { ffi::rlLoadDrawCube() }
}

/// Load and draw a quad
pub fn load_draw_quad() {
    unsafe { ffi::rlLoadDrawQuad() }
}
