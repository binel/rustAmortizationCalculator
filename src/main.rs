use std::io; 

fn main() {
    println!("Enter the loan principal:");
    let principal: f64 = match get_input() {
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
    
    println!("Enter the term in years");
    let term_years: i32 = match get_input() {
    	Ok(num) => num,
    	Err(_) => {
    		println!("Please enter a number");
    		return;
    	}
    };
    
    // Calculate Monthly Payment 
    let num_payments: i32 = term_years * 12;
    let monthly_rate: f64 = interest_rate / 12.0;
    let rp = (1.0 + monthly_rate).powi(num_payments);
    let monthly_payment: f64 = principal * ((monthly_rate * rp)/(rp - 1.0));
    
    
    println!("Your monthly payment will be ${:.2}", monthly_payment);
    println!("Schedule:");
    let mut balance = principal;
    for i in 0..num_payments {
    	let interest = balance * monthly_rate;
    	let principal_payment = monthly_payment - interest;
    	balance = balance - principal_payment;
    	println!("Payment {}: Balance: {:.2}, Interest {:.2}, Principal {:.2}", i + 1, balance, interest, principal_payment); 
    }
}

fn get_input<T: std::str::FromStr>() -> Result<T, T::Err> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	input.trim().parse::<T>()
}