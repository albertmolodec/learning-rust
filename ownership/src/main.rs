fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("{}", word);

    s.clear(); // this empties the String, making it equal to ""

    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];

    for element in slice.iter() {
        print!("{}", element);
    }
    println!();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
