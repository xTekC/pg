use crate::cli::xcore::xcore_main;
use clap::Parser;
use owo_colors::OwoColorize;
use random_string::generate;

/// Generate passwords
#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set length of password
    #[arg(short = 'l', value_name = "LEN", short)]
    length: usize,

    /// Give password an alias
    #[arg(short = 'a', short)]
    alias: Option<String>,
}

pub async fn cli_main() {
    let cli = Cli::parse();

    match cli.length.to_owned() {
        8 => {
            let l = generate(cli.length, xcore_main().await);
            let a = cli.alias.unwrap_or_default();
            println!(
                "Generated {} char password with {} alias",
                cli.length.green(),
                a.green()
            );
            println!(" ");
            println!("{}  {}", a.yellow(), l.red());
            println!(" ");
        }
        7 => println!("Password is too short!"),
        6 => println!("Password is too short!"),
        5 => println!("Password is too short!"),
        4 => println!("Password is too short!"),
        3 => println!("Password is too short!"),
        2 => println!("Password is too short!"),
        1 => println!("Password is too short!"),
        0 => println!("Password is too short!"),
        _ => {
            let l = generate(cli.length, xcore_main().await);
            let a = cli.alias.unwrap_or_default();
            println!(
                "Generated {} char password with {} alias",
                cli.length.green(),
                a.green()
            );
            println!(" ");
            println!("{}  {}", a.yellow(), l.red());
            println!(" ");
        }
    }
}
