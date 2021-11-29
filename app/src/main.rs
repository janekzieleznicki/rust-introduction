use serde::Deserialize;
use reqwest::Error;
use sha256_wrapper::sha256hash;
use hex;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://passwordinator.herokuapp.com/generate?len={len}",
                              len = 32);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let password: Password = response.json().await?;
    println!("{:?}|{:?}", password, Sha256::from_pwd(&password));
    Ok(())
}