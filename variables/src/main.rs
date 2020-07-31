fn main() {
    let array: [u32; 5] = [1; 5];

    for x in &array {
        print!("{} ", x);
    }
}
