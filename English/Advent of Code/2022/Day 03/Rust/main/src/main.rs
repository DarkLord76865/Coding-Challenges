fn main() {
    let input = include_str!("../../../input.txt");

    let mut result: u64 = 0;
    for line in input.split("\n") {
        if line != "" {
            let (first, last) = line.split_at(line.chars().count() / 2);
            for character in first.chars() {
                if last.contains(character) {
                    result += letter_value(character);
                    break;
                }
            }
        }
    }
    println!("{}", result);

    let mut result: u64 = 0;
    let mut curr_group_members: Vec<&str> = vec![];
    for line in input.split("\n") {
        if curr_group_members.len() == 3 {
            for character in curr_group_members[0].chars() {
                if curr_group_members[1].contains(character) && curr_group_members[2].contains(character) {
                    result += letter_value(character);
                    break;
                }
            }
            curr_group_members = vec![line];
        } else {
            curr_group_members.push(line);
        }
    }
    println!("{}", result);
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn letter_value(letter: char) -> u64 {
    let mut value: u64 = 1;
    for x in ALPHABET.chars() {
        if letter == x {
            break
        }
        value += 1;
    }
    value
}
