/// A single entry in an amortization schedule 
pub struct AmortizationPayment {
	/// Which payment in the schedule this is 
	pub payment_number: i32,

	/// The total amount due for this payment (principal + interest) 
	pub payment: f64,
	
	/// The amount of the payment put towards paying down the principal of the 
	/// loan - the amount this payment reduces the balance of the loan
	pub principal: f64,
	
	/// The interest payment for this period of the loan 
	/// (balance * monthly interest rate) 
	pub interest: f64,
	
	/// The remaining balance of the loan after this payment has been applied 
	pub balance: f64,
	
	/// The total amount of interest that has been paid after this payment
	pub running_interest: f64,
	
	/// The total cost of the loan after this payment - the sum of this and all prior 
	/// interest + principal payments
	pub total_cost: f64,
}

pub fn amortization_schedule_from_loan(loan: super::loan::NaiveLoan) -> Vec<AmortizationPayment> {
	amortization_schedule(loan.original_principal, loan.interest_rate, loan.term_years * 12)
}

pub fn amortization_schedule(principal: f64, interest: f64, num_payments: i32) -> Vec<AmortizationPayment> {
	let mut schedule = Vec::new();	

	let monthly_rate: f64 = interest / 12.0;
    let rp = (1.0 + monthly_rate).powi(num_payments);
    let monthly_payment: f64 = principal * ((monthly_rate * rp)/(rp - 1.0));
    
    let mut balance = principal; 
    let mut interest_accumulator: f64 = 0.0;
    let mut principal_accumulator: f64 = 0.0;
    
    for i in 0..num_payments
    {
    	let interest = monthly_rate * balance;
    	let principal = monthly_payment - interest; 
    	
    	balance -= principal; 
    	interest_accumulator += interest;
    	principal_accumulator += principal;
    	
    	schedule.push(AmortizationPayment{
    		payment_number: i + 1,
    		payment: monthly_payment,
    		principal,
    		interest,
    		balance,
    		running_interest: interest_accumulator,
    		total_cost: principal_accumulator + interest_accumulator,
    	});
    } 
    
    schedule
}