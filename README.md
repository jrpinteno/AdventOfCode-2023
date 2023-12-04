# AdventOfCode-2023

It's [that time](https://adventofcode.com/2023) of the year. This time around I'm gonna use this AoC to learn [Rust](https://www.rust-lang.org/). 
As opposed to [last year](https://github.com/jrpinteno/AdventOfCode-2022) with C++, I have no previous knowledge of the language.
So the first days will most likely be a bit challenging, not because of the tasks, but to get used to the nuances of Rust.

Note: I _might_ also do it in C++, in fact that might be my first solve.

## Log
### Day 01
First contact with Rust. Coming from Swift some of the syntax feels familiar. I used this first day to get my bearings
and check what does the language offer, and its constraints. First part was quite straightforward. Some minor struggle 
between `&str` and `String`. I guess it will become more familiar as days pass. I've decided I'll be
using the _test_ functionality that comes with Rust from now onwards.

For the second part I was bit surprised in the difficulty, I'm not saying it was hard, but the jump felt bit higher than
previous year. I took different approach to play around.

### Day 02
On [day 01](###-day-01) I used `read_to_string` to parse the input into a `Vec<String>`. Today I decided to use `std::io::BufRead`
and iterators to generate that vector. Parsing was somehow complicated. I don't know why, but for some reason I began trying
to get the whole thing done only using the functional aspect of Rust. It was good training, but I guess I should have tried
a different approach.

### Day 03
Today I decided to move already some utils I most likely will be using alongside the _AoC trip_. I got to play with the
Rust's counterpart to Swift's [extensions](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/extensions/) and [protocols](https://docs.swift.org/swift-book/documentation/the-swift-programming-language/protocols/): [traits](https://doc.rust-lang.org/book/ch10-02-traits.html). Something simple to start with, I just wanted a way to generate a `Vec<String>` from
a multiline `str` (I'm still struggling on the [difference](https://rustjobs.dev/blog/difference-between-string-and-str-in-rust) besides mutability).

The process today was almost [TDD](https://en.wikipedia.org/wiki/Test-driven_development). I began with the valid symbols (anything that was not a '.' and 
that was considered `is_ascii_punctuation` by Rust; granted, that check could have been `!symbol.is_digit && !symbol.is_ascii_alphabetic`, but I assumed 
they wouldn't have included other symbols. Then extracted the position where a symbol would be on the vector (note: I will implement a rough Grid struct quite 
soon, it would have made my life easier for this challenge).

Extracting the numbers was the most difficult part. I had gone with one idea in the morning, but scratched that. When I got back to the task I implemented the current version.
For the adjacency of part numbers/symbols I played with ranges and `checked_sub` since the first position would be problematic.
Adding all the part numbers adjacent to symbols was quite easy in the end. One external iterator on _part numbers_ filtered by an internal one over position of the symbols 
and checking the adjacency.

Second part was quite straightforward once again. I refactored a bit some of the methods to accept an `is_gear` bool and for the sum of _gear ratios_ the iterators are reversed:
External iteration over points filtering when the symbol would had exactly 2 adjacent parts .

### Day 04
The time I ~wasted~ invested in parsing the games for [day 02](###-day-02) has payed off and it's been fairly easy to get the different moving parts for today's challenge.
Part 1 was a walk in the park. Upon reading it, what came to mind was using _set intersection_, but I realized it was enough by just filtering out numbers in one side from the other.

For the second part, I created a `card_copies` vector initially holding 1 for all. Then just modified the `get_card_score` to return a tuple `(score, matched_count)`. `matched_count` is
used to add same amount of copies we already have of a given card, to the next `matched_count` cards in the copies vector.

In terms of difficulty, I'd say this would have been good for [day 02](###-day-02). That one felt more complicated.

_Addendum_: Quickly implemented a simple `Grid` struct helper for the future. I feel it coming into use very soon, I'll add at that point the parsing into the grid implementation. Might even revisit [day 02](###-day-02)
at some point and add the helper functions needed for that one.
