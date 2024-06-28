use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: temperature-converter <value> <from c | f | k> <to c | f | k>");
        return;
    }
    let value = args[1].parse::<f64>().unwrap();
    let from = args[2].parse::<char>().unwrap().to_ascii_lowercase();
    let to = args[3].parse::<char>().unwrap().to_ascii_lowercase();
    let result = convert_temperature(value, from, to);
    println!("{}", result);
}

fn convert_temperature(value: f64, from: char, to: char) -> f64 {
    match from {
        'c' => match to {
            'c' => value,
            'f' => (value * 1.8) + 32.0,
            'k' => (value + 273.15) * 1.8,
            _ => panic!("Invalid <to> parameter, allowed values are c, f, k"),
        },
        'f' => match to {
            'c' => (value - 32.0) / 1.8,
            'f' => value,
            'k' => (value + 459.67) / 1.8,
            _ => panic!("Invalid <to> parameter, allowed values are c, f, k"),
        },
        'k' => match to {
            'c' => (value - 273.15) / 1.8,
            'f' => (value * 1.8) - 459.67,
            'k' => value,
            _ => panic!("Invalid <to> parameter, allowed values are c, f, k"),
        },
        _ => panic!("Invalid <from> parameter, allowed values are c, f, k"),
    }
}
