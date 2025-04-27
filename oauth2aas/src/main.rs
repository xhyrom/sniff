use std::env;

use gpapi::Gpapi;

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();

    let email = args.get(1).expect("Missing email");
    let oauth2 = args.get(2).expect("Missing oauth2 token");

    let mut api = Gpapi::new("ad_g3_pro", &email);
    println!("{:?}", api.request_aas_token(oauth2).await);
    println!("{:?}", api.get_aas_token().unwrap());
}
