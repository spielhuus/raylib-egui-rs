#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod color;
mod ffi;
pub mod math;
pub mod raylib;
pub mod rlgl;

#[cfg(feature = "raygui")]
pub mod raygui;

pub mod egui;
