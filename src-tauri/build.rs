use std::path::Path;

fn main() {
    // Rebuild CSS whenever the source stylesheet or any UI file changes
    println!("cargo:rerun-if-changed=../ui-src/input.css");
    println!("cargo:rerun-if-changed=../ui/index.html");

    // Run Tailwind standalone CLI if the binary exists next to Cargo.toml
    let tailwind = Path::new("../tailwindcss");
    if tailwind.exists() {
        let status = std::process::Command::new(tailwind)
            .args([
                "-i", "../ui-src/input.css",
                "-o", "../ui/styles.css",
                "--content", "../ui/**/*.{html,js}",
                "--minify",
            ])
            .status();
        if let Ok(s) = status {
            if !s.success() {
                eprintln!("cargo:warning=Tailwind CSS build failed");
            }
        }
    }

    tauri_build::build()
}
