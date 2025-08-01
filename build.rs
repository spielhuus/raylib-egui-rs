use std::{collections::HashSet, env, error::Error};

extern crate bindgen;

use bindgen::callbacks::DeriveTrait;
use bindgen::callbacks::ImplementsTrait;
use bindgen::callbacks::ParseCallbacks;
use std::path::{Path, PathBuf};

use cmake::Config;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Platform {
    Web,
    Desktop,
    Android,
    RPI, // raspberry pi
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlatformOS {
    Windows,
    Linux,
    BSD,
    OSX,
    Unknown,
}

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

fn uname() -> String {
    use std::process::Command;
    String::from_utf8_lossy(
        &Command::new("uname")
            .output()
            .expect("failed to run uname")
            .stdout,
    )
    .trim()
    .to_owned()
}

fn platform_from_target(target: &str) -> (Platform, PlatformOS) {
    let platform = if target.contains("wasm") {
        Platform::Web
    } else if target.contains("armv7-unknown-linux") {
        Platform::RPI
    } else if target.contains("android") {
        Platform::Android
    } else {
        Platform::Desktop
    };

    let platform_os = if platform == Platform::Desktop {
        // Determine PLATFORM_OS in case PLATFORM_DESKTOP selected
        if env::var("OS")
            .unwrap_or("".to_owned())
            .contains("Windows_NT")
            || env::var("TARGET")
                .unwrap_or("".to_owned())
                .contains("windows")
        {
            // No uname.exe on MinGW!, but OS=Windows_NT on Windows!
            // ifeq ($(UNAME),Msys) -> Windows
            PlatformOS::Windows
        } else {
            let un: &str = &uname();
            match un {
                "Linux" => PlatformOS::Linux,
                "FreeBSD" => PlatformOS::BSD,
                "OpenBSD" => PlatformOS::BSD,
                "NetBSD" => PlatformOS::BSD,
                "DragonFly" => PlatformOS::BSD,
                "Darwin" => PlatformOS::OSX,
                _ => panic!("Unknown platform {}", uname()),
            }
        }
    } else if matches!(platform, Platform::RPI | Platform::Android) {
        let un: &str = &uname();
        if un == "Linux" {
            PlatformOS::Linux
        } else {
            PlatformOS::Unknown
        }
    } else {
        PlatformOS::Unknown
    };

    (platform, platform_os)
}

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

#[derive(Debug)]
struct TypeOverrideCallback;

impl ParseCallbacks for TypeOverrideCallback {
    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        const OK_TRAITS: [DeriveTrait; 3] = [
            DeriveTrait::Copy,
            DeriveTrait::Debug,
            DeriveTrait::PartialEqOrPartialOrd,
        ];
        let overridden_types = [
            "Vector2",
            "Vector3",
            "Vector4",
            "Matrix",
            "Quaternion",
            "Rectangle",
            "Color",
        ];

        (OK_TRAITS.contains(&derive_trait) && overridden_types.contains(&name))
            .then_some(ImplementsTrait::Yes)
    }
}

