use std::{error::Error, process};

use translatify::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = run().await {
        println!("Error {}", err);
        process::exit(1);
    }
    Ok(())
}
