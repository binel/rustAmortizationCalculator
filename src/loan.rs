use chrono::NaiveDate;

/// A simple loan struct for simple calculations
pub struct NaiveLoan
{
	/// The total amount of the loan at the time it was taken out 
	pub original_principal: f64,
	
	/// The interest rate of the loan (4% = 0.04) 
	pub interest_rate: f64,
	
	/// The length of the loan in years
	pub term_years: i32,
}

/// Tries to fully model a loan - including the exact dates of 
/// what happened when, and a full prior payment history
pub struct Loan
{
	/// The total amount of the loan at the time it was taken out 
	pub original_principal: f64,
	
	/// The interest rate of the loan (4% = 0.04) 
	pub interest_rate: f64,
	
	/// The length of the loan in years
	pub term_years: i32,
	
	/// The date of the first payment of the loan
	pub first_payment_date: NaiveDate,
	
	/// The current date in the progress of the loan 
	pub current_date: NaiveDate,
	
	/// All prior payments that have been made towards the loan
	pub payments: Vec<Payment>,
}

/// A single payment made to a loan.
pub struct Payment
{
	/// The date the payment was made 
	pub date: NaiveDate,
	
	/// The total amount paid
	pub amount: f64,
	
	/// The remaining balance of the loan after this payment 
	pub balance: f64,
	
	/// The amount of this payment that was directed towards paying off 
	/// the principal of the loan 
	pub principal_payment: f64,
	
	/// The amount of this payment that was directed towards paying off 
	/// accrued interest
	pub interest_payment: f64,
}