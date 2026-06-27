use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=ui-src/input.css");
    println!("cargo:rerun-if-changed=ui/index.html");

    // Locate the Tailwind standalone binary for the current platform.
    // Download the right binary and drop it next to Cargo.toml:
    //   macOS arm64  → tailwindcss-macos-arm64
    //   macOS x86_64 → tailwindcss-macos-x64
    //   Linux arm64  → tailwindcss-linux-arm64
    //   Linux x86_64 → tailwindcss-linux-x64
    //   Windows x64  → tailwindcss-windows-x64.exe
    let binary = if cfg!(target_os = "windows") {
        "tailwindcss-windows-x64.exe"
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "tailwindcss-macos-arm64"
        } else {
            "tailwindcss-macos-x64"
        }
    } else if cfg!(target_arch = "aarch64") {
        "tailwindcss-linux-arm64"
    } else {
        "tailwindcss-linux-x64"
    };

    // Generic fallback: a binary simply named "tailwindcss" in src-tauri/
    let path = if Path::new(binary).exists() {
        binary
    } else if Path::new("tailwindcss").exists() {
        "tailwindcss"
    } else {
        tauri_build::build();
        return;
    };

    let status = std::process::Command::new(path)
        .args(["-i", "ui-src/input.css", "-o", "ui/styles.css", "--minify"])
        .status();

    if let Ok(s) = status {
        if !s.success() {
            eprintln!("cargo:warning=Tailwind CSS build failed");
        }
    }

    tauri_build::build()
}
