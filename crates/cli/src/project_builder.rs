use std::fs;
use std::path::PathBuf;
use std::process::Command;
// use std::time::{SystemTime, UNIX_EPOCH}; // No longer used for timestamp
use colored::Colorize;
use rustx_core::ast::Stmt;
use rustx_core::compiler::transpiler::Transpiler;

pub struct ProjectBuilder;

impl ProjectBuilder {
    /// Builds and runs the project if JIT is required, otherwise standard build.
    /// If `run_after_build` is true, it runs the resulting binary.
    /// Returns the path to the executable if built, or None if failed.
    pub fn build(
        _source: &str,
        ast: &[Stmt],
        output_path: Option<PathBuf>,
        run: bool,
        verbose: bool,
    ) -> Result<(), String> {
        // 1. Transpile
        let mut transpiler = Transpiler::new();
        let code = transpiler.transpile(ast);

        // 2. Generate Hash for Caching
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);

        let mut deps_str = String::new();
        for stmt in ast {
            if let Stmt::RustImport {
                crate_name,
                version,
                ..
            } = stmt
            {
                deps_str.push_str(crate_name);
                deps_str.push_str(version);
            }
        }
        deps_str.hash(&mut hasher);

        // Also hash the core path to ensure we rebuild if core changes location (dev env)
        let core_path = std::env::current_exe()
            .ok()
            .and_then(|p| {
                p.parent()?
                    .parent()?
                    .parent()?
                    .join("crates/core")
                    .canonicalize()
                    .ok()
            })
            .unwrap_or_else(|| PathBuf::from("/home/grandpa/me/code/rust/RustX/crates/core"));
        core_path.hash(&mut hasher);

        let hash = hasher.finish();
        let binary_name = format!("jit_{:x}", hash);

        // 3. Setup Directories
        let cache_root = std::env::temp_dir().join("rustx_jit_cache");
        let binaries_dir = cache_root.join("binaries");
        let runner_dir = cache_root.join("runner"); // Single persistent project
        let shared_target_dir = cache_root.join("target");

        if !binaries_dir.exists() {
            fs::create_dir_all(&binaries_dir).unwrap();
        }

        let final_binary_path = binaries_dir.join(&binary_name);

        if verbose {
            println!(
                "{} {}",
                "JIT Binary Path:".dimmed(),
                final_binary_path.display()
            );
        }

        // 4. Execution Logic
        // If binary exists, run it immediately (FAST PATH)
        if run && final_binary_path.exists() {
            if verbose {
                println!(
                    "{} {}",
                    "Running cached binary:".dimmed(),
                    final_binary_path.display()
                );
            }
            let status = Command::new(&final_binary_path)
                .status()
                .map_err(|e| format!("Failed to run cached binary: {}", e))?;
            if !status.success() {
                return Err("Execution failed".to_string());
            }
            return Ok(());
        }

        // 5. Build Logic (SLOW PATH - but optimized by single runner)
        // Only enter here if we need to compile (or just build output without running)

        if verbose {
            println!("{}", "Compiling JIT...".bright_yellow());
        }

        if !runner_dir.exists() {
            fs::create_dir_all(&runner_dir).map_err(|e| format!("Error creating dir: {}", e))?;

            // Initialize cargo project
            let status = Command::new("cargo")
                .arg("init")
                .arg("--bin")
                .arg("--name")
                .arg("rustx_jit_runner")
                .current_dir(&runner_dir)
                .output()
                .map_err(|e| format!("Failed to run cargo init: {}", e))?;

            if !status.status.success() {
                return Err(format!(
                    "Error initializing cargo: {}",
                    String::from_utf8_lossy(&status.stderr)
                ));
            }
        }

        // Configure Cargo.toml (Idempotent - cargo handles change detection)
        let mut cargo_toml = r#"[package]
name = "rustx_jit_runner"
version = "0.1.0"
edition = "2021"

[dependencies]
"#
        .to_string();

        cargo_toml.push_str(&format!(
            "rustx_core = {{ path = \"{}\" }}\n",
            core_path.display()
        ));

        for stmt in ast {
            if let Stmt::RustImport {
                crate_name,
                version,
                ..
            } = stmt
            {
                if version.trim().starts_with('{') {
                    cargo_toml.push_str(&format!("{} = {}\n", crate_name, version));
                } else {
                    cargo_toml.push_str(&format!("{} = \"{}\"\n", crate_name, version));
                }
            }
        }

        cargo_toml.push_str("\n[profile.release]\nlto = false\npanic = \"abort\"\nopt-level = 3\n");

        fs::write(runner_dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

        // Write Source to main.rs
        fs::write(runner_dir.join("src/main.rs"), &code)
            .map_err(|e| format!("Failed to write main.rs: {}", e))?;

        // Build
        let status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--offline") // Use cached dependencies
            .env("CARGO_TARGET_DIR", &shared_target_dir)
            .current_dir(&runner_dir)
            .output()
            .map_err(|e| format!("Failed to run cargo build: {}", e))?;

        if !status.status.success() {
            return Err(format!(
                "Compilation failed:\n{}",
                String::from_utf8_lossy(&status.stderr)
            ));
        }

        // Copy resulting binary to persistent store
        // Binary name will always be 'rustx_jit_runner' in target dir
        let build_artifact = shared_target_dir.join("release").join("rustx_jit_runner");
        fs::copy(&build_artifact, &final_binary_path)
            .map_err(|e| format!("Failed to cache binary: {}", e))?;

        // 6. Run if needed
        if run {
            let status = Command::new(&final_binary_path)
                .status()
                .map_err(|e| format!("Failed to run binary: {}", e))?;
            if !status.success() {
                return Err("Execution failed".to_string());
            }
        }

        if let Some(out_path) = output_path {
            fs::copy(&final_binary_path, &out_path)
                .map_err(|e| format!("Failed to export binary: {}", e))?;
        }

        Ok(())
    }
}
