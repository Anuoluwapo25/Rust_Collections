// This module defines the Expense struct and its methods

#[derive(Debug, Clone)]
pub struct Expense {
    pub amount: f64,
    pub category: String,
    pub date: String,
}

impl Expense {
    /// Creates a new Expense
    pub fn new(amount: f64, category: &str, date: &str) -> Expense {
        Expense {
            amount,
            category: category.to_string(),
            date: date.to_string(),
        }
    }

    /// Display an expense nicely
    pub fn display(&self) {
        println!("${:.2} - {} ({})", self.amount, self.category, self.date);
    }
}