use rustx::{run_file, run_repl};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 1 {
        // No arguments, run REPL
        run_repl()?;
    } else if args.len() == 2 {
        // One argument, could be a file or help flag
        let arg = &args[1];
        if arg == "--help" || arg == "-h" {
            print_help();
        } else {
            // Treat as file path
            run_file(arg)?;
        }
    } else {
        eprintln!("Usage: rustx [file]");
        std::process::exit(1);
    }
    
    Ok(())
}

fn print_help() {
    println!("RustX - A minimal scripting language");
    println!("");
    println!("Usage:");
    println!("  rustx [file]    Execute a RustX script file");
    println!("  rustx           Start interactive REPL");
    println!("  rustx --help    Show this help message");
    println!("");
    println!("Example:");
    println!("  rustx demo/main.rsx");
}
