use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Shaw Atkinson", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!(
        "Hello {} (from rust-nix-template)!",
        args.name.unwrap_or("world".to_string())
    );
}
