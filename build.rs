extern crate bindgen;
extern crate glob;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_dependencies();

    println!("cargo:rerun-if-changed=examples");

    let (vs_targets, fs_targets) = shader_targets();

    compile_example_shaders("vs", &vs_targets);
    compile_example_shaders("fs", &fs_targets);
}

fn build_dependencies() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let first_div = target.find('-').unwrap();
    let last_div = target.rfind('-').unwrap();

    let arch = &target[..first_div];
    let platform = &target[(first_div + 1)..last_div];
    let compiler = &target[(last_div + 1)..];
    let bitness = if arch == "x86_64" { 64 } else { 32 };

    let current_path = env::current_dir().expect("Failed to get current directory");

    let mut submodules_path = PathBuf::from(env::current_dir().unwrap());
    submodules_path.push("submodules");
    env::set_current_dir(&submodules_path).is_ok();

    match compiler {
        "msvc" => build_msvc(bitness),
        "gnu" | "darwin" => build_gmake(bitness, &profile, platform),
        _ => panic!("Unsupported compiler"),
    }

    env::set_current_dir(&current_path).is_ok();

    bindgen();
}

/// Builds the bgfx binaries for `msvc` targets.
fn build_msvc(bitness: u32) {
    let vs_version = env::var("VisualStudioVersion").expect("Visual Studio version not detected");
    let platform = if bitness == 32 { "X86" } else { "X64" };

    let vs_release = match vs_version.as_ref() {
        "12.0" => "2013",
        "14.0" => "2015",
        "15.0" => "2017",
        _ => panic!(format!("Unknown Visual Studio version: {:?}", vs_version)),
    };

    Command::new("bx/tools/bin/windows/genie.exe")
        .current_dir("bgfx")
        .arg("--with-dynamic-runtime")
        .arg("--with-tools")
        .arg(format!("vs{}", vs_release))
        .output()
        .expect("Failed to generate project files");

    let status = Command::new("MSBuild.exe")
        .current_dir("bgfx")
        .arg("/p:Configuration=Release")
        .arg(format!("/p:Platform={}", platform))
        .arg(format!(".build/projects/vs{}/bgfx.sln", vs_release))
        .status()
        .expect("Failed to build bgfx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx");
    }

    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(format!("win{}_vs{}", bitness, vs_release));
    path.push("bin");

    // must copy tool binaries to expected location
    let tool_src = format!("{}\\shadercRelease.exe", path.as_os_str().to_str().unwrap());
    println!("SHADERC SRC = {}", tool_src);
    let mut tool_dst = PathBuf::from(env::current_dir().unwrap());
    tool_dst.push("bgfx");
    tool_dst.push("tools");
    tool_dst.push("bin");
    tool_dst.push("windows");
    tool_dst.push("shaderc.exe");
    println!("SHADERC DST = {}", tool_dst.as_os_str().to_str().unwrap());
    fs::copy(tool_src, tool_dst).unwrap();

    println!("cargo:rustc-link-lib=static=bxRelease");
    println!("cargo:rustc-link-lib=static=bimgRelease");
    println!("cargo:rustc-link-lib=static=bgfxRelease");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=user32");
    println!(
        "cargo:rustc-link-search=native={}",
        path.as_os_str().to_str().unwrap()
    );
}

