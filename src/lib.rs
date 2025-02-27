use csv::ReaderBuilder;
use std::error::Error;

pub fn get_args() -> (usize, Vec<char>) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("请提供长度和字符");
        std::process::exit(1);
    }
    let length: usize = args[1].parse().unwrap();
    let ch: Vec<char> = args[2].chars().collect();
    (length, ch)
}

#[derive(Debug, Clone)]
pub struct Word {
    pub content: String,
    pub chinese: String,
}

pub fn read_csv(data: &str) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_reader(data.as_bytes());
    let mut result = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let word = Word {
            content: record[0].to_string(),
            chinese: record[1].to_string(),
        };
        result.push(word);
    }
    Ok(result)
}

pub fn filter_words(records: Vec<Word>, length: usize, ch: Vec<char>) -> Vec<Word> {
    records
        .into_iter()
        .filter(|record| {
            record.content.len() == length
                && ch
                    .iter()
                    .all(|c| record.content.chars().collect::<Vec<char>>().contains(c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let path = "assets/EnWords.csv";
        let records = read_csv(path).unwrap();
        assert_eq!(records[0].content, "a");
    }

    #[test]
    fn test_filter_words() {
        let data = vec![
            Word {
                content: "abuse".to_string(),
                chinese: "绝对的".to_string(),
            },
            Word {
                content: "reuse".to_string(),
                chinese: "复用".to_string(),
            },
            Word {
                content: "plume".to_string(),
                chinese: "羽流".to_string(),
            },
        ];

        let result = filter_words(data, 5, vec!['a', 'u']);
        assert_eq!(result.len(), 1);
    }
}
