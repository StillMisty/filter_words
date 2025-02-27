use filter_words::filter_words;
use filter_words::get_args;
use filter_words::read_csv;

fn main() {
    let (length, ch) = get_args();

    let data = include_str!("../assets/EnWords.csv");
    let records = read_csv(data).unwrap();

    let filtered = filter_words(records, length, ch);
    if filtered.is_empty() {
        println!("无符合要求单词");
    } else {
        filtered
            .iter()
            .for_each(|word| println!("{}: {}", word.content, word.chinese));
    }
}
