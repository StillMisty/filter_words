use filter_words::filter_words;
use filter_words::get_args;
use filter_words::read_csv;

fn main() {
    let (length, ch, ch_parts) = get_args();

    let data = include_str!("../assets/EnWords.csv");
    let records = read_csv(data).unwrap();

    let filtered = filter_words(records, length, ch, ch_parts);

    if filtered.is_empty() {
        println!("无符合要求单词");
    } else {
        filtered
            .iter()
            .enumerate()
            .for_each(|(i, word)| println!("{}\t{}\t{}", i + 1, word.content, word.chinese));
    }
}
