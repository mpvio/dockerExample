use std::io;
use reqwest::blocking::Client;
use serde_json::Value;

fn main() {
    println!("Enter String:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    let client = Client::new();
    let response = client.post("http://set1:5000/process")
    .body(input.trim().to_string())
    .send()
    .expect("Failed to send request.");

    let json_response: Value = response.json().expect("Failed to parse JSON");
    println!("Response: {}", json_response["message"]);
}