use csv::ReaderBuilder;
use std::error::Error;

pub fn get_args() -> (usize, Vec<char>, Vec<(char, usize)>) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("请提供长度和字符");
        std::process::exit(1);
    }
    // 第一个参数是单词长度，第二个参数是包含的字符但不知道具体位置，其余是正确字符加位置e5，a6
    let length: usize = args[1].parse().unwrap();
    let ch: Vec<char> = args[2].chars().collect();
    let mut ch_parts = Vec::new();
    for i in 3..args.len() {
        let c: char = args[i].chars().nth(0).unwrap();
        let index: usize = args[i][1..].parse().unwrap();
        ch_parts.push((c, index));
    }
    (length, ch, ch_parts)
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

pub fn filter_words(
    records: Vec<Word>,
    length: usize,
    ch: Vec<char>,
    ch_parts: Vec<(char, usize)>,
) -> Vec<Word> {
    records
        .into_iter()
        .filter(|record| {
            record.content.len() == length
                && ch_parts
                    .iter()
                    .all(|(c, i)| record.content.chars().nth(*i - 1).unwrap() == *c)
                && ch.iter().all(|c| record.content.contains(*c))
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

        let result = filter_words(data, 5, vec!['a', 'u'], vec![('b', 2)]);
        assert_eq!(result.len(), 1);
    }
}
