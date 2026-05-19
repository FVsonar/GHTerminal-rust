fn main() {
    let frontend_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("frontend");
    let dist = frontend_dir.join("dist");

    // 如果 dist 目录已存在，跳过构建
    if dist.exists() {
        println!("cargo:warning=Frontend dist/ exists, skipping npm build");
    } else {
        println!("cargo:warning=Building frontend...");
        let status = std::process::Command::new("npm")
            .args(["run", "build"])
            .current_dir(&frontend_dir)
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(s) => {
                println!("cargo:warning=npm build exited with code {}", s.code().unwrap_or(-1));
            }
            Err(e) => {
                println!("cargo:warning=npm build failed: {e}. Try: cd frontend && npm install && npm run build");
            }
        }
    }

    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("index.html").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("package.json").display()
    );
}
