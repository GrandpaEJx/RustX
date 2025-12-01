use rustx::compile_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 2 {
        let arg = &args[1];
        if arg == "--help" || arg == "-h" {
            print_help();
        } else {
            match compile_file(arg) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        eprintln!("Usage: rustx [file]");
        std::process::exit(1);
    }
    
    Ok(())
}

fn print_help() {
    println!("RustX - A minimal scripting language (Compiler)");
    println!("");
    println!("Usage:");
    println!("  rustx [file]    Compile a RustX script file to Rust");
    println!("  rustx --help    Show this help message");
}
