#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString, c_char};
use std::os::raw::{c_int, c_void};

pub mod color;
mod ffi;
pub mod math;
pub mod raylib;

#[cfg(feature = "raygui")]
pub mod raygui;

pub use ffi::{
    AudioStream, BoundingBox, Camera, Camera2D, Camera3D, Font, Image, KeyboardKey, Material, Mesh,
    Model, ModelAnimation, Music, NPatchInfo, Ray, RayCollision, RenderTexture2D, Shader, Sound,
    Texture2D, TextureCubemap, VrDeviceInfo, VrStereoConfig, Wave, float3, float16,
};
