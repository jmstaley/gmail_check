extern crate reqwest;
extern crate feed_rs;

use std::env;
use reqwest::Error;
use feed_rs::parser;

fn main() -> Result<(), Error> {
    let url = "https://mail.google.com/mail/feed/atom";
    let args: Vec<String> = env::args().collect();
    let username = &args[1];
    let password = &args[2];

    let response = reqwest::blocking::Client::new()
        .get(url)
        .basic_auth(username, Some(password))
        .send()?;
    let atom = response.text().unwrap();
    let feed = parser::parse(atom.as_bytes()).unwrap();
    println!("{}", feed.entries.len());
    Ok(())
}
