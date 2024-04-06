use std::io;

fn main() {
    println!("Olá, escreva um número binário: ");

    let mut binary_number = String::new();
    let mut converted_numbers: Vec<u32> = Vec::new();

    io::stdin().read_line(&mut binary_number).unwrap();

    let mut pos = 0;

    for binary in binary_number.trim().chars().rev() {
        if binary == '1' {
            let result = 1 * 2_u32.pow(pos);
            converted_numbers.push(result);
        }
        pos += 1;
    }

    let decimal_number = convert_to_decimal(&mut converted_numbers);

    println!(
        "O número binário {} em decimal é {}",
        binary_number, decimal_number
    );
}

fn convert_to_decimal(numbers: &mut Vec<u32>) -> u32 {
    let mut result = 0;

    for number in numbers {
        result += number.clone();
    }

    result
}
