mod farenheit_to_celsius;
mod ndarray;
mod map_example;
mod guess_number;

fn main() {

    //println!("cargo:rustc-link-search=native=C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\Tools\\MSVC\\14.44.35207\\bin\\Hostx64\\x64\\link.exe");

    println!("Hello, world!");
  
    println!(" Celsius value of 10 degree Fahrenheit is : {}",farenheit_to_celsius::celsius(10));
    let s = format!("{:?}", ndarray::array_print()); 
    println!("Output of NdArray {}", s);
    map_example::summ();

    guess_number::guess();
}
