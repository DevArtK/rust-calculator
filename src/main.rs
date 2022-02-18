use std::env::args;

fn main() {

    // Capture arguments passed with Args struct
    let args: Vec<String> = args().collect();

    check_args(&args);

    // Get 1st operand
    let first = Some(&args[1]).unwrap().parse::<f32>().unwrap();
    // Get operator
    let operator = Some(&args[2]).unwrap().parse::<char>().unwrap();
    // Get 2nd operand
    let second = Some(&args[3]).unwrap().parse::<f32>().unwrap();


    println!("---------");
    let match_result = calculate(match_op(&operator), first, second);
    // println!("RESULT = {}", match_result);
    println!("\\e[1;96;127m{}\\e[0m\n", match_result);

}

enum Operators {
    ADD(char),
    SUBTRACT(char),
    MULTIPLY(char),
    DIVIDE(char),
    MODULUS(char)

}

fn match_op(operator: &char) -> Operators {
    match operator {
        '+' => Operators::ADD('+'),
        '-' => Operators::SUBTRACT('-'),
        '*' => Operators::MULTIPLY('*'),
        '/' => Operators::DIVIDE('/'),
        '%' => Operators::MODULUS('%'),
        _ => panic!("Invalid Operator")

    }
}


fn calculate(operator: Operators, first_numb: f32, second_number: f32) -> f32 {
    match operator {
        Operators::ADD('+')  => {
            first_numb + second_number
        }

        Operators::SUBTRACT('-') => {
            first_numb - second_number
        }

        Operators::MULTIPLY('*') => {
            first_numb * second_number
        }

        Operators::DIVIDE('/') => {
            first_numb / second_number
        }

        Operators::MODULUS('%') => {
            first_numb % second_number
        }
        _ => {0.0}


    }
}

fn check_args(args: &Vec<String>) -> bool {
    if args.len() < 3 { panic!("Not enough arguments passed") } else { println!("Number of args {}", args.len()) };

    if args.len() < 3 { false } else { true }
}





