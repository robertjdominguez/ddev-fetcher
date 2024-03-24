---
title: "WILT: Week of 2/23/2024"
hook: "WILT — A dive and recap into what I learned this week. This is part of a weekly series that is summarized via daily reflections and compiled by ChatGPT 🚀"
slug: wilt-week-of-2024-02-23
created_at: 2024-02-23T19:11:45.23547+00:00
image: https://media.graphassets.com/mZuL9rwaQHOtlFux5Kb3
---

# WILT: Week of 2/23/2024

# WILT March [Enter Date]

## Rust: The Adventure Continues

My journey with Rust is ongoing, and every day uncovers something new—kind of like finding another puzzle piece when you thought the board was nearly complete. The key realizations this week revolved around Rust's approach to data structures and error handling.

### Type Juggling with Rust

Data juggling (or should I say “type juggling”) in Rust is an interesting challenge compared to the hand-holding of JavaScript or TypeScript. Rust honors the old adage: If you want something done right, do it yourself. Here's where I found myself converting types like a street magician shuffling cards.

Say you're starting with a `Vec<String>`—a vector of strings—and you want each string uppercase. It's an iterator conversion game:

```rust
let words = vec![rustacean, ferris, crab];
let uppercase_words: Vec<String> = words.into_iter().map(|word| word.to_uppercase()).collect();
// Now, `uppercase_words` contains [RUSTACEAN, FERRIS, CRAB]
```

It's a dance of `iter()`, `map()`, `collect()`, and sometimes `into_iter()` to satisfy the stern but fair borrow checker. Picking up these patterns is essential, and thankfully, my past flirtations with TypeScript give me a déjà vu advantage.

### Reading Text Files: Simple yet Efficient

Here's a practical Rust snippet where I was working on reading a text file and looping over it:

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new(my_precious_log.txt);
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();
    
    for line in lines {
        if let Ok(ip) = line {
            println!({}, ip);
        }
    }
    Ok(())
}
```

This code elegantly opens a file and prints each line, showcasing the beauty of Rust's error handling with the `Result` type. Just don't forget to handle these `Ok` and `Err` variants, or the compiler will remind you with a wagging finger.

## The Past Makes a Comeback

Reflecting on a past project, I built a tool in Rust for intelligently redirecting 404s. It was functional but rough around the edges—think Frankenstein's monster before a makeover. That project, however, laid crucial groundwork for my ongoing Rust enlightenment.

## The Philosophy Behind Coding

### Shifting Left is Shifting Wise

There's this business mantra, shift left, that I can't shake off—and honestly, I wouldn't want to. It applies as much to life as it does to software development: preempt the problems. The sooner we address potential issues, be it in code or strategy meetings, the less dumpster fire management we have to do down the line. It's good for my sanity and the company's bottom line.

### The 12 Week Year: A Peek into Personal Growth

I often find lessons for my professional life in unexpected places, like the new book I'm burrowing into: The 12 Week Year. The concept of periodization, familiar to me from my coaching days, is resurfacing as a guide for personal and professional rhythm. Here’s a distilled essence of the empowering message I'm imbibing:

Own your actions. Nothing is mandated from on high; every task, every attempt, every single line of code written, they are choices. This stoic mindset isn't just sobering; it's liberating. When we choose our actions, we accept that we're the masters of our domains—even if that domain currently is mostly console logs and compiler errors.

## Closing Thoughts

As I dole out these morsels from my week, wrapped in a veneer of self-deprecation and sprinkled with cynicism, I hope it's evident that behind it all, there's a genuine passion for learning and sharing my journey—missteps included. Whether it's iterating over a file line by line or plotting my growth in 12-week sprints, each day serves as a reminder: we're all lifelong students, sometimes of code, sometimes of life, always of both.
