use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", (a + b - 1) / b);
}
