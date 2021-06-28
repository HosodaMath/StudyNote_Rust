use::proconio::input;
fn main() {
    input! {
        n: usize,
        data: [i64; n],
    }
    for value in data {
        println!("{}", value);
    }
}
