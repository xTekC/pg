mod xcli;
use futures::executor::block_on;
use xcli::cli::*;

#[tokio::main]
async fn main() {
    block_on(cli_main());
}