/// Builds the bgfx binaries for makefile based targets.
fn build_gmake(bitness: u32, profile: &str, platform: &str) {
    let project_name = match platform {
        "pc-windows" => "gmake-mingw-gcc",
        "unknown-linux" => "gmake-linux",
        "apple" => "gmake-osx",
        _ => panic!("Unsupported OS"),
    };

    let output_name = match platform {
        "pc-windows" => format!("win{}_mingw-gcc", bitness),
        "unknown-linux" => format!("linux{}_gcc", bitness),
        "apple" => format!("osx{}_clang", bitness),
        _ => unreachable!(),
    };

    // Generate makefiles
    let status = Command::new("make")
        .arg("-C")
        .arg("bgfx")
        .arg(format!(".build/projects/{}", project_name))
        .status()
        .expect("Failed to generate makefiles");

    if status.code().unwrap() != 0 {
        panic!("Failed to generate makefiles.");
    }

    // C flags
    let cflags = if platform == "pc-windows" && bitness == 32 {
        "-fPIC -DBGFX_CONFIG_MULTITHREADED=1 -mincoming-stack-boundary=2"
    } else {
        "-fPIC -DBGFX_CONFIG_MULTITHREADED=1"
    };

    // Build bgfx
    let status = Command::new("make")
        .env("CFLAGS", cflags)
        .arg("-R")
        .arg("-C")
        .arg(format!("bgfx/.build/projects/{}", project_name))
        .arg(format!("config={}{}", profile, bitness))
        .arg("verbose=1")
        .arg("bgfx")
        .status()
        .expect("Failed to build bgfx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx.");
    }

    // Build bimg
    let status = Command::new("make")
        .env("CFLAGS", cflags)
        .arg("-R")
        .arg("-C")
        .arg(format!("bgfx/.build/projects/{}", project_name))
        .arg(format!("config={}{}", profile, bitness))
        .arg("verbose=1")
        .arg("bimg")
        .status()
        .expect("Failed to build bimg");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bimg.");
    }

    // Build tools
    let status = Command::new("make")
        .current_dir("bgfx")
        .arg("shaderc")
        .arg("geometryc")
        .arg("texturec")
        .status()
        .expect("Failed to build tools");
    if status.code().unwrap() != 0 {
        panic!("Failed to build tools.");
    }

    // Output linker config
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(output_name);
    path.push("bin");

    let config = if profile == "debug" {
        "Debug"
    } else {
        "Release"
    };
    println!("cargo:rustc-link-lib=bgfx{}", config);
    println!("cargo:rustc-link-lib=bimg{}", config);
    println!("cargo:rustc-link-lib=bx{}", config);
    println!("cargo:rustc-link-lib=stdc++");
    println!(
        "cargo:rustc-link-search=native={}",
        path.as_os_str().to_str().unwrap()
    );

    match platform {
        "pc-windows" => {
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=opengl32");
            println!("cargo:rustc-link-lib=psapi");
        }
        "unknown-linux" => {
            println!("cargo:rustc-link-lib=GL");
            println!("cargo:rustc-link-lib=X11");
        }
        "apple" => {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=OpenGL");

            if should_link_metal() {
                println!("cargo:rustc-link-lib=framework=Metal");
            }
        }
        _ => unreachable!(),
    }
}

/// Determines whether we should link with Metal on OSX. The Metal framework
/// is only included starting with OSX 10.11. We do this through the C
/// compiler so we can test the same macro bgfx tests for support with.
fn should_link_metal() -> bool {
    let test = "#ifdef __ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__\nv=__ENVIRONMENT_MAC_OS_X_VER\
                SION_MIN_REQUIRED__\n#else\nv=1\n#endif";

    let mut cc = Command::new("cc")
        .arg("-xc")
        .arg("-E")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = cc.stdin.take().unwrap();
        stdin.write_fmt(format_args!("{}", test)).unwrap();
    }

    let output = cc.wait_with_output().unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();
    let ver_line = output_str.lines().find(|l| l.starts_with("v=")).unwrap();
    let ver_str = &ver_line[2..];
    let ver = ver_str.parse::<u32>().unwrap();

    ver >= 101100
}

