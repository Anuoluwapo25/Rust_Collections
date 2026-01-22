# Rust Quick Cheat Sheet

## Collections (Data Structures)

```rust
// Vec - Dynamic array/list
let mut v = vec![1, 2, 3];
v.push(4);                    // Add
v.pop();                      // Remove last
v[0];                         // Access by index
v.len();                      // Length

// HashMap - Key-value store
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");   // Add
map.get("key");               // Get (returns Option)
map.remove("key");            // Remove
map.contains_key("key");      // Check exists

// HashSet - Unique values
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(1);                // Add
set.contains(&1);             // Check membership
set.remove(&1);               // Remove

// String
let mut s = String::from("hello");
s.push_str(" world");         // Append string
s.push('!');                  // Append char
```

## Iterator Adapters (Return Iterator)

```rust
let nums = vec![1, 2, 3, 4, 5];

// Transform
nums.iter().map(|n| n * 2)                    // [2,4,6,8,10]

// Filter
nums.iter().filter(|n| *n > 2)                // [3,4,5]

// Both
nums.iter()
    .filter(|n| *n > 2)
    .map(|n| n * 2)                           // [6,8,10]

// Add index
nums.iter().enumerate()                       // [(0,1),(1,2),(2,3),...]

// Combine two
let a = vec![1, 2];
let b = vec![3, 4];
a.iter().zip(b.iter())                        // [(1,3),(2,4)]

// Take first N
nums.iter().take(3)                           // [1,2,3]

// Skip first N
nums.iter().skip(2)                           // [3,4,5]

// Reverse
nums.iter().rev()                             // [5,4,3,2,1]

// Chain two iterators
a.iter().chain(b.iter())                      // [1,2,3,4]

// Flatten nested
let nested = vec![vec![1,2], vec![3,4]];
nested.iter().flatten()                       // [1,2,3,4]
```

## Iterator Consumers (Return Final Value)

```rust
let nums = vec![1, 2, 3, 4, 5];

// Collect into Vec
nums.iter().collect::<Vec<_>>()               // Vec

// Sum
nums.iter().sum::<i32>()                      // 15

// Count
nums.iter().count()                           // 5

// Max/Min
nums.iter().max()                             // Some(&5)
nums.iter().min()                             // Some(&1)

// Find first match
nums.iter().find(|n| **n > 3)                 // Some(&4)

// Check any
nums.iter().any(|n| *n > 3)                   // true

// Check all
nums.iter().all(|n| *n > 0)                   // true

// Fold (reduce)
nums.iter().fold(0, |acc, n| acc + n)         // 15

// For each (side effects)
nums.iter().for_each(|n| println!("{}", n))
```

## Closures

```rust
// Basic syntax
|x| x + 1                           // One parameter
|x, y| x + y                        // Multiple parameters
|x| { let y = x * 2; y + 1 }       // Multiple statements

// With types
|x: i32| -> i32 { x + 1 }

// Capturing variables
let factor = 10;
let multiply = |x| x * factor;      // Captures 'factor'

// Using with iterators
vec![1,2,3].iter().map(|n| n * 2)
vec![1,2,3].iter().filter(|n| *n > 1)
```

## Common Patterns

```rust
// Filter + Map + Collect
let result: Vec<i32> = items.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.value)
    .collect();

// Group by category
let mut groups: HashMap<String, Vec<Item>> = HashMap::new();
for item in items {
    groups.entry(item.category.clone())
        .or_insert(Vec::new())
        .push(item);
}

// Count occurrences
let mut counts: HashMap<String, usize> = HashMap::new();
for word in words {
    *counts.entry(word).or_insert(0) += 1;
}

// Get unique values
let unique: HashSet<_> = items.iter()
    .map(|x| &x.category)
    .collect();

// Sum by category
let total: f64 = items.iter()
    .filter(|x| x.category == "food")
    .map(|x| x.amount)
    .sum();

// Pagination
let page: Vec<_> = all_items.iter()
    .skip(page_size * (page_num - 1))
    .take(page_size)
    .collect();

// Top N
let mut sorted = items.clone();
sorted.sort_by_key(|x| x.score);
let top_5: Vec<_> = sorted.iter().rev().take(5).collect();
```

## Quick Decision Guide

**Choose Collection:**
- Ordered list → `Vec<T>`
- Lookup by key → `HashMap<K, V>`
- Unique values → `HashSet<T>`
- Need sorting → `BTreeMap<K, V>` or `BTreeSet<T>`

**Choose Iterator Method:**
- Transform items → `.map()`
- Keep some items → `.filter()`
- Transform & filter → `.filter_map()`
- Get total → `.sum()`
- Count items → `.count()`
- Find one → `.find()`
- Check condition → `.any()` or `.all()`

**Closure Syntax:**
```rust
// Short:  |params| expression
|x| x + 1

// Long:   |params| { statements }
|x| {
    let y = x * 2;
    y + 1
}
```

## Method Chaining Tips

```rust
// Start → Adapt → Adapt → Consume
vec.iter()          // Start
   .filter()        // Adapt
   .map()           // Adapt
   .collect()       // Consume

// Can chain many adapters
vec.iter()
   .filter(|x| x > 0)
   .map(|x| x * 2)
   .filter(|x| x < 100)
   .take(10)
   .collect()

// Only ONE consumer at the end
// ✅ Good:
vec.iter().map(|x| x * 2).collect()

// ❌ Bad:
vec.iter().map(|x| x * 2).collect().sum()  // Can't do this
```

## Remember

- **Collections** = Data structures that hold multiple values
- **Iterators** = Ways to process those values
- **Closures** = Anonymous functions `|x| x + 1`
- **Adapters** = Transform iterators (return Iterator)
- **Consumers** = Produce final result (return value)

---

Print this out and keep it next to you while coding! 📌
