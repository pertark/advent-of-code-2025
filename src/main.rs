fn main() {
    // command line cargo run [day number] runs [day number].rs file from src/bin directory
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run [day number]");
        std::process::exit(1);
    }
    let day = &args[1];
    let bin_path = format!("src/bin/{}.rs", day);
    if !std::path::Path::new(&bin_path).exists() {
        eprintln!("Error: File {} does not exist.", bin_path);
        std::process::exit(1);
    }

    print!("Running day {}...\n", day);
    let status = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(day)
        .status()
        .expect("Failed to execute cargo run");
    
    if !status.success() {
        eprintln!("Error: cargo run failed.");
        std::process::exit(1);
    }
}
