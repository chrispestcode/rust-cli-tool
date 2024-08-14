use clap::Parser;

#[derive(Parser)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
    let args =  Cli::parse();
    
    println!("pattern {:?}, path: {:?}", args.pattern, args.path);

    let result = std::fs::read_to_string(&args.path);
    
    let content = match result {
        Ok(content)  => {content}
        Err(error) => {panic!("Error found: {} ", error);}
    };
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