fn bindgen() {
    let bindings = bindgen::builder()
        .no_unstable_rust()
        .constified_enum("bgfx_renderer_type")
        .constified_enum("bgfx_attrib")
        .constified_enum("bgfx_render_frame")
        .header("submodules/bgfx/include/bgfx/c99/platform.h")
        .clang_arg("-Isubmodules/bgfx/include")
        .clang_arg("-Isubmodules/bx/include")
        .clang_arg("-include/bgfx/c99/bgfx.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}

fn compile_example_shaders(stype: &str, targets: &Vec<ShaderTarget>) {
    let pattern = format!("examples/**/{}_*.sc", stype);
    for entry in glob::glob(&pattern).unwrap() {
        let input_path = entry.unwrap();
        let example_dir = input_path.parent().unwrap();
        println!("cargo:rerun-if-changed={}", example_dir.to_str().unwrap());

        for target in targets {
            let out_dir = format!("{}/out/{}", example_dir.to_str().unwrap(), target.name);
            fs::create_dir_all(&Path::new(&out_dir)).expect("Failed to create output directory");

            let input_filename = input_path.file_stem().unwrap().to_str().unwrap();
            let out_file = format!("{}/{}.bin", out_dir, input_filename);
            compile_shader(input_path.to_str().unwrap(), &out_file, target);
        }
    }
}

fn compile_shader(input_file: &str, output_file: &str, target: &ShaderTarget) {
    let mut command = shaderc_command();
    command
        .arg("-f")
        .arg(input_file)
        .arg("-o")
        .arg(output_file)
        .arg("--type")
        .arg(&target.stype)
        .arg("--platform")
        .arg(&target.platform);

    if let &Some(ref profile) = &target.profile {
        command.arg("-p").arg(profile);
    }
    if let &Some(ref optimization) = &target.optimization {
        command.arg("-O").arg(optimization);
    }

    println!("input_file: {}, output_file: {}", input_file, output_file);

    let output = command.output().expect("Failed to compile shader");

    writeln!(
        std::io::stderr(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    ).unwrap();
    assert!(output.status.success());

    println!("cargo:rerun-if-changed={}", input_file);
    println!("cargo:rerun-if-changed={}", output_file);
}

#[cfg(target_os = "windows")]
fn shaderc_command() -> Command {
    return Command::new("submodules/bgfx/tools/bin/windows/shaderc.exe");
}

#[cfg(target_os = "linux")]
fn shaderc_command() -> Command {
    return Command::new("submodules/bgfx/tools/bin/linux/shaderc");
}

#[cfg(target_os = "macos")]
fn shaderc_command() -> Command {
    return Command::new("submodules/bgfx/tools/bin/darwin/shaderc");
}

struct ShaderTarget {
    stype: String,
    name: String,
    platform: String,
    profile: Option<String>,
    optimization: Option<String>,
}

impl ShaderTarget {
    fn new(
        stype: &str,
        name: &str,
        platform: &str,
        profile: Option<&str>,
        optimization: Option<&str>,
    ) -> ShaderTarget {
        let profile = match profile {
            Some(str) => Some(String::from(str)),
            None => None,
        };
        let optimization = match optimization {
            Some(str) => Some(String::from(str)),
            None => None,
        };
        return ShaderTarget {
            stype: String::from(stype),
            name: String::from(name),
            platform: String::from(platform),
            profile: profile,
            optimization: optimization,
        };
    }
}

fn shader_targets() -> (Vec<ShaderTarget>, Vec<ShaderTarget>) {
    // Targets based on submodules/bgfx/scripts/shader.mk
    let mut vs_targets: Vec<ShaderTarget> = vec![
        ShaderTarget::new("vertex", "essl", "nacl", None, None),
        ShaderTarget::new("vertex", "android", "android", None, None),
        ShaderTarget::new("vertex", "gl", "linux", Some("120"), None),
        ShaderTarget::new("vertex", "metal", "osx", Some("metal"), None),
        ShaderTarget::new("vertex", "vulkan", "linux", Some("spirv"), None),
    ];
    let mut fs_targets: Vec<ShaderTarget> = vec![
        ShaderTarget::new("fragment", "nacl", "nacl", None, None),
        ShaderTarget::new("fragment", "android", "android", None, None),
        ShaderTarget::new("fragment", "gl", "linux", Some("120"), None),
        ShaderTarget::new("fragment", "metal", "osx", Some("metal"), None),
        ShaderTarget::new("fragment", "vulkan", "linux", Some("spirv"), None),
    ];
    let (additional_vs_targets, additional_fs_targets) = additional_shader_targets();
    vs_targets.extend(additional_vs_targets);
    fs_targets.extend(additional_fs_targets);
    return (vs_targets, fs_targets);
}

#[cfg(target_os = "windows")]
fn additional_shader_targets() -> (Vec<ShaderTarget>, Vec<ShaderTarget>) {
    // Targets based on submodules/bgfx/scripts/shader.mk
    let vs_targets = vec![
        ShaderTarget::new("vertex", "dx9", "windows", Some("vs_3_0"), Some("3")),
        ShaderTarget::new("vertex", "dx11", "windows", Some("vs_4_0"), Some("3")),
    ];
    let fs_targets = vec![
        ShaderTarget::new("fragment", "dx9", "windows", Some("ps_3_0"), Some("3")),
        ShaderTarget::new("fragment", "dx11", "windows", Some("ps_4_0"), Some("3")),
    ];
    return (vs_targets, fs_targets);
}

#[cfg(not(target_os = "windows"))]
fn additional_shader_targets() -> (Vec<ShaderTarget>, Vec<ShaderTarget>) {
    return (vec![], vec![]);
}
