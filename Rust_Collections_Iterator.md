# Complete Rust Collections & Iterator Methods Guide

## Table of Contents
1. [Data Structures (Collections)](#data-structures-collections)
2. [Iterator Methods](#iterator-methods)
3. [Closures & Function Syntax](#closures--function-syntax)
4. [Common Patterns & Use Cases](#common-patterns--use-cases)

---

# Data Structures (Collections)

These are types that store multiple values.

## 1. Vec<T> - Dynamic Array (List)

**What it is:** A growable list of items of the same type.

**When to use:**
- ✅ You need an ordered collection
- ✅ You need to access items by index
- ✅ You need dynamic size (grows/shrinks)
- ✅ Most common collection type

**Example:**
```rust
// Create
let mut numbers = Vec::new();
let mut numbers: Vec<i32> = Vec::new();
let mut numbers = vec![1, 2, 3, 4, 5];  // vec! macro

// Add items
numbers.push(6);
numbers.push(7);

// Access by index
let first = numbers[0];          // Panics if out of bounds
let first = numbers.get(0);      // Returns Option<&T>

// Remove items
numbers.pop();                   // Removes last, returns Option<T>
numbers.remove(0);               // Removes at index

// Iterate
for num in &numbers {
    println!("{}", num);
}

// Common methods
numbers.len();                   // Length
numbers.is_empty();              // Check if empty
numbers.contains(&5);            // Check if contains value
numbers.clear();                 // Remove all items
```

**Real-world use case:**
```rust
// List of expenses
let mut expenses: Vec<Expense> = Vec::new();
expenses.push(Expense::new(50.0, "food", "2026-01-08"));
```

---

## 2. HashMap<K, V> - Key-Value Store (Dictionary)

**What it is:** Stores pairs of keys and values. Fast lookups by key.

**When to use:**
- ✅ You need to look up values by a key
- ✅ Each key is unique
- ✅ Order doesn't matter
- ✅ Need O(1) average lookup time

**Example:**
```rust
use std::collections::HashMap;

// Create
let mut scores = HashMap::new();

// Insert
scores.insert("Alice", 100);
scores.insert("Bob", 95);
scores.insert("Carol", 87);

// Get value by key
let alice_score = scores.get("Alice");           // Some(&100)
let alice_score = scores.get("Alice").unwrap();  // 100 (panics if not found)
let dave_score = scores.get("Dave");             // None

// Check if key exists
if scores.contains_key("Alice") {
    println!("Alice exists!");
}

// Update value
scores.insert("Alice", 105);  // Overwrites old value

// Insert only if key doesn't exist
scores.entry("David").or_insert(90);

// Iterate
for (name, score) in &scores {
    println!("{}: {}", name, score);
}

// Remove
scores.remove("Bob");

// Common methods
scores.len();
scores.is_empty();
scores.clear();
scores.keys();         // Iterator over keys
scores.values();       // Iterator over values
```

**Real-world use cases:**
```rust
// 1. Total expenses by category
let mut category_totals: HashMap<String, f64> = HashMap::new();
category_totals.insert("food".to_string(), 150.0);
category_totals.insert("transport".to_string(), 50.0);

// 2. Counting occurrences
let text = "hello world hello rust";
let mut word_count = HashMap::new();
for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}
// Result: {"hello": 2, "world": 1, "rust": 1}

// 3. User database
let mut users: HashMap<u32, String> = HashMap::new();
users.insert(1, "Alice".to_string());
users.insert(2, "Bob".to_string());
```

---

## 3. HashSet<T> - Unique Values (Set)

**What it is:** A collection of unique values. No duplicates allowed.

**When to use:**
- ✅ You need unique values only
- ✅ You need to check membership quickly
- ✅ Order doesn't matter
- ✅ Need to do set operations (union, intersection)

**Example:**
```rust
use std::collections::HashSet;

// Create
let mut tags = HashSet::new();

// Insert
tags.insert("rust");
tags.insert("programming");
tags.insert("rust");  // Duplicate ignored!

// Check membership
if tags.contains("rust") {
    println!("Has rust tag!");
}

// Remove
tags.remove("rust");

// Set operations
let set1: HashSet<_> = [1, 2, 3, 4].iter().collect();
let set2: HashSet<_> = [3, 4, 5, 6].iter().collect();

let union: HashSet<_> = set1.union(&set2).collect();          // {1,2,3,4,5,6}
let intersection: HashSet<_> = set1.intersection(&set2).collect();  // {3,4}
let difference: HashSet<_> = set1.difference(&set2).collect();      // {1,2}

// Common methods
tags.len();
tags.is_empty();
tags.clear();
```

**Real-world use cases:**
```rust
// 1. Unique categories
let mut categories = HashSet::new();
for expense in &expenses {
    categories.insert(&expense.category);
}

// 2. Remove duplicates from Vec
let numbers = vec![1, 2, 2, 3, 3, 4];
let unique: HashSet<_> = numbers.into_iter().collect();
let unique_vec: Vec<_> = unique.into_iter().collect();  // [1,2,3,4]

// 3. Track visited items
let mut visited = HashSet::new();
if !visited.contains(&item_id) {
    visited.insert(item_id);
    process_item(item_id);
}
```

---

## 4. BTreeMap<K, V> - Sorted Key-Value Store

**What it is:** Like HashMap but keys are kept in sorted order.

**When to use:**
- ✅ You need keys in sorted order
- ✅ You need to get ranges of keys
- ✅ You iterate in order frequently

**Example:**
```rust
use std::collections::BTreeMap;

let mut scores = BTreeMap::new();
scores.insert("Carol", 87);
scores.insert("Alice", 100);
scores.insert("Bob", 95);

// Iterates in sorted key order
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
// Output: Alice: 100, Bob: 95, Carol: 87 (alphabetical!)

// Get range
for (name, score) in scores.range("A".."C") {
    // Only Alice and Bob
}
```

**Use case:** When order matters (leaderboards, alphabetical lists).

---

## 5. BTreeSet<T> - Sorted Unique Values

**What it is:** Like HashSet but values are kept in sorted order.

**When to use:**
- ✅ You need unique values in sorted order

**Example:**
```rust
use std::collections::BTreeSet;

let mut numbers = BTreeSet::new();
numbers.insert(5);
numbers.insert(2);
numbers.insert(8);
numbers.insert(2);  // Ignored

// Iterates in sorted order: 2, 5, 8
for num in &numbers {
    println!("{}", num);
}
```

---

## 6. VecDeque<T> - Double-Ended Queue

**What it is:** Like Vec but efficient at adding/removing from both ends.

**When to use:**
- ✅ Need efficient push/pop from both front and back
- ✅ Implementing queues or sliding windows

**Example:**
```rust
use std::collections::VecDeque;

let mut queue = VecDeque::new();

// Add to back
queue.push_back(1);
queue.push_back(2);

// Add to front
queue.push_front(0);

// Remove from front (FIFO queue)
let first = queue.pop_front();  // Some(0)

// Remove from back
let last = queue.pop_back();    // Some(2)
```

**Use case:** Task queues, breadth-first search, sliding window algorithms.

---

## 7. String vs &str

**String** - Owned, growable text
**&str** - Borrowed string slice

**Example:**
```rust
// String (owned)
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');

// &str (borrowed)
let s: &str = "hello";  // String literal
let slice: &str = &s[0..5];  // Slice of String

// Convert between them
let string: String = "hello".to_string();
let str_ref: &str = &string;
```

---

## Quick Reference Table: Collections

| Type | Ordered? | Unique? | Key-Value? | Use When |
|------|----------|---------|------------|----------|
| **Vec<T>** | ✅ Yes | ❌ No | ❌ No | General list of items |
| **HashMap<K,V>** | ❌ No | ✅ Keys | ✅ Yes | Fast key lookups |
| **HashSet<T>** | ❌ No | ✅ Yes | ❌ No | Unique values, membership test |
| **BTreeMap<K,V>** | ✅ Sorted | ✅ Keys | ✅ Yes | Sorted key-value pairs |
| **BTreeSet<T>** | ✅ Sorted | ✅ Yes | ❌ No | Sorted unique values |
| **VecDeque<T>** | ✅ Yes | ❌ No | ❌ No | Queue, double-ended |
| **String** | ✅ Yes | ❌ No | ❌ No | Owned text |

---

# Iterator Methods

These are methods you can call on iterators to transform or consume data.

## Iterator Adapters (Return new iterator)

### 1. map() - Transform Each Item

**What it does:** Applies a function to each item and returns new values.

```rust
let numbers = vec![1, 2, 3, 4];

let doubled: Vec<i32> = numbers.iter()
    .map(|n| n * 2)
    .collect();
// Result: [2, 4, 6, 8]

// Extract fields
let expenses = vec![...];
let amounts: Vec<f64> = expenses.iter()
    .map(|e| e.amount)
    .collect();

// Transform types
let strings: Vec<String> = numbers.iter()
    .map(|n| n.to_string())
    .collect();
// Result: ["1", "2", "3", "4"]
```

**Use cases:**
- Converting types
- Extracting fields from structs
- Applying calculations to each item
- Transforming data shape

---

### 2. filter() - Keep Only Matching Items

**What it does:** Keeps only items where the condition is true.

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let evens: Vec<&i32> = numbers.iter()
    .filter(|n| *n % 2 == 0)
    .collect();
// Result: [2, 4, 6]

// Multiple conditions
let result: Vec<&i32> = numbers.iter()
    .filter(|n| *n > 2 && *n < 5)
    .collect();
// Result: [3, 4]

// Filter expenses
let food: Vec<&Expense> = expenses.iter()
    .filter(|e| e.category == "food")
    .collect();
```

**Use cases:**
- Removing unwanted items
- Finding items matching criteria
- Data filtering and searching

---

### 3. filter_map() - Filter and Transform in One Step

**What it does:** Combines filter and map - transforms and filters out None.

```rust
let strings = vec!["1", "two", "3", "four", "5"];

let numbers: Vec<i32> = strings.iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
// Result: [1, 3, 5] (non-numbers filtered out)
```

**Use cases:**
- Parsing with error handling
- Converting and filtering in one step

---

### 4. enumerate() - Add Index to Each Item

**What it does:** Pairs each item with its index (0, 1, 2, ...).

```rust
let names = vec!["Alice", "Bob", "Carol"];

for (index, name) in names.iter().enumerate() {
    println!("{}: {}", index, name);
}
// Output:
// 0: Alice
// 1: Bob
// 2: Carol

// With collect
let indexed: Vec<(usize, &str)> = names.iter()
    .enumerate()
    .collect();
// Result: [(0, "Alice"), (1, "Bob"), (2, "Carol")]
```

**Use cases:**
- Need index while iterating
- Creating numbered lists
- Tracking position

---

### 5. zip() - Combine Two Iterators

**What it does:** Pairs up items from two iterators.

```rust
let names = vec!["Alice", "Bob", "Carol"];
let scores = vec![100, 95, 87];

let pairs: Vec<_> = names.iter()
    .zip(scores.iter())
    .collect();
// Result: [("Alice", 100), ("Bob", 95), ("Carol", 87)]

// Stops at shortest iterator
let a = vec![1, 2, 3, 4];
let b = vec!["a", "b"];
let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
// Result: [(1, "a"), (2, "b")] - stops at 2 items
```

**Use cases:**
- Combining parallel arrays
- Creating pairs of related data

---

### 6. take() - Take First N Items

**What it does:** Takes only the first N items.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let first_three: Vec<_> = numbers.iter()
    .take(3)
    .collect();
// Result: [1, 2, 3]

// Useful for pagination
let page_size = 10;
let page_1: Vec<_> = items.iter().take(page_size).collect();
```

**Use cases:**
- Pagination
- Limiting results
- Getting first N items

---

### 7. skip() - Skip First N Items

**What it does:** Skips the first N items.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let after_two: Vec<_> = numbers.iter()
    .skip(2)
    .collect();
// Result: [3, 4, 5]

// Pagination: page 2
let page_size = 10;
let page = 2;
let page_items: Vec<_> = items.iter()
    .skip(page_size * (page - 1))
    .take(page_size)
    .collect();
```

**Use cases:**
- Pagination
- Skipping headers
- Processing after offset

---

### 8. take_while() - Take Until Condition Fails

**What it does:** Takes items while condition is true, stops at first false.

```rust
let numbers = vec![1, 2, 3, 4, 1, 2];

let result: Vec<_> = numbers.iter()
    .take_while(|n| **n < 4)
    .collect();
// Result: [1, 2, 3] (stops at 4)
```

---

### 9. skip_while() - Skip Until Condition Fails

**What it does:** Skips items while condition is true, then takes rest.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let result: Vec<_> = numbers.iter()
    .skip_while(|n| **n < 3)
    .collect();
// Result: [3, 4, 5]
```

---

### 10. chain() - Combine Two Iterators Sequentially

**What it does:** Connects two iterators one after another.

```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];

let combined: Vec<_> = a.iter()
    .chain(b.iter())
    .collect();
// Result: [1, 2, 3, 4, 5, 6]
```

**Use cases:**
- Combining multiple lists
- Processing different sources together

---

### 11. rev() - Reverse Iterator

**What it does:** Reverses the iteration order.

```rust
let numbers = vec![1, 2, 3, 4];

let reversed: Vec<_> = numbers.iter()
    .rev()
    .collect();
// Result: [4, 3, 2, 1]
```

---

### 12. flatten() - Flatten Nested Iterators

**What it does:** Flattens nested collections into a single iterator.

```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];

let flat: Vec<_> = nested.iter()
    .flatten()
    .collect();
// Result: [1, 2, 3, 4, 5, 6]

// Flatten Option
let options = vec![Some(1), None, Some(2), Some(3)];
let values: Vec<_> = options.into_iter()
    .flatten()
    .collect();
// Result: [1, 2, 3] (Nones removed)
```

---

### 13. flat_map() - Map Then Flatten

**What it does:** Like map, but flattens the results.

```rust
let words = vec!["hello world", "foo bar"];

let chars: Vec<_> = words.iter()
    .flat_map(|s| s.chars())
    .collect();
// Result: ['h','e','l','l','o',' ','w','o','r','l','d','f','o','o',' ','b','a','r']
```

---

### 14. inspect() - Debug/Peek at Items

**What it does:** Lets you see items passing through without changing them.

```rust
let numbers = vec![1, 2, 3, 4];

let result: Vec<_> = numbers.iter()
    .inspect(|n| println!("Processing: {}", n))
    .map(|n| n * 2)
    .inspect(|n| println!("After doubling: {}", n))
    .collect();
```

**Use case:** Debugging iterator chains

---

## Iterator Consumers (Produce Final Value)

### 15. collect() - Gather Into Collection

**What it does:** Collects iterator into a collection (Vec, HashMap, etc.).

```rust
let numbers = vec![1, 2, 3];

// Into Vec
let vec: Vec<i32> = numbers.iter().map(|n| n * 2).collect();

// Into HashMap
let pairs = vec![("a", 1), ("b", 2)];
let map: HashMap<_, _> = pairs.into_iter().collect();

// Into HashSet
let set: HashSet<_> = numbers.into_iter().collect();

// Into String
let chars = vec!['h', 'e', 'l', 'l', 'o'];
let word: String = chars.into_iter().collect();
```

---

### 16. sum() - Add All Items

**What it does:** Adds all numbers together.

```rust
let numbers = vec![1, 2, 3, 4, 5];
let total: i32 = numbers.iter().sum();
// Result: 15

// With expenses
let total: f64 = expenses.iter()
    .map(|e| e.amount)
    .sum();
```

---

### 17. product() - Multiply All Items

**What it does:** Multiplies all numbers together.

```rust
let numbers = vec![1, 2, 3, 4];
let result: i32 = numbers.iter().product();
// Result: 24 (1*2*3*4)
```

---

### 18. count() - Count Items

**What it does:** Counts how many items are in the iterator.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let count = numbers.iter()
    .filter(|n| **n > 2)
    .count();
// Result: 3 (values 3, 4, 5)
```

---

### 19. max() / min() - Find Maximum/Minimum

**What it does:** Finds the largest or smallest item.

```rust
let numbers = vec![1, 5, 3, 9, 2];

let max = numbers.iter().max();  // Some(&9)
let min = numbers.iter().min();  // Some(&1)

// Returns Option because iterator might be empty
```

---

### 20. max_by() / min_by() - Find Max/Min with Custom Comparison

**What it does:** Finds max/min using a custom comparison function.

```rust
let expenses = vec![...];

let most_expensive = expenses.iter()
    .max_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());

// By key
let most_expensive = expenses.iter()
    .max_by_key(|e| e.amount);
```

---

### 21. find() - Find First Matching Item

**What it does:** Returns first item where condition is true.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let first_even = numbers.iter()
    .find(|n| *n % 2 == 0);
// Result: Some(&2)

let big_number = numbers.iter()
    .find(|n| **n > 10);
// Result: None
```

---

### 22. position() - Find Index of First Match

**What it does:** Returns index of first matching item.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let pos = numbers.iter()
    .position(|n| *n == 3);
// Result: Some(2) (index 2)
```

---

### 23. any() - Check if Any Match

**What it does:** Returns true if any item matches condition.

```rust
let numbers = vec![1, 2, 3, 4, 5];

let has_even = numbers.iter().any(|n| n % 2 == 0);
// Result: true

let has_big = numbers.iter().any(|n| *n > 10);
// Result: false
```

---

### 24. all() - Check if All Match

**What it does:** Returns true if all items match condition.

```rust
let numbers = vec![2, 4, 6, 8];

let all_even = numbers.iter().all(|n| n % 2 == 0);
// Result: true

let all_positive = numbers.iter().all(|n| *n > 0);
// Result: true
```

---

### 25. fold() - Reduce to Single Value

**What it does:** Accumulates values using a function.

```rust
let numbers = vec![1, 2, 3, 4];

// Sum (manual)
let sum = numbers.iter()
    .fold(0, |acc, n| acc + n);
// Result: 10

// Product
let product = numbers.iter()
    .fold(1, |acc, n| acc * n);
// Result: 24

// Build string
let s = numbers.iter()
    .fold(String::new(), |mut acc, n| {
        acc.push_str(&n.to_string());
        acc.push(',');
        acc
    });
// Result: "1,2,3,4,"
```

**Use cases:**
- Custom aggregations
- Building complex results
- When sum() or count() aren't enough

---

### 26. for_each() - Do Something with Each Item

**What it does:** Performs an action on each item (returns nothing).

```rust
let numbers = vec![1, 2, 3];

numbers.iter()
    .for_each(|n| println!("{}", n));

// Equivalent to for loop
for n in &numbers {
    println!("{}", n);
}
```

---

### 27. partition() - Split Into Two Groups

**What it does:** Splits items into two collections based on condition.

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let (evens, odds): (Vec<_>, Vec<_>) = numbers.iter()
    .partition(|n| *n % 2 == 0);
// evens: [2, 4, 6]
// odds:  [1, 3, 5]
```

---

## Quick Reference: Iterator Methods

| Method | Type | Returns | Use For |
|--------|------|---------|---------|
| **map()** | Adapter | Iterator | Transform each item |
| **filter()** | Adapter | Iterator | Keep matching items |
| **filter_map()** | Adapter | Iterator | Transform & filter |
| **enumerate()** | Adapter | Iterator | Add index |
| **zip()** | Adapter | Iterator | Combine two iterators |
| **take()** | Adapter | Iterator | First N items |
| **skip()** | Adapter | Iterator | Skip N items |
| **chain()** | Adapter | Iterator | Combine iterators |
| **rev()** | Adapter | Iterator | Reverse order |
| **flatten()** | Adapter | Iterator | Flatten nested |
| **inspect()** | Adapter | Iterator | Debug peek |
| **collect()** | Consumer | Collection | Gather into Vec, HashMap, etc |
| **sum()** | Consumer | Number | Add all |
| **count()** | Consumer | usize | Count items |
| **max()** | Consumer | Option<T> | Find maximum |
| **min()** | Consumer | Option<T> | Find minimum |
| **find()** | Consumer | Option<T> | First match |
| **any()** | Consumer | bool | Check if any match |
| **all()** | Consumer | bool | Check if all match |
| **fold()** | Consumer | T | Custom accumulation |

---

# Closures & Function Syntax

## What are Closures?

Closures are **anonymous functions** (functions without names) that can **capture variables** from their environment.

### Basic Syntax

```rust
// Closure syntax:
|parameters| expression
|parameters| { statements; return_value }

// Examples:
let add_one = |x| x + 1;
let add = |x, y| x + y;
let print = |msg| println!("{}", msg);

// With type annotations
let add: fn(i32, i32) -> i32 = |x, y| x + y;

// Multi-line
let complex = |x| {
    let doubled = x * 2;
    let added = doubled + 10;
    added
};
```

### Comparison: Function vs Closure

```rust
// Regular function
fn add_one(x: i32) -> i32 {
    x + 1
}

// Closure (short)
let add_one = |x| x + 1;

// Closure (explicit types)
let add_one = |x: i32| -> i32 { x + 1 };

// Using them
add_one(5);         // Function: 6
add_one(5);         // Closure: 6
```

### Capturing Environment

```rust
// Closures can capture variables from surrounding scope
let multiplier = 10;

// This closure "captures" multiplier
let multiply = |x| x * multiplier;

println!("{}", multiply(5));  // 50

// Regular functions CANNOT do this!
// This would NOT compile:
// fn multiply(x: i32) -> i32 {
//     x * multiplier  // Error: can't find multiplier
// }
```

### Closure Types

```rust
// 1. FnOnce - Consumes captured variables (can only call once)
let s = String::from("hello");
let consume = || {
    drop(s);  // Takes ownership of s
};
consume();    // OK
// consume(); // Error! Can't call again

// 2. FnMut - Mutably borrows captured variables
let mut count = 0;
let mut increment = || {
    count += 1;  // Mutably borrows count
};
increment();  // count is now 1
increment();  // count is now 2

// 3. Fn - Immutably borrows (most common)
let x = 5;
let print_x = || println!("{}", x);  // Immutably borrows x
print_x();  // Can call many times
print_x();
```

### Common Closure Patterns

```rust
// Pattern 1: With iterator methods
let numbers = vec![1, 2, 3, 4];
numbers.iter().map(|n| n * 2).collect();

// Pattern 2: As function parameter
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let double = |x| x * 2;
apply(double, 5);  // 10

// Pattern 3: Returning closures
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

let times_three = make_multiplier(3);
times_three(10);  // 30
```

### move Keyword

```rust
// Without move: closure borrows
let x = 5;
let print = || println!("{}", x);
// x is still accessible here

// With move: closure takes ownership
let x = 5;
let print = move || println!("{}", x);
// x moved into closure, can't use x here anymore
```

---

# Common Patterns & Use Cases

## Pattern 1: Filter → Map → Collect

```rust
// Get amounts of food expenses over $20
let amounts: Vec<f64> = expenses.iter()
    .filter(|e| e.category == "food")
    .filter(|e| e.amount > 20.0)
    .map(|e| e.amount)
    .collect();
```

## Pattern 2: Group By (Using HashMap)

```rust
// Group expenses by category
let mut by_category: HashMap<String, Vec<Expense>> = HashMap::new();

for expense in expenses {
    by_category
        .entry(expense.category.clone())
        .or_insert(Vec::new())
        .push(expense);
}

// Or with fold
let by_category = expenses.into_iter()
    .fold(HashMap::new(), |mut acc, expense| {
        acc.entry(expense.category.clone())
            .or_insert(Vec::new())
            .push(expense);
        acc
    });
```

## Pattern 3: Sum By Category

```rust
// Total expenses per category
let mut totals: HashMap<String, f64> = HashMap::new();

for expense in &expenses {
    *totals.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
}
```

## Pattern 4: Find and Process

```rust
// Find and update first matching expense
if let Some(expense) = expenses.iter_mut()
    .find(|e| e.category == "food" && e.amount > 50.0) {
    expense.amount *= 0.9;  // 10% discount
}
```

## Pattern 5: Chaining Multiple Operations

```rust
let result: f64 = expenses.iter()
    .filter(|e| e.date == "2026-01-08")          // Only today
    .filter(|e| e.category == "food")            // Only food
    .map(|e| e.amount)                           // Get amounts
    .filter(|&amount| amount > 10.0)             // Over $10
    .map(|amount| amount * 1.1)                  // Add 10% tax
    .sum();                                      // Total
```

## Pattern 6: Count Occurrences

```rust
let text = "the quick brown fox jumps over the lazy dog";
let mut word_count: HashMap<&str, usize> = HashMap::new();

for word in text.split_whitespace() {
    *word_count.entry(word).or_insert(0) += 1;
}
```

## Pattern 7: Unique Values

```rust
// Get unique categories
let unique_categories: HashSet<_> = expenses.iter()
    .map(|e| &e.category)
    .collect();

// Or as Vec
let unique_vec: Vec<_> = unique_categories.into_iter().collect();
```

## Pattern 8: Pagination

```rust
let page_size = 10;
let page = 2;

let page_items: Vec<_> = all_items.iter()
    .skip(page_size * (page - 1))
    .take(page_size)
    .collect();
```

## Pattern 9: Top N Items

```rust
// Get top 5 most expensive
let mut sorted = expenses.clone();
sorted.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap());
let top_5: Vec<_> = sorted.iter().take(5).collect();
```

## Pattern 10: All/Any Checks

```rust
// Check if all expenses are positive
let all_positive = expenses.iter().all(|e| e.amount > 0.0);

// Check if any expense is over budget
let over_budget = expenses.iter().any(|e| e.amount > 1000.0);

// Check if none are negative
let none_negative = !expenses.iter().any(|e| e.amount < 0.0);
```

---

# Summary

## Key Takeaways

1. **Collections**: Choose based on your needs
   - **Vec** for ordered lists
   - **HashMap** for key-value lookups
   - **HashSet** for unique values
   - **BTree*** for sorted data

2. **Iterator Methods**: Chain them together
   - **Adapters** (map, filter) transform iterators
   - **Consumers** (collect, sum, count) produce final values

3. **Closures**: Anonymous functions
   - Syntax: `|params| expression`
   - Can capture environment variables
   - Used heavily with iterators

## Common Workflow

```rust
collection.iter()          // Start iterating
    .filter(|x| ...)       // Keep some items
    .map(|x| ...)          // Transform items
    .collect()             // Gather results
```

## When to Use What

- Need unique items? → **HashSet**
- Need key-value pairs? → **HashMap**
- Need ordered list? → **Vec**
- Need sorted data? → **BTreeMap/BTreeSet**
- Transform items? → **map()**
- Filter items? → **filter()**
- Get total? → **sum()**
- Count items? → **count()**
- Find item? → **find()**

---

This is your complete reference! Bookmark it and refer back when needed. 🚀
