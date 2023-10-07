/******************************
 *  Copyright (c) xTekC.      *
 *  Licensed under MPL-2.0.   *
 *  See LICENSE for details.  *
 *                            *
 ******************************/

use clap::Parser;
use owo_colors::OwoColorize;
use pg::xcore::core::core_main;
use random_string::generate;

/// Generate secure passwords
#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Length of password
    #[arg(short = 'l', value_name = "LEN", short)]
    length: usize,

    /// Alias for password
    #[arg(short = 'a', short)]
    alias: Option<String>,
}

pub async fn cli_main() {
    let cli = Cli::parse();

    match cli.length.to_owned() {
        8 => {
            let l = generate(cli.length, core_main().await);
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
        0..=7 => println!("Password is too short!"),
        _ => {
            let l = generate(cli.length, core_main().await);
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
