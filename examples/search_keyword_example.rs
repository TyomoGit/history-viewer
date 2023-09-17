use line_history::LineHistory;
use std::fs;

fn read() -> String {
    fs::read_to_string("./history.txt").unwrap()
}

fn main() {
    let content = read();
    let history = LineHistory::new(&content);
    let result = history.search_by_keyword("a");
    for elem in result {
        println!("{}", elem.line);
    }
}