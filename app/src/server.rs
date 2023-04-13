use crate::hashed_password_generator_server::{
    HashedPasswordGenerator, HashedPasswordGeneratorServer,
};
use clap::Parser;
use serde::Deserialize;
use sha256_wrapper::sha256hash;
use tokio::signal;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

tonic::include_proto!("hasher");

#[derive(Deserialize, Debug)]
struct Password {
    data: String,
}

async fn generator_loop(
    tx: mpsc::Sender<Result<HashedPassword, Status>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut count = 0;
    loop {
        count += 1;
        let request_url = format!(
            "https://passwordinator.onrender.com/?len={len}",
            len = 31+count
        );
        log::info!("Count: {} URL {}",count,request_url);
        match reqwest::get(&request_url).await {
            Err(e) => log::error!("Server returned {}",e),
            Ok(response) => { let password: Password = response.json().await ?;
                tx.send(Ok(HashedPassword {
                    index: count as i32,
                    password: password.data.to_string(),
                    hash: hex::encode(sha256hash(password.data.as_str())),
                })).await ?;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PasswordWithHash {
    password: String,
    hash: String,
}

struct HasherService {}

#[tonic::async_trait]
impl HashedPasswordGenerator for HasherService {
    type GeneratePasswordStream = ReceiverStream<Result<HashedPassword, Status>>;

    async fn generate_password(
        &self,
        _: Request<()>,
    ) -> Result<Response<Self::GeneratePasswordStream>, Status> {
        let (tx, rx) = mpsc::channel(2);
        tokio::spawn(generator_loop(tx));
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Me <me@myaddress.com>")]
struct Opts {
    /// Address to run server on
    #[clap(short, long, default_value = "[::1]:50051")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let opts: Opts = Opts::parse();
    let addr = opts.address.parse()?;
    println!("Spawning grpc server on {}", addr);
    let hasher_service = HasherService {};
    tokio::task::spawn(
        tonic::transport::Server::builder()
            .add_service(HashedPasswordGeneratorServer::new(hasher_service))
            .serve(addr),
    );
    signal::ctrl_c().await.expect("failed to listen for event");
    println!("Graceful shutdown");
    Ok(())
}
