use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string());

    match target_os.as_str() {
        "windows" => {
            // 1. Check if user provided a path via environment variable
            if let Ok(path) = env::var("OPENCL_LIB_PATH") {
                println!("cargo:rustc-link-search=native={}", path);
            } else {
                // 2. Search common SDK locations
                let candidates = vec![
                    // NVIDIA CUDA
                    env::var("CUDA_PATH").ok().map(|p| PathBuf::from(p).join("lib").join("x64")),
                    // Intel
                    Some(PathBuf::from("C:\\Program Files (x86)\\Common Files\\Intel\\Shared Libraries\\lib")),
                    // AMD
                    env::var("AMDAPPSDKROOT").ok().map(|p| PathBuf::from(p).join("lib").join("x86_64")),
                    // vcpkg / generic
                    Some(PathBuf::from("C:\\vcpkg\\packages\\opencl_x64-windows\\lib")),
                ];

                let mut found = false;
                for candidate in candidates.into_iter().flatten() {
                    if candidate.exists() && candidate.join("OpenCL.lib").exists() {
                        println!("cargo:rustc-link-search=native={}", candidate.display());
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("cargo:warning=OpenCL.lib not found in common paths. If linking fails, set OPENCL_LIB_PATH.");
                }
            }
            println!("cargo:rustc-link-lib=OpenCL");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=framework=OpenCL");
        }
        _ => {
            // Linux and others
            println!("cargo:rustc-link-lib=OpenCL");
        }
    }
}
