mod config;
mod parser;
mod path_builder;
mod conflict_check;
mod creator;

use config::Config;
use parser::parse;
use path_builder::build;
use conflict_check::check;
use creator::create;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new();
    let project_name = match parse(&args) {
        Ok(name) => name,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let path = build(&config, &project_name);

    if let Err(e) = check(&path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    if let Err(e) = create(&config, &path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("✓ Created project: {}", path.display());
    println!();
    println!("Subdirectories:");
    for dir in &config.default_dirs {
        println!("  {}/{}", path.display(), dir);
    }
}
