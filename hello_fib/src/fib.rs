fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let next_fib = a + b;
        a = b;
        b = next_fib;
    }
    
}

fn main() {
    let n = 10; // Calculate the 10th Fibonacci number
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}