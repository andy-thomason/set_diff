use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the dictionary to a string, enabling zero copy in dict and fewer syscalls.
    let dict = std::fs::read_to_string("dictionary.txt")?;

    // Collect the distinct words into a set shadowing the variable `dict`.
    let dict : HashSet<&str> = dict
        .split_ascii_whitespace()
        .collect();

    // Read the input to a string, for fewer syscalls.
    // Note that for very large files, a memory map may be better.
    let input = std::fs::read_to_string("input.txt")?;

    // Filter the words not in the dictionary and print them.
    input
        .split_ascii_whitespace()
        .filter(|word| !dict.contains(word))
        .for_each(|word| println!("{}", word));

    Ok(())
}
