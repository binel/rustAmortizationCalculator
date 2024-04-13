use std::io; 

fn main() {
    println!("Enter the loan amount:");
    let loan_amount: f64 = match get_input() {
    	Ok(num) => num,
    	Err(_) => {
    		println!("Please enter a number");
    		return;
    	}
    };
    
    println!("Enter the interest rate (ex: 4% as 0.04):");
    let interest_rate: f64 = match get_input() {
    	Ok(num) => num,
    	Err(_) => {
    		println!("Please enter a number");
    		return;
    	}
    };
    
    let payment_interest = loan_amount * (interest_rate  / 12.0);
    
    println!("The interest on your first payment will be ${:.2}", payment_interest);
}

fn get_input<T: std::str::FromStr>() -> Result<T, T::Err> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	input.trim().parse::<T>()
}
