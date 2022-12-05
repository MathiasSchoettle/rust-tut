fn main() {
    let mut x = 5;

    let a = &x;
    let b = &x;
    let c = &b;

    x = 2;

    println!("{}", x);
}
