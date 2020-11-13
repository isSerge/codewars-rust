fn parse_number (string: &str) -> i8 {
    string
        .chars()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap() as i8
}

fn order(sentence: &str) -> String {
    let mut r = sentence
        .split_whitespace()
        .collect::<Vec<&str>>();
    
    r.sort_by(|a, b| parse_number(a).cmp(&parse_number(b)));

    r.join(" ")
}