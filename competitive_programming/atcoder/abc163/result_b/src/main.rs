use::proconio::input;
fn main() {
    input! {
        n: i64,
        m: usize,
        data: [i64; m],
    }
    let mut sum = 0;
    for value in data {
        sum += value;
    }
    
    if sum <= n {
        println!("{}", n - sum);
    } else {
        println!("{}", -1);
    }
}
