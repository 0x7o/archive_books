mod utils;

use std::fs;
use chrono::{DateTime, Utc};
use colored::*;
use serde_json::Value;
use std::io::{Read, Write};
use std::ops::Range;

pub struct Book {
    title: String,
    description: String,
    text: String,
    date: DateTime<Utc>,
    pdf_path: String,
}

pub struct Worker {
    pages: Range<i32>,
    filter: String,
    clean: bool,
    id: i32,
}

impl Worker {
    pub fn init(start_page: i32, end_page: i32, filter: String, clean: bool, id: i32) -> Worker {
        Worker {
            pages: start_page..end_page + 1,
            filter,
            clean,
            id,
        }
    }
}

async fn get(page: i32, filter: &String) -> Value {
    let url = String::from("https://archive.org/services/search/beta/page_production/?user_query=&page_type=collection_details&page_target=texts&hits_per_page=50&page=".to_owned() + &*page.to_string() + &*"&filter_map=".to_owned() + &*filter + "&sort=publicdate:desc&aggregations=false");
    let raw = reqwest::get(url)
        .await
        .expect("Failed to send a GET request. Check availability of archive.org")
        .text()
        .await
        .unwrap();
    serde_json::from_str(&raw).expect("Error when reading JSON from text")
}

async fn get_hits(page: i32, filter: &String) -> Vec<Value> {
    let response = get(page, filter).await;
    response["response"]["body"]["hits"]["hits"]
        .as_array()
        .unwrap()
        .clone()
}

async fn get_book_text(identifier: String) -> Option<String> {
    let url =
        String::from("https://archive.org/compress/".to_owned() + &identifier + "/formats=DjVuTXT");
    let mut file = tempfile::tempfile().unwrap();
    let response = match reqwest::get(url.clone()).await {
        Ok(value) => value,
        Err(_) => {
            return None;
        }
    };
    if response.status() == 200 {
        let bytes = response.bytes().await.unwrap();
        file.write(&*bytes).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        let mut file = zip.by_index(0).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let content = String::from_utf8(buffer).unwrap();
        Some(content)
    } else {
        None
    }
}

impl Worker {
    pub async fn start(&self) {
        for page in self.pages.clone().into_iter() {
            println!("Processing page #{}", page);
            let hits = get_hits(page, &self.filter).await;
            for hit in hits.into_iter() {
                println!(
                    "Processing {}...",
                    hit["fields"]["title"].to_string().bold()
                );
                let text = get_book_text(
                    hit["fields"]["identifier"].to_string().replace('"', "")
                ).await;
                match text {
                    None => { eprintln!("Failed to download the text of the book {}", hit["fields"]["title"]) }
                    Some(value) => {
                        let paragraphs = value.split("\n\n").collect::<Vec<&str>>();
                        let mut cleaned_text = String::new();
                        for parahraph in paragraphs {
                            let cleaned_p = utils::remove_line_breaks(parahraph);
                            let entropy = utils::get_entropy(&cleaned_p);
                            if entropy < 2.9 || entropy > 3.4 {
                                continue;
                            }
                            cleaned_text.push_str(&*(cleaned_p + &*"\n".to_owned()));
                        }
                        cleaned_text = cleaned_text.trim().to_string();
                        fs::write(hit["fields"]["title"].to_string().replace('"', "") + ".txt", cleaned_text.into_bytes()).expect("Failed to write text");
                    }
                }
            }
        }
    }
}
