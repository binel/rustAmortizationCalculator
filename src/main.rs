use std::io; 
mod amortization;
mod loan;

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
    
    let loan = loan::NaiveLoan {
    	original_principal: principal,
    	interest_rate,
    	term_years
    };
    
    let schedule = amortization::amortization_schedule_from_loan(loan);
    
    for i in 0..schedule.len()
    {
    	println!("{}\n\tPayment: ${:.2} Balance: ${:.2} Interest: ${:.2} Principal: ${:.2} Total Interest: ${:.2} Total Cost: ${:.2}",
    		schedule[i].payment_number,
    		schedule[i].payment,
    		schedule[i].balance,
    		schedule[i].interest,
    		schedule[i].principal,
    		schedule[i].running_interest,
    		schedule[i].total_cost);
    }
    
}

fn get_input<T: std::str::FromStr>() -> Result<T, T::Err> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	input.trim().parse::<T>()
}