use regex_macro::load_regex_files;

fn main() {
    load_regex_files!("tests/regex");
    let events = Event::vec_from_file("tests/samples/events.txt").unwrap();
    println!("{}", serde_json::to_string_pretty(&events).unwrap());
}