fn gen_bindings() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let (platform, os) = platform_from_target(&target);

    let plat = match platform {
        Platform::Desktop => "-DPLATFORM_DESKTOP",
        Platform::RPI => "-DPLATFORM_RPI",
        Platform::Android => "-DPLATFORM_ANDROID",
        Platform::Web => "-DPLATFORM_WEB",
    };

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    let raylib_path = "./raylib";
    let raylib_header_path = PathBuf::from(&raylib_path).join("src");

    // Link to the compiled raylib library
    // println!("cargo:rustc-link-search=native={}", raylib_header_path.display());
    // println!("cargo:rustc-link-lib=static=raylib");

    let mut builder = bindgen::Builder::default()
        .header("binding/binding.h")
        .rustified_enum(".+")
        .derive_partialeq(true)
        .derive_default(true)
        .generate_inline_functions(true)
        .blocklist_type("Vector2")
        .blocklist_type("Vector3")
        .blocklist_type("Vector4")
        .blocklist_type("Matrix")
        .blocklist_type("Quaternion")
        .blocklist_type("Rectangle")
        .blocklist_type("Color")
        .parse_callbacks(Box::new(TypeOverrideCallback))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{}", raylib_header_path.display()))
        .clang_arg("-std=c99")
        .clang_arg(plat)
        .parse_callbacks(Box::new(ignored_macros));

    #[cfg(feature = "raygui")]
    {
        let raygui_path = "./raygui";
        let raygui_header_path = PathBuf::from(&raygui_path).join("src");
        builder = builder
            .clang_arg(format!("-I{}", raygui_header_path.display()))
            .clang_arg("-DRAYGUI_IMPLEMENTATION");
    }

    if target.contains("emscripten") {
        let emsdk = env::var("EMSDK").unwrap();
        let emscripten_sys_include =
            PathBuf::from(emsdk).join("upstream/emscripten/cache/sysroot/include");

        // Tell bindgen to use the Emscripten sysroot headers
        builder = builder
            .clang_arg(format!(
                "--sysroot={}",
                emscripten_sys_include.parent().unwrap().display()
            ))
            .clang_arg(format!("-I{}", emscripten_sys_include.display()));
    }

    if platform == Platform::Desktop && os == PlatformOS::Windows {
        // odd workaround for booleans being broken
        builder = builder.clang_arg("-D__STDC__");
    }

    if platform == Platform::Web {
        builder = builder
            .clang_arg("-fvisibility=default")
            .clang_arg("--target=wasm32-emscripten");
    }

    // Build
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_with_cmake(src_path: &str) {
    // CMake uses different lib directories on different systems.
    // I do not know how CMake determines what directory to use,
    // so we will check a few possibilities and use whichever is present.
    fn join_cmake_lib_directory(path: PathBuf) -> PathBuf {
        let possible_cmake_lib_directories = ["lib", "lib64", "lib32"];
        for lib_directory in &possible_cmake_lib_directories {
            let lib_path = path.join(lib_directory);
            if lib_path.exists() {
                return lib_path;
            }
        }
        path
    }

    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    let (platform, platform_os) = platform_from_target(&target);

    let mut conf = cmake::Config::new(src_path);

    let emsdk = env::var("EMSDK").expect("EMSDK env var not set. Have you sourced emsdk_env.sh?");
    let emscripten_cmake =
        PathBuf::from(emsdk).join("upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake");
    if target == "wasm32-unknown-emscripten" {
        // conf.define("PLATFORM", "Web");
        conf.define("CMAKE_TOOLCHAIN_FILE", emscripten_cmake);
    }

    let profile;
    #[cfg(debug_assertions)]
    {
        // note: For some extra perf in debug mode, you can change this to Release by default since we're building a static lib ONCE, while your game is in debug, so you get the best of ~both worlds(high perf drawing, debugger info for the rust code)
        profile = "Debug";
    }
    #[cfg(not(debug_assertions))]
    {
        profile = "Release";
    }
    #[cfg(feature = "release_with_debug_info")]
    {
        profile = "RelWithDebInfo"
    }
    #[cfg(feature = "min_size_rel")]
    {
        profile = "MinSizeRel"
    }

    let builder = conf.profile(profile);

    builder.define("CMAKE_BUILD_TYPE", profile);
    // NOTE(WINDOWS ONLY, linux is fine): Custom builds that turn off modules might not link because the linker tries to find the function def which windowss sometimes does not optimize out
    // The correct/safe way would be to edit the raylib-rs bindings and feature flag remove calls to things you are opting out of(preferably making them stub and do nothing but still exist) but to atm to fix this dangerously(e.g UB when you call LoadSound() while SUPPORT_MODULE_RAUDIO OFF), create a `build.rs` script for in mygameproject and add the following:
    //`println!("cargo:rustc-link-arg=/FORCE:UNRESOLVED");`
    builder
        .define("BUILD_EXAMPLES", "OFF")
        .define("CUSTOMIZE_BUILD", "ON"); // MUST BE ON or else it'll ignore all other flags
    features_from_env(builder);

    // Scope implementing flags for forcing OpenGL version
    // See all possible flags at https://github.com/raysan5/raylib/wiki/CMake-Build-Options
    {
        #[cfg(feature = "opengl_43")]
        builder.define("OPENGL_VERSION", "4.3");

        #[cfg(feature = "opengl_33")]
        builder.define("OPENGL_VERSION", "3.3");

        #[cfg(feature = "opengl_21")]
        builder.define("OPENGL_VERSION", "2.1");

        #[cfg(feature = "opengl_11")]
        builder.define("OPENGL_VERSION", "1.1");

        #[cfg(feature = "opengl_es_20")]
        {
            builder.define("OPENGL_VERSION", "ES 2.0");
            println!("cargo:rustc-link-lib=GLESv2");
            println!("cargo:rustc-link-lib=GLdispatch");
        }

        #[cfg(feature = "opengl_es_30")]
        {
            builder.define("OPENGL_VERSION", "ES 3.0");
            println!("cargo:rustc-link-lib=GLESv2");
            println!("cargo:rustc-link-lib=GLdispatch");
        }
    }

    match platform {
        Platform::Desktop => {
            #[cfg(feature = "sdl")]
            {
                println!("cargo:rustc-link-lib=SDL2");
                conf.define("PLATFORM", "SDL")
            }
            #[cfg(not(feature = "sdl"))]
            {
                conf.define("PLATFORM", "Desktop")
            }
        }
        Platform::Web => conf.define("PLATFORM", "Web"),
        Platform::RPI => conf.define("PLATFORM", "Raspberry Pi"),
        Platform::Android => {
            // get required env variables
            let android_ndk_home = env::var("ANDROID_NDK_HOME")
                .expect("Please set the environment variable: ANDROID_NDK_HOME:(e.g /home/u/Android/Sdk/ndk/VXXX/)");
            let android_platform = target.split("-").last().expect("fail to parse the android version of the target triple, example:'aarch64-linux-android25'");
            let abi_version = android_platform
                .split("-")
                .last()
                .expect("Could not get abi version. Is ANDROID_PLATFORM valid?");
            let toolchain_file =
                format!("{}/build/cmake/android.toolchain.cmake", &android_ndk_home);
            // Detect ANDROID_ABI using the target triple
            let android_arch_abi = match target.as_str() {
                "aarch64-linux-android" => "arm64-v8a",
                "armv7-linux-androideabi" => "armeabi-v7a",
                _ => panic!("Unsupported target triple for Android"),
            };
            // we'll set as many variables as possible according to:
            // https://developer.android.com/ndk/guides/cmake#command-line_1
            // https://cmake.org/cmake/help/v3.31/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-the-ndk
            // how to build:
            // 0) set the correct linker in your game project's Cargo.toml (note: platform number should match):
            // [target.aarch64-linux-android]
            // linker = "/home/u/Android/Sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android<PLATFORM_NUMBER>-clang"

            // 1) set env variable ANDROID_NDK_HOME
            // 2) compile with: `cargo ndk -t <ARCH> -p <PLATFORM_NUMBER> -o ./jniLibs build`
            // example(cargo ndk -t arm64-v8a -p 25 -o ./jniLibs build)

            conf.define("CMAKE_SYSTEM_NAME", "Android")
                .define("PLATFORM", "Android")
                .define("CMAKE_SYSTEM_VERSION", abi_version)
                .define("ANDROID_ABI", android_arch_abi)
                .define("CMAKE_ANDROID_ARCH_ABI", android_arch_abi)
                .define("CMAKE_ANDROID_NDK", &android_ndk_home)
                .define("ANDROID_PLATFORM", android_platform)
                .define("CMAKE_TOOLCHAIN_FILE", &toolchain_file)
        }
    };

    let dst = conf.build();
    let dst_lib = join_cmake_lib_directory(dst);
    // on windows copy the static library to the proper file name
    if platform_os == PlatformOS::Windows {
        if Path::new(&dst_lib.join("raylib.lib")).exists() {
            // DO NOTHING
        } else if Path::new(&dst_lib.join("raylib_static.lib")).exists() {
            std::fs::copy(
                dst_lib.join("raylib_static.lib"),
                dst_lib.join("raylib.lib"),
            )
            .expect("failed to create windows library");
        } else if Path::new(&dst_lib.join("libraylib_static.a")).exists() {
            std::fs::copy(
                dst_lib.join("libraylib_static.a"),
                dst_lib.join("libraylib.a"),
            )
            .expect("failed to create windows library");
        } else if Path::new(&dst_lib.join("libraylib.a")).exists() {
            // DO NOTHING
        } else {
            panic!("failed to create windows library");
        }
    } else if platform == Platform::Web && !Path::new(&dst_lib.join("libraylib.a")).exists() {
        std::fs::copy(dst_lib.join("libraylib.bc"), dst_lib.join("libraylib.a"))
            .expect("failed to create wasm library");
    }
    // println!("cmake build {}", c.display());
    println!("cargo:rustc-link-search=native={}", dst_lib.display());
    if platform == Platform::Android {
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=android");
        println!("cargo:rustc-link-lib=EGL");
        println!("cargo:rustc-link-lib=GLESv2");
        println!("cargo:rustc-link-lib=OpenSLES");
        println!("cargo:rustc-link-lib=c");
        println!("cargo:rustc-link-lib=m");
    }
}

