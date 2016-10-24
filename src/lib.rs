#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

extern crate libc;

#[repr(C)]
pub struct glslopt_shader;
#[repr(C)]
pub struct glslopt_ctx;

pub type glslopt_shader_type = libc::c_int;
pub const kGlslOptShaderVertex: glslopt_shader_type = 0;
pub const kGlslOptShaderFragment: glslopt_shader_type = 1;

// Options flags for glsl_optimize
pub type glslopt_options = libc::c_uint;
// Skip preprocessing shader source. Saves some time if you know you don't need it.
pub const kGlslOptionSkipPreprocessor: glslopt_options = (1 << 0);
// Passed shader is not the full shader source. This makes some optimizations weaker.
pub const kGlslOptionNotFullShader: glslopt_options = (1 << 1);

// Optimizer target language
pub type glslopt_target = libc::c_int;
pub const kGlslTargetOpenGL: glslopt_target = 0;
pub const kGlslTargetOpenGLES20: glslopt_target = 1;
pub const kGlslTargetOpenGLES30: glslopt_target = 2;
pub const kGlslTargetMetal: glslopt_target = 3;

// Type info
pub type glslopt_basic_type = libc::c_int;
pub const kGlslTypeFloat: glslopt_basic_type = 0;
pub const kGlslTypeInt: glslopt_basic_type = 1;
pub const kGlslTypeBool: glslopt_basic_type = 2;
pub const kGlslTypeTex2D: glslopt_basic_type = 3;
pub const kGlslTypeTex3D: glslopt_basic_type = 4;
pub const kGlslTypeTexCube: glslopt_basic_type = 5;
pub const kGlslTypeTex2DShadow: glslopt_basic_type = 6;
pub const kGlslTypeTex2DArray: glslopt_basic_type = 7;
pub const kGlslTypeOther: glslopt_basic_type = 8;
pub const kGlslTypeCount: glslopt_basic_type = 9;

pub type glslopt_precision = libc::c_int;
pub const kGlslPrecHigh: glslopt_precision = 0;
pub const kGlslPrecMedium: glslopt_precision = 1;
pub const kGlslPrecLow: glslopt_precision = 2;
pub const kGlslPrecCount: glslopt_precision = 3;

extern "C" {
    pub fn glslopt_initialize(target: glslopt_target) -> *mut glslopt_ctx;
    pub fn glslopt_cleanup(ctx: *mut glslopt_ctx);

    pub fn glslopt_set_max_unroll_iterations(ctx: *mut glslopt_ctx, iterations: libc::c_uint);

    pub fn glslopt_optimize(ctx: *mut glslopt_ctx,
                            ty: glslopt_shader_type,
                            shaderSource: *const libc::c_char,
                            options: libc::c_uint)
                            -> *mut glslopt_shader;
    pub fn glslopt_get_status(shader: *mut glslopt_shader) -> bool;
    pub fn glslopt_get_output(shader: *mut glslopt_shader) -> *const libc::c_char;
    pub fn glslopt_get_raw_output(shader: *mut glslopt_shader) -> *const libc::c_char;
    pub fn glslopt_get_log(shader: *mut glslopt_shader) -> *const libc::c_char;
    pub fn glslopt_shader_delete(shader: *mut glslopt_shader);

    pub fn glslopt_shader_get_input_count(shader: *mut glslopt_shader) -> libc::c_int;
    pub fn glslopt_shader_get_input_desc(shader: *mut glslopt_shader,
                                         index: libc::c_int,
                                         outName: *const *mut libc::c_char,
                                         outType: *mut glslopt_basic_type,
                                         outPrec: *mut glslopt_precision,
                                         outVecSize: *mut libc::c_int,
                                         outMatSize: *mut libc::c_int,
                                         outArraySize: *mut libc::c_int,
                                         outLocation: *mut libc::c_int);
    pub fn glslopt_shader_get_uniform_count(shader: *mut glslopt_shader) -> libc::c_int;
    pub fn glslopt_shader_get_uniform_total_size(shader: *mut glslopt_shader) -> libc::c_int;
    pub fn glslopt_shader_get_uniform_desc(shader: *mut glslopt_shader,
                                           index: libc::c_int,
                                           outName: *const *mut libc::c_char,
                                           outType: *mut glslopt_basic_type,
                                           outPrec: *mut glslopt_precision,
                                           outVecSize: *mut libc::c_int,
                                           outMatSize: *mut libc::c_int,
                                           outArraySize: *mut libc::c_int,
                                           outLocation: *mut libc::c_int);
    pub fn glslopt_shader_get_texture_count(shader: *mut glslopt_shader) -> libc::c_int;
    pub fn glslopt_shader_get_texture_desc(shader: *mut glslopt_shader,
                                           index: libc::c_int,
                                           outName: *const *mut libc::c_char,
                                           outType: *mut glslopt_basic_type,
                                           outPrec: *mut glslopt_precision,
                                           outVecSize: *mut libc::c_int,
                                           outMatSize: *mut libc::c_int,
                                           outArraySize: *mut libc::c_int,
                                           outLocation: *mut libc::c_int);

    // Get *very* approximate shader stats:
    // Number of math, texture and flow control instructions.
    pub fn glslopt_shader_get_stats(shader: *mut glslopt_shader,
                                    approxMath: *mut libc::c_int,
                                    approxTex: *mut libc::c_int,
                                    approxFlow: *mut libc::c_int);
}
