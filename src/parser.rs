mod worker;

use colored::Colorize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize)]
struct Filter {
    year: HashMap<String, String>,
    language: HashMap<String, String>,
}

pub struct Parser {
    filter: String,
    workers: i32,
    output_dir: String
}

impl Parser {
    pub fn create(language: String, year_from: String, year_to: String, workers: i32, output_dir: String) -> Parser {
        let filter = Filter {
            year: HashMap::from([
                (year_from, String::from("gte")),
                (year_to, String::from("lte")),
            ]),
            language: HashMap::from([(language, String::from("inc"))]),
        };
        Parser {
            filter: serde_json::to_string(&filter).unwrap_or_else(|_| {
                eprintln!("Error while parsing filter JSON!");
                String::from("")
            }),
            workers,
            output_dir
        }
    }
}

impl Parser {
    pub async fn start(&self) {
        assert_eq!(
            200 % self.workers,
            0,
            "200 must be a multiple of the number of workers (2, 4, 8, 20...)"
        );
        fs::create_dir_all(self.output_dir.clone()).expect("Failed to create output_dir!");
        for i in 0..self.workers {
            let start_page = i * (200 / self.workers);
            let end_page = i * (200 / self.workers) + (200 / self.workers);
            let worker = worker::Worker::init(start_page, end_page, self.filter.clone(), true, i);
            println!(
                "Starting Worker #{}. Page {} to {}",
                i, start_page, end_page
            );
            worker.start().await;
        }
    }
}