#[cfg(feature = "raygui")]
fn gen_rgui() {
    let target = env::var("TARGET").unwrap();

    // Compile the code and link with cc crate
    let raylib_src_path = PathBuf::from("raylib/src");
    let mut build = cc::Build::new();
    build
        .files(vec!["binding/rgui_wrapper.c"])
        .include("binding")
        .include(raylib_src_path)
        .warnings(false)
        .extra_warnings(false);

    if target.contains("emscripten") {
        let emsdk =
            env::var("EMSDK").expect("EMSDK env var not set. Have you sourced emsdk_env.sh?");
        let emscripten_sys_include =
            PathBuf::from(emsdk).join("upstream/emscripten/cache/sysroot/include");

        // Add the Emscripten system include path
        build.include(emscripten_sys_include);
    }
    build.compile("rgui");
}

fn link(platform: Platform, platform_os: PlatformOS) {
    match platform_os {
        PlatformOS::Windows => {
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=shell32");
        }
        PlatformOS::Linux => {
            // X11 linking
            #[cfg(all(not(feature = "wayland"), target_os = "android"))]
            {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=X11");
            }

            // Wayland linking
            #[cfg(feature = "wayland")]
            {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=wayland-client");
                println!("cargo:rustc-link-lib=glfw"); // Link against locally installed glfw
            }
        }
        PlatformOS::OSX => {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }
        _ => (),
    }
    if platform == Platform::Web {
        println!("cargo:rustc-link-lib=glfw");
    } else if platform == Platform::RPI {
        println!("cargo:rustc-link-search=/opt/vc/lib");
        println!("cargo:rustc-link-lib=bcm_host");
        println!("cargo:rustc-link-lib=brcmEGL");
        println!("cargo:rustc-link-lib=brcmGLESv2");
        println!("cargo:rustc-link-lib=vcos");
    }

    println!("cargo:rustc-link-lib=static=raylib");
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./binding/binding.h");
    //for cross compiling on switch arm
    //https://users.rust-lang.org/t/cross-compiling-arm/96456/10
    if std::env::var("CROSS_SYSROOT").is_ok() {
        unsafe {
            std::env::remove_var("CROSS_SYSROOT");
        }
    }
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    if target.contains("wasm32-unknown-emscripten") {
        if let Err(e) = env::var("EMCC_CFLAGS") {
            if e == std::env::VarError::NotPresent {
                panic!(
                    "\nYou must have to set EMCC_CFLAGS yourself to compile for WASM.\n{}{}\"\n",
                    {
                        #[cfg(target_family = "windows")]
                        {
                            "set EMCC_CFLAGS="
                        }
                        #[cfg(not(target_family = "windows"))]
                        {
                            "export EMCC_CFLAGS="
                        }
                    },
                    "\"-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1\""
                );
            } else {
                panic!("\nError regarding EMCC_CFLAGS: {:?}\n", e);
            }
        }
    }

    let (platform, platform_os) = platform_from_target(&target);

    let raylib_src = "./raylib";
    if is_directory_empty(raylib_src) {
        panic!("raylib source does not exist in: `raylib-sys/raylib`. Please copy it in");
    }
    build_with_cmake(raylib_src);

    gen_bindings();

    link(platform, platform_os);

    #[cfg(feature = "raygui")]
    gen_rgui();

    Ok(())
}

