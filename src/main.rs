use std::io;

fn main() {
    println!("Welcome to the temperature converter app!");
    
    loop {
        println!("Select input temperature unit (C/F), or press 'q' to quit:");
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit: char = match unit.trim().parse() {
            Ok(val) => { 
                if val == 'C' || val == 'F' { val }
                else if val == 'q' { break; }
                else {
                    continue;
                }
            },
            Err(_) => { 
                println!("Please enter either C or F.");
                continue;
            },
        };

        println!("Enter the temperature:");
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Please enter a valid number.");
                continue;
            },
        };

        let value: f64 = if unit == 'C' { to_fahrenheit(temp) } else { to_celsius(temp) };

        let unit: char = if unit == 'C' { 'F' } else { 'C' };
        
        println!("{:.1} {}", value, unit);
    }
}

fn to_celsius(far: f64) -> f64 {
    let cel: f64 = (far - 32 as f64)*(5 as f64/9 as f64);
    cel
}

fn to_fahrenheit(cel: f64) -> f64 {
    let far: f64 = (9 as f64/5 as f64)*cel + 32 as f64;
    far
}
