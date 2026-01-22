// fn main() {
//     let result = addition(4,5);
//     println!("Hello, World {}!", result);

//     let mut expenses = Vec::new();
//     add_expense(&mut expenses, 15.0, "gift", "2025-12-29");
//     add_expense(&mut expenses, 30.0, "food", "2025-12-30");
// }

// fn addition(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn real_caculator(){}


// // expense_tracker
// // -add amount, category and date
// // view expeneses by date
// // view expenses by category
// struct Expense {
//         amount: f64,
//         category: String,
//         date: String,
//     }


// impl Expense {
//         fn new(amount: f64, category: &str, date: &str) -> Expense {
//             Expense {
//                 amount,
//                 category: category.to_string(),
//                 date: date.to_string(),
//             }
//         }

//     }
// fn add_expense(expeneses: &mut Vec<Expense>, amount: f64, category: &str, date: &str) {
//         let expense = Expense::new(amount, category, date);
//         expenses.push(expense);
//     }
// fn view_expenses_by_date(expenses: &Vec<Expense>, date: &str) -> Vec<&Expense> {
//         expenses.iter().filter(|e| e.date == date).collect()
//     }
// fn calculate_total(expeneses: &Vec<Expense>) -> f64 {
//     expenses.iter().map(|e| e.amount).sum()
// }

// fn get_by_category<'a>(expenses: &'a Vec<Expense>, category: &str) -> Vec<&'a Expense> {
//     expenses.iter().filter(|e| e.category == category).collect()    
// }

// fn count_by_category(expenses: &Vec<Expense>, category: &str) -> usize {
//     expenses.iter().filter(|e| e.category == category).count()
// }

// fn find_max(expenses: &Vec<Expense>) -> Option<&Expense> {
//     expenses.iter().max_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap())

// }



// #[cfg(test)]

// mod tests {
//     use super::*;
//     #[test]
//     fn test_addition() {
//         assert_eq!(addition(2, 3), 5);
//     } 

//     #[test]
//     fn test_add_expense() {
//         let mut expenses =Vec::new();
//         add_expense(&mut expenses, 20.0, "food", "2024-06-01");
//         assert_eq!(expenses.len(), 1);
//         assert_eq!(expenses[0].amount, 20.0);
//         assert_eq!(expenses[0].category, "food");
//     }
// }



// Entry point of the application

// Declare modules (tells Rust about other files)
mod expense;      // Looks for expense.rs
mod operations;   // Looks for operations.rs

// Import what we need from our modules
use expense::Expense;
use operations::*;

fn main() {
    println!("=== Expense Tracker ===\n");

    // Create a list to store expenses
    let mut expenses = Vec::new();

    // Add some expenses
    add_expense(&mut expenses, 45.50, "food", "2026-01-08");
    add_expense(&mut expenses, 20.00, "transport", "2026-01-08");
    add_expense(&mut expenses, 100.00, "rent", "2026-01-08");
    add_expense(&mut expenses, 30.00, "food", "2026-01-07");

    // Display all expenses
    println!("All Expenses:");
    for expense in &expenses {
        expense.display();
    }

    // Calculate total
    println!("\nTotal: ${:.2}", calculate_total(&expenses));

    // View today's expenses
    println!("\nExpenses for 2026-01-08:");
    let today = view_expenses_by_date(&expenses, "2026-01-08");
    for expense in today {
        expense.display();
    }

    // Get food expenses
    println!("\nFood expenses:");
    let food = get_by_category(&expenses, "food");
    for expense in food {
        expense.display();
    }
    println!("Food total: ${:.2}", total_by_category(&expenses, "food"));

    // Find max expense
    if let Some(max) = find_max(&expenses) {
        println!("\nMost expensive:");
        max.display();
    }

    // Count by category
    println!("\nFood expense count: {}", count_by_category(&expenses, "food"));
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expense() {
        let mut expenses = Vec::new();
        add_expense(&mut expenses, 20.0, "food", "2024-06-01");
        assert_eq!(expenses.len(), 1);
        assert_eq!(expenses[0].amount, 20.0);
        assert_eq!(expenses[0].category, "food");
    }

    #[test]
    fn test_calculate_total() {
        let mut expenses = Vec::new();
        add_expense(&mut expenses, 10.0, "food", "2024-06-01");
        add_expense(&mut expenses, 20.0, "transport", "2024-06-01");
        assert_eq!(calculate_total(&expenses), 30.0);
    }

    #[test]
    fn test_find_max() {
        let mut expenses = Vec::new();
        add_expense(&mut expenses, 10.0, "food", "2024-06-01");
        add_expense(&mut expenses, 50.0, "rent", "2024-06-01");
        add_expense(&mut expenses, 20.0, "transport", "2024-06-01");
        
        let max = find_max(&expenses).unwrap();
        assert_eq!(max.amount, 50.0);
    }
}