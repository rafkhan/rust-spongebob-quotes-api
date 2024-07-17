use axum::{
    routing::get,
    Router,
    response::Json,
    extract::Query
};
use std::collections::HashMap;
use serde_json::{Value, json};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

const PORT: i16 = 4200;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    println!("Starting server...");
    println!("Reading spongebob episode transcripts...");

    // run our app with hyper (wtf is hyper?)
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.unwrap();
    println!("Listening on http://0.0.0.0:{PORT}",);
    axum::serve(listener, app).await.unwrap();
}

async fn root(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    // TODO frontload reading the file
    let file = File::open("src/static/krusty-krab-pizza-transcript.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let size: usize = match params.get("size") {
        Some(i) => i.parse::<usize>().unwrap(),
        None => 1
    };

    let all_lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let mut lines: Vec<&String> = Vec::new();

    for i in 0..=size {
        let random_index = rand::thread_rng().gen_range(0..all_lines.len());
        let random_line = &all_lines[random_index];
        lines.push(random_line);
    }

    return Json(json!({ "data": lines }));
}