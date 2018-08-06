extern crate glob;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=examples");

    let (vs_targets, fs_targets) = shader_targets();

    compile_example_shaders("vs", &vs_targets);
    compile_example_shaders("fs", &fs_targets);
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
    return Command::new("bgfx-sys/bgfx/tools/bin/windows/shaderc.exe");
}

#[cfg(target_os = "linux")]
fn shaderc_command() -> Command {
    return Command::new("bgfx-sys/bgfx/tools/bin/linux/shaderc");
}

#[cfg(target_os = "macos")]
fn shaderc_command() -> Command {
    return Command::new("bgfx-sys/bgfx/tools/bin/darwin/shaderc");
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
    // Targets based on bgfx-sys/bgfx/scripts/shader.mk
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
    // Targets based on bgfx-sys/bgfx/scripts/shader.mk
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
