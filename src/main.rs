use std::{collections::BTreeSet, error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the dictionary to a string, enabling zero copy in dict and fewer syscalls.
    let dict = std::fs::read_to_string("dictionary.txt")?;

    // Collect the distinct words into a set shadowing the variable `dict`.
    let dict : BTreeSet<&str> = dict
        .split_ascii_whitespace()
        .collect();

    // Read the input to a string, for fewer syscalls.
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    // Collect the distinct words into a set shadowing the variable `dict`.
    let input : BTreeSet<&str> = input
        .split_ascii_whitespace()
        .collect();

    // Output the set difference.
    input
        .difference(&dict)
        .for_each(|word| println!("{}", word));

    Ok(())
}
