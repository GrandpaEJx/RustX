use rustx::{compile_file, run_file, convert_to_rs, Error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
        }
        "-o" => {
            // Compile to binary mode
            if args.len() != 3 {
                eprintln!("Usage: rustx -o <file.rsx>");
                std::process::exit(1);
            }
            let input_file = &args[2];
            if !input_file.ends_with(".rsx") {
                eprintln!("Error: Input file must have .rsx extension");
                std::process::exit(1);
            }
            
            let output_file = input_file.trim_end_matches(".rsx");
            
            match compile_file(input_file) {
                Ok(rust_code) => {
                    // Write the compiled Rust code to a .rs file
                    let rs_file = format!("{}.rs", output_file);
                    std::fs::write(&rs_file, rust_code).map_err(|e| {
                        Error::RuntimeError(format!("Failed to write output file: {}", e))
                    })?;
                    
                    println!("Compiled {} to {}", input_file, rs_file);
                    
                    // Try to compile to binary using rustc
                    match std::process::Command::new("rustc")
                        .arg(&rs_file)
                        .arg("-o")
                        .arg(output_file)
                        .output()
                    {
                        Ok(output) => {
                            if output.status.success() {
                                println!("Binary created: {}", output_file);
                                // Clean up the intermediate .rs file
                                let _ = std::fs::remove_file(&rs_file);
                            } else {
                                eprintln!("Warning: Failed to compile binary:");
                                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                                println!("Rust code saved to: {}", rs_file);
                            }
                        }
                        Err(_) => {
                            eprintln!("Warning: rustc not found, skipping binary compilation");
                            println!("Rust code saved to: {}", rs_file);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error compiling {}: {}", input_file, e);
                    std::process::exit(1);
                }
            }
        }
        "-s" => {
            // Convert to .rs mode
            if args.len() != 3 {
                eprintln!("Usage: rustx -s <file.rsx>");
                std::process::exit(1);
            }
            let input_file = &args[2];
            if !input_file.ends_with(".rsx") {
                eprintln!("Error: Input file must have .rsx extension");
                std::process::exit(1);
            }
            
            let output_file = input_file.trim_end_matches(".rsx");
            let rs_file = format!("{}.rs", output_file);
            
            match convert_to_rs(input_file) {
                Ok(rust_code) => {
                    std::fs::write(&rs_file, rust_code).map_err(|e| {
                        Error::RuntimeError(format!("Failed to write output file: {}", e))
                    })?;
                    println!("Converted {} to {}", input_file, rs_file);
                }
                Err(e) => {
                    eprintln!("Error converting {}: {}", input_file, e);
                    std::process::exit(1);
                }
            }
        }
        file => {
            // Direct execution mode
            if !file.ends_with(".rsx") {
                eprintln!("Error: File must have .rsx extension");
                std::process::exit(1);
            }
            
            match run_file(file) {
                Ok(_) => {
                    // Success - program executed
                }
                Err(e) => {
                    eprintln!("Error running {}: {}", file, e);
                    std::process::exit(1);
                }
            }
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("RustX - A minimal scripting language");
    println!("");
    println!("Usage:");
    println!("  rustx <file.rsx>     Run a RustX script file directly");
    println!("  rustx -o <file.rsx>  Compile to binary executable");
    println!("  rustx -s <file.rsx>  Convert to Rust (.rs) file");
    println!("  rustx --help         Show this help message");
    println!("");
    println!("Examples:");
    println!("  rustx demo/main.rsx       # Run the script");
    println!("  rustx -o demo/main.rsx    # Create binary executable");
    println!("  rustx -s demo/main.rsx    # Convert to main.rs");
}