#[must_use]
/// returns false if the directory does not exist
fn is_directory_empty(path: &str) -> bool {
    match std::fs::read_dir(path) {
        Ok(mut dir) => dir.next().is_none(),
        Err(_) => true,
    }
}

#[rustfmt::skip]
fn features_from_env(cmake: &mut Config) {
    let is_android = cfg!(target_os = "android"); // skip linking to x11 & wayland
    cmake.define("ENABLE_ASAN", bstr(cfg!(feature = "ENABLE_ASAN")));
    cmake.define("ENABLE_UBSAN", bstr(cfg!(feature = "ENABLE_UBSAN")));
    cmake.define("ENABLE_MSAN", bstr(cfg!(feature = "ENABLE_MSAN")));
    cmake.define("WITH_PIC", bstr(cfg!(feature = "WITH_PIC")));
    cmake.define("BUILD_SHARED_LIBS", bstr(cfg!(feature = "BUILD_SHARED_LIBS")));
    cmake.define("USE_EXTERNAL_GLFW", bstr(cfg!(feature = "USE_EXTERNAL_GLFW")));
    cmake.define("GLFW_BUILD_WAYLAND", bstr(cfg!(feature = "GLFW_BUILD_WAYLAND") && !is_android));
    cmake.define("GLFW_BUILD_X11", bstr(cfg!(feature = "GLFW_BUILD_X11") && !is_android));
    cmake.define("INCLUDE_EVERYTHING", bstr(cfg!(feature = "INCLUDE_EVERYTHING")));
    cmake.define("USE_AUDIO", bstr(cfg!(feature = "USE_AUDIO")));
    cmake.define("SUPPORT_MODULE_RSHAPES", bstr(cfg!(feature = "SUPPORT_MODULE_RSHAPES")));
    cmake.define("SUPPORT_MODULE_RTEXTURES", bstr(cfg!(feature = "SUPPORT_MODULE_RTEXTURES")));
    cmake.define("SUPPORT_MODULE_RTEXT", bstr(cfg!(feature = "SUPPORT_MODULE_RTEXT")));
    cmake.define("SUPPORT_MODULE_RMODELS", bstr(cfg!(feature = "SUPPORT_MODULE_RMODELS")));
    cmake.define("SUPPORT_MODULE_RAUDIO", bstr(cfg!(feature = "SUPPORT_MODULE_RAUDIO")));
    cmake.define("SUPPORT_BUSY_WAIT_LOOP", bstr(cfg!(feature = "SUPPORT_BUSY_WAIT_LOOP")));
    cmake.define("SUPPORT_CAMERA_SYSTEM", bstr(cfg!(feature = "SUPPORT_CAMERA_SYSTEM")));
    cmake.define("SUPPORT_GESTURES_SYSTEM", bstr(cfg!(feature = "SUPPORT_GESTURES_SYSTEM")));
    cmake.define("SUPPORT_RPRAND_GENERATOR", bstr(cfg!(feature = "SUPPORT_RPRAND_GENERATOR")));
    cmake.define("SUPPORT_MOUSE_GESTURES", bstr(cfg!(feature = "SUPPORT_MOUSE_GESTURES")));
    cmake.define("SUPPORT_SSH_KEYBOARD_RPI", bstr(cfg!(feature = "SUPPORT_SSH_KEYBOARD_RPI")));
    cmake.define("SUPPORT_WINMM_HIGHRES_TIMER", bstr(cfg!(feature = "SUPPORT_WINMM_HIGHRES_TIMER")));
    cmake.define("SUPPORT_PARTIALBUSY_WAIT_LOOP", bstr(cfg!(feature = "SUPPORT_PARTIALBUSY_WAIT_LOOP")));
    cmake.define("SUPPORT_GIF_RECORDING", bstr(cfg!(feature = "SUPPORT_GIF_RECORDING")));
    cmake.define("SUPPORT_COMPRESSION_API", bstr(cfg!(feature = "SUPPORT_COMPRESSION_API")));
    cmake.define("SUPPORT_AUTOMATION_EVENTS", bstr(cfg!(feature = "SUPPORT_AUTOMATION_EVENTS")));
    cmake.define("SUPPORT_CUSTOM_FRAME_CONTROL", bstr(cfg!(feature = "SUPPORT_CUSTOM_FRAME_CONTROL")));
    cmake.define("SUPPORT_CLIPBOARD_IMAGE", bstr(cfg!(feature = "SUPPORT_CLIPBOARD_IMAGE")));
    cmake.define("SUPPORT_QUADS_DRAW_MODE", bstr(cfg!(feature = "SUPPORT_QUADS_DRAW_MODE")));
    cmake.define("SUPPORT_FILEFORMAT_PNG", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_PNG")));
    cmake.define("SUPPORT_FILEFORMAT_BMP", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_BMP")));
    cmake.define("SUPPORT_FILEFORMAT_TGA", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_TGA")));
    cmake.define("SUPPORT_FILEFORMAT_JPG", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_JPG")));
    cmake.define("SUPPORT_FILEFORMAT_GIF", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_GIF")));
    cmake.define("SUPPORT_FILEFORMAT_QOI", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_QOI")));
    cmake.define("SUPPORT_FILEFORMAT_PSD", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_PSD")));
    cmake.define("SUPPORT_FILEFORMAT_DDS", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_DDS")));
    cmake.define("SUPPORT_FILEFORMAT_HDR", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_HDR")));
    cmake.define("SUPPORT_FILEFORMAT_PIC", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_PIC")));
    cmake.define("SUPPORT_FILEFORMAT_KTX", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_KTX")));
    cmake.define("SUPPORT_FILEFORMAT_ASTC", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_ASTC")));
    cmake.define("SUPPORT_FILEFORMAT_PKM", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_PKM")));
    cmake.define("SUPPORT_FILEFORMAT_PVR", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_PVR")));
    cmake.define("SUPPORT_IMAGE_EXPORT", bstr(cfg!(feature = "SUPPORT_IMAGE_EXPORT")));
    cmake.define("SUPPORT_IMAGE_GENERATION", bstr(cfg!(feature = "SUPPORT_IMAGE_GENERATION")));
    cmake.define("SUPPORT_IMAGE_MANIPULATION", bstr(cfg!(feature = "SUPPORT_IMAGE_MANIPULATION")));
    cmake.define("SUPPORT_DEFAULT_FONT", bstr(cfg!(feature = "SUPPORT_DEFAULT_FONT")));
    cmake.define("SUPPORT_FILEFORMAT_TTF", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_TTF")));
    cmake.define("SUPPORT_FILEFORMAT_FNT", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_FNT")));
    cmake.define("SUPPORT_FILEFORMAT_BDF", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_BDF")));
    cmake.define("SUPPORT_TEXT_MANIPULATION", bstr(cfg!(feature = "SUPPORT_TEXT_MANIPULATION")));
    cmake.define("SUPPORT_FONT_ATLAS_WHITE_REC", bstr(cfg!(feature = "SUPPORT_FONT_ATLAS_WHITE_REC")));
    cmake.define("SUPPORT_FILEFORMAT_OBJ", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_OBJ")));
    cmake.define("SUPPORT_FILEFORMAT_MTL", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_MTL")));
    cmake.define("SUPPORT_FILEFORMAT_IQM", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_IQM")));
    cmake.define("SUPPORT_FILEFORMAT_GLTF", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_GLTF")));
    cmake.define("SUPPORT_FILEFORMAT_VOX", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_VOX")));
    cmake.define("SUPPORT_FILEFORMAT_M3D", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_M3D")));
    cmake.define("SUPPORT_MESH_GENERATION", bstr(cfg!(feature = "SUPPORT_MESH_GENERATION")));
    cmake.define("SUPPORT_FILEFORMAT_WAV", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_WAV")));
    cmake.define("SUPPORT_FILEFORMAT_OGG", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_OGG")));
    cmake.define("SUPPORT_FILEFORMAT_MP3", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_MP3")));
    cmake.define("SUPPORT_FILEFORMAT_QOA", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_QOA")));
    cmake.define("SUPPORT_FILEFORMAT_FLAC", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_FLAC")));
    cmake.define("SUPPORT_FILEFORMAT_XM", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_XM")));
    cmake.define("SUPPORT_FILEFORMAT_MOD", bstr(cfg!(feature = "SUPPORT_FILEFORMAT_MOD")));
    cmake.define("SUPPORT_STANDARD_FILEIO", bstr(cfg!(feature = "SUPPORT_STANDARD_FILEIO")));
    cmake.define("SUPPORT_TRACELOG", bstr(cfg!(feature = "SUPPORT_TRACELOG")));
    cmake.define("SUPPORT_SCREEN_CAPTURE", bstr(cfg!(feature = "SUPPORT_SCREEN_CAPTURE")));
    cmake.define("SUPPORT_VR_SIMULATOR", bstr(cfg!(feature = "SUPPORT_VR_SIMULATOR")));
    cmake.define("SUPPORT_DISTORTION_SHADER", bstr(cfg!(feature = "SUPPORT_DISTORTION_SHADER")));
    cmake.define("SUPPORT_FONT_TEXTURE", bstr(cfg!(feature = "SUPPORT_FONT_TEXTURE")));
}

#[must_use]
fn bstr(b: bool) -> &'static str {
    if b { "ON" } else { "OFF" }
}
