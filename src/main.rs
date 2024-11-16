extern crate reqwest;
extern crate tokio;
extern crate urlencoding;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::env;
use tokio::main;

async fn tele_msg(bot_token: &str, chat_id: &str, msg: &str) {
    let telegram_url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&parse_mode=HTML&text={}",
        bot_token, chat_id, msg
    );

    match reqwest::get(&telegram_url).await {
        Ok(_) => println!("Message sent!"),
        Err(e) => eprintln!("Failed to send message: {}", e),
    }
}

#[main]
async fn main() {
    println!("Starting...");

    let sosad_cookie = env::var("SESSION").expect("SESSION env variable is not set");
    let bot_token = env::var("TG_SECRET").expect("TG_SECRET env variable is not set");
    let chat_id = env::var("TG_CHAT_ID").expect("TG_CHAT_ID env variable is not set");

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(&sosad_cookie).unwrap());
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 11_2_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    match client.get("https://sosad.fun/qiandao").send().await {
        Ok(response) => {
            let now = chrono::Local::now();
            let now_str = now.format("%Y-%m-%d %H:%M:%S %Z").to_string();

            let message = format!(
                "✅ 签到于 {}\nThe status code: {}.",
                now_str, response.status()
            );

            println!("{}", message);

            tele_msg(&bot_token, &chat_id, &urlencoding::encode(&message)).await;
        }
        Err(e) => eprintln!("Error fetching sosad page: {}", e),
    }
}