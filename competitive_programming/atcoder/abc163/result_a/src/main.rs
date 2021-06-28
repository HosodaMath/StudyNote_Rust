use::proconio::input;
fn main() {
    input! {
        n: f64,
    }
    println!("{}", std::f64::consts::PI * n);
}
