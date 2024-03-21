use serde_derive::{Deserialize, Serialize};

// 转换上面内容为结构体
#[derive(Deserialize, Serialize, Debug)]
struct University {
    alpha_two_code: String,
    country: String,
    domains: Vec<String>,
    name: String,
    state_province: Option<String>,
    web_pages: Vec<String>,
    add_field: Option<String>,
}

fn main() {
    let input_path = std::env::args().nth(1).unwrap_or("./data/china_science.json".to_string());
    let output_path = std::env::args().nth(2).unwrap_or("./data/china_science_output.json".to_string());
    let mut china_science = {
        // Load the first file into a string.
        let china_science_text = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Vec<University>>(&china_science_text).unwrap()
    };
    for university in china_science.iter_mut() {
        university.name = university.name.replace("University", "大学");
        university.add_field = Option::from(String::from(&university.country) + " ==> " + university.name.as_str());
    }
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&china_science).unwrap(),
    ).unwrap();
}
