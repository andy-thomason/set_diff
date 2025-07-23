# Code critiqe no. 154

## A comparison of C++ and Rust iterators.

Both C++ and Rust have gone a long way to make iterators the centre
of the language.

Iterators can be expressive and solve problems with fewer lines of code
than with conventional control flow loops.

Both C++ and Rust use the reductive power SSA optimisers to remove function
calls, load store pairs and any obvious repetition resulting in generated
code being concise and efficient. In both languages iterators reduce
to single loops in most cases or to constants in the best case.

In C++, we have input and output iterators equivalent to immutable and
mutable iterators in Rust.

## Input the dictionary

The code:

```C++
  ifstream dictionary("dictionary.txt");
  set<string> dict;
  copy(istream_iterator<string>(dictionary),
       istream_iterator<string>(),
       inserter(dict, dict.begin()));
```

Uses the `copy` function in the standard library to
a mutable iterator for the set.

The equivalent in Rust is:

```Rust
    // Read the dictionary to a string, enabling zero copy in dict and fewer syscalls.
    let dict = std::fs::read_to_string("dictionary.txt")?;

    // Collect the distinct words into a set shadowing the variable `dict`.
    let dict : BTreeSet<&str> = dict
        .split_ascii_whitespace()
        .collect();
```

The Rust equivalent to `set` is `BTreeSet` which is a balanced binary tree
set or ordered set.

We could have used a `BufReader` and the `split` method to be more equivalent to
the C++ code, but we would lose the zero copy property and `split` does not support
a general predicate.

## Input the text.

For the input text, the C++ code is:

```C++
  multiset<string> words;
  copy(istream_iterator<string>(cin),
       istream_iterator<string>(),
       inserter(words, words.begin()));
```

This builds a set that allows duplicate entries.
There is no direct equivalent in standard Rust, although we could use
`BTreeSet<(&str, usize)>` where the second parameter is an incrementing
nonce. Ignoring the nonce on reading achieves the same result.

However, the use of `multiset` in this example is likely the
problem with the code as when we replace it with `set` the
problem goes away. This is because `set_difference` which works
on sorted data only counts a single repeated item once.

So Rust equivalent is:

```Rust
    // Read the input to a string, for fewer syscalls.
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    // Collect the distinct words into a set shadowing the variable `dict`.
    let input : BTreeSet<&str> = input
        .split_ascii_whitespace()
        .collect();
```

The final difference in C++ is:

```C++
  set_difference(
      words.begin(), words.end(),
      dict.begin(), dict.end(),
      ostream_iterator<string>(cout, "\n"));
```

With the rust equivalent:

```Rust
    // Output the set difference.
    input
        .difference(&dict)
        .for_each(|word| println!("{}", word));
```

## Performance note:

In both cases using a vector and `sort` may be more efficient
than using a `set` or `BTreeSet`. This is because the cost of performing
insertion and tree rotations in a binary tree is nonzero.

Note `std::string_view` is an equivalent to Rust's `&str` and may
be a better choice than `string`.

Amy Thomason
Atomic Increment Ltd - Rust and C++ training, porting and wizzardry.
