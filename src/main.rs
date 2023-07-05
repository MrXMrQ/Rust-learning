use std::io;

fn main() {
    convert_temperatures();

    let fib_sequence = fibonacci(10);
    // Prints the Fibonacci sequence in debug format.
    println!("Fibonacci sequence: {:?}", fib_sequence);
}

fn convert_temperatures() {
    let mut input = String::new();

    println!("Please enter the temperature in °C that you would like to convert!");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let input: i32 = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            return;
        }
    };

    let fahrenheit = (input * 9/5) + 32;

    println!("{input}°C is {fahrenheit}°F!")
}

fn fibonacci(n: u32) -> Vec<u32> {
    // Initialization of a vector with the starting points 0 and 1 of the Fibonacci sequence.
    let mut sequence = vec![0, 1]; 
    
    // Start at '2' and continue until reaching the value 'n'.
    for i in 2..n as usize { 
        let next_number = sequence[i - 1] + sequence[i - 2];

    // Stores the result in the vector to have new addition values for the the Fibonacci sequence.
        sequence.push(next_number);
    }

    // Returns the "Fibonacci sequence".
    sequence
}