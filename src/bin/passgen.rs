mod cli;
use cli::xcli::*;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    block_on(cli_main());
}
