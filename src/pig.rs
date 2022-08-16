fn main() {
    let s = String::from("vectors ond strings and hash");

    let mut res: Vec<String> = Vec::new();
    for line in s.split_whitespace() {
        let first_char = line.chars().nth(0).unwrap();
        if is_vowel(&first_char) {
            res.push(format!("{}-hay", line))
        } else {
            res.push(format!("{}-{}ay", &line[1..], first_char))
        }
    }

    println!("{}", res.join(" "));
}

fn is_vowel(char: &char) -> bool {
    let vowels = ['a', 'e', 'y', 'u', 'i', 'o'];
    vowels.contains(char)
}
