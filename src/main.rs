use scanner::scanner::Scanner;

// Example
fn main() {
    let input = "8 + 9";

    let mut scanner = Scanner::new(&input);

    let x = scanner.next_number().expect("Expecting number");
    let op = scanner.next_word().expect("Expecting word");
    let y = scanner.next_number().expect("Expecting number");

    let result = match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Invalid operator"),
    };

    println!("{x} {op} {y} = {result}");
}
