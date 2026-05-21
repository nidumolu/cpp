mod farenheit_to_celsius;
mod ndarray;
mod map_example;

fn main() {
    println!("Hello, world!");
  
    println!(" Celsius value of 10 degree Fahrenheit is : {}",farenheit_to_celsius::celsius(10));
    let s = format!("{:?}", ndarray::array_print()); 
    println!("Output of NdArray {}", s);
    map_example::summ();
}
