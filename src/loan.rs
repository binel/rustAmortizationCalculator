pub struct Loan
{
	/// The total amount of the loan at the time it was taken out 
	pub original_principal: f64,
	
	/// The interest rate of the loan (4% = 0.04) 
	pub interest_rate: f64,
	
	/// The length of the loan in years
	pub term_years: i32,
}