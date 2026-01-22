// This module contains functions that operate on expenses

use crate::expense::Expense;

/// Add an expense to the list
pub fn add_expense(expenses: &mut Vec<Expense>, amount: f64, category: &str, date: &str) {
    let expense = Expense::new(amount, category, date);
    expenses.push(expense);
}

/// View expenses by date
pub fn view_expenses_by_date<'a>(expenses: &'a Vec<Expense>, date: &str) -> Vec<&'a Expense> {
    expenses.iter().filter(|e| e.date == date).collect()
}

/// Calculate total of all expenses
pub fn calculate_total(expenses: &Vec<Expense>) -> f64 {
    expenses.iter().map(|e| e.amount).sum()
}

/// Get expenses by category
pub fn get_by_category<'a>(expenses: &'a Vec<Expense>, category: &str) -> Vec<&'a Expense> {
    expenses.iter().filter(|e| e.category == category).collect()
}

/// Count expenses in a category
pub fn count_by_category(expenses: &Vec<Expense>, category: &str) -> usize {
    expenses.iter().filter(|e| e.category == category).count()
}

/// Find the most expensive expense
pub fn find_max(expenses: &Vec<Expense>) -> Option<&Expense> {
    expenses.iter()
        .max_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap_or(std::cmp::Ordering::Equal))
}

/// Find the least expensive expense
pub fn find_min(expenses: &Vec<Expense>) -> Option<&Expense> {
    expenses.iter()
        .min_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap_or(std::cmp::Ordering::Equal))
}

/// Get total for a specific category
pub fn total_by_category(expenses: &Vec<Expense>, category: &str) -> f64 {
    expenses.iter()
        .filter(|e| e.category == category)
        .map(|e| e.amount)
        .sum()
}