pub fn intro() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("Hello Wolrd!");

    let word = first_word(&s); // word will get the value 5

    println!("{word}");

    s.clear(); // this empities the strings, making it equal to ""

    // word still has the value 5 here, but there's no more string
    // that we coud meaningfully use the value 5 with word is now
    // tottaly invalid!
}
