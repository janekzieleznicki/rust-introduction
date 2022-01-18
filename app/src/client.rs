use clap::Parser;
use crate::hashed_password_generator_client::HashedPasswordGeneratorClient;

tonic::include_proto!("hasher");

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Me <me@myaddress.com>")]
struct Opts {
    /// Server address to access
    #[clap(short, long, default_value = "[::1]:50051")]
    server: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let mut client = HashedPasswordGeneratorClient::connect(format!("http://{}", opts.server)).await?;
    let mut stream = client.generate_password(()).await?.into_inner();
    while let Some(hashed_password) = stream.message().await? {
        println!("{:?}", hashed_password)
    }
    Ok(())
}