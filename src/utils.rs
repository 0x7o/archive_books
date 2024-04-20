use std::collections::HashMap;

pub fn remove_line_breaks(text: String) -> String {
    text.replace("-\n", "").replace("\n", " ")
}

pub fn get_entropy(input_string: &str) -> f64 {
    let mut alphabet: HashMap<char, f64> = HashMap::new();
    let alphabet_size = input_string.len() as f64;
    let mut entropy = 0.0;

    for char in input_string.chars() {
        *alphabet.entry(char).or_insert(0.0) += 1.0;
    }

    for (_, count) in &alphabet {
        let prob = count / alphabet_size;
        entropy -= prob * prob.log2();
    }

    entropy
}