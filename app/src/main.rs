use serde::Deserialize;
use reqwest::Error;
use sha256_wrapper::sha256hash;
use hex;
use tokio::sync::mpsc;
use std::time::{Duration, Instant};
use tokio::time;

#[derive(Deserialize, Debug)]
struct Password {
    data: String,
}

#[derive(Debug)]
struct Sha256 {
    hash: String,
}

impl Sha256 {
    pub fn from_pwd(pwd: &Password) -> Sha256 {
        Sha256 {
            hash: String::from(hex::encode(sha256hash(pwd.data.as_str())))
        }
    }
}

async fn getter(len: usize, mpsc_tx: mpsc::Sender<Password>) -> Result<(), Error> {
    for _ in 0..3 {
        let request_url = format!("https://passwordinator.herokuapp.com/generate?len={len}",
                                  len = len);
        println!("{}", request_url);
        let response = reqwest::get(&request_url).await?;

        let password: Password = response.json().await?;
        mpsc_tx.send(password).await;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    tokio::task::spawn(getter(32, tx) );
    tokio::task::spawn(getter(64, tx2) );

    // time::sleep(time::Duration::from_secs(5)).await;
    while let Some(pwd) = rx.recv().await {
        println!("{:?}|{:?}", pwd, Sha256::from_pwd(&pwd));
    }
    Ok(())
}