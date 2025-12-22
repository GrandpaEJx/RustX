use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use colored::Colorize;
use rustx_core::ast::Stmt;
use rustx_core::compiler::transpiler::Transpiler;

pub struct ProjectBuilder;

impl ProjectBuilder {
    /// Builds and runs the project if JIT is required, otherwise standard build.
    /// If `run_after_build` is true, it runs the resulting binary.
    /// Returns the path to the executable if built, or None if failed.
    pub fn build(_source: &str, ast: &[Stmt], output_path: Option<PathBuf>, run: bool, verbose: bool) -> Result<(), String> {
        // 1. Transpile
        let mut transpiler = Transpiler::new();
        let code = transpiler.transpile(ast);
        
        // 2. Setup Build Environment
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let build_dir = std::env::temp_dir().join(format!("rustx_jit_{}", timestamp));
        
        if let Err(e) = fs::create_dir_all(&build_dir) {
            return Err(format!("Error creating temp dir: {}", e));
        }
        
        if verbose {
            println!("{} {}", "Build setup:".dimmed(), build_dir.display());
        }
        
        // Initialize cargo project
        let status = Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .arg("--name")
            .arg("app")
            .current_dir(&build_dir)
            .output()
            .map_err(|e| format!("Failed to run cargo init: {}", e))?;
            
        if !status.status.success() {
            return Err(format!("Error initializing cargo: {}", String::from_utf8_lossy(&status.stderr)));
        }
        
        // 3. Configure Dependencies (Cargo.toml)
        let mut cargo_toml = fs::read_to_string(build_dir.join("Cargo.toml"))
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;
            
        // Add core dependency (hardcoded for local dev, should be dynamic in prod)
        let core_path = std::env::current_exe() // Try to find relative to executable or fallback
            .ok()
            .and_then(|p| p.parent()?.parent()?.parent()?.join("crates/core").canonicalize().ok())
            .unwrap_or_else(|| PathBuf::from("/home/grandpa/me/code/rust/RustX/crates/core"));
            
        cargo_toml.push_str(&format!("\nrustx_core = {{ path = \"{}\" }}\n", core_path.display()));
        
        // Add dynamic imports
        for stmt in ast {
            if let Stmt::RustImport { crate_name, version, .. } = stmt {
                // If version starts with '{', assume inline table (e.g. { version = "...", features = ... })
                if version.trim().starts_with('{') {
                     cargo_toml.push_str(&format!("{} = {}\n", crate_name, version));
                } else {
                     cargo_toml.push_str(&format!("{} = \"{}\"\n", crate_name, version));
                }
            }
        }

        // Add optimizations (at the end)
        cargo_toml.push_str("\n[profile.release]\nlto = true\ncodegen-units = 1\npanic = \"abort\"\nopt-level = 3\n");
        
        fs::write(build_dir.join("Cargo.toml"), cargo_toml)
             .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;
        
        // 4. Write Source
        fs::write(build_dir.join("src/main.rs"), code)
            .map_err(|e| format!("Failed to write main.rs: {}", e))?;
            
        // 5. Build/Run
        if run {
            println!("{}", "Compiling and running (release mode)...".bright_yellow());
            let status = Command::new("cargo")
                .arg("run")
                .arg("--release") 
                //.arg("--quiet") // Show compilation progress
                .current_dir(&build_dir)
                .status()
                .map_err(|e| format!("Failed to run cargo run: {}", e))?;
                
            if !status.success() {
                return Err("Execution failed".to_string());
            }
        } else {
            if verbose { println!("{}", "Compiling...".bright_yellow()); }
            let status = Command::new("cargo")
                .arg("build")
                .arg("--release")
                .current_dir(&build_dir)
                .output()
                .map_err(|e| format!("Failed to run cargo build: {}", e))?;
                
            if !status.status.success() {
                return Err(format!("Compilation failed:\n{}", String::from_utf8_lossy(&status.stderr)));
            }
            
            // Move binary if output specified
            if let Some(out_path) = output_path {
                let binary = build_dir.join("target/release/app");
                fs::copy(&binary, &out_path)
                    .map_err(|e| format!("Failed to move binary: {}", e))?;
                if verbose {
                    println!("{} {}", "Successfully built:".bright_green().bold(), out_path.display());
                }
            }
        }
        
        // Cleanup? 
        // fs::remove_dir_all(&build_dir).ok(); 
        
        Ok(())
    }
}
