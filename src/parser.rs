use std::collections::HashMap;
use serde::{Serialize};

#[derive(Serialize)]
struct Filter {
    year: HashMap<String, String>,
    language: HashMap<String, String>,
}

pub struct Parser {
    filter: String,
}

impl Parser {
    pub fn create(language: String, year_from: String, year_to: String) -> Parser {
        let filter = Filter {
            year: HashMap::from([
                (year_from, String::from("gte")),
                (year_to, String::from("lte"))
            ]),
            language: HashMap::from([
                (language, String::from("inc"))
            ]),
        };
        Parser {
            filter: match serde_json::to_string(&filter) {
                Ok(value) => { value }
                Err(_) => {
                    eprintln!("Error while parsing filter JSON!");
                    String::from("")
                }
            },
        }
    }
}