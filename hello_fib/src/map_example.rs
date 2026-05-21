pub fn summ() {
    let summation: i32 = (1..6).map(|i| 2 * i).sum();
    println!("{}", summation);
}