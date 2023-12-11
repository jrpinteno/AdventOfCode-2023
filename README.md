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

### Day 05

### Day 06
I knew from the beginning I'd solve this one by using math instead of bruteforce, although for the first part I implemented the brute force version.

The idea is quite basic. The basic equation for movement is $d = v * t$. In our case, while the car is accelerating it is not moving. Let's consider it's accumulating 
speed at a constant rate of $1 \text{mm}/\text{ms}$. Therefore,
```math
\begin{align}
d &= v \cdot t \\
0 &= v \cdot \left(t - v\right) - d\\
v^2 - vt + d &= 0
\end{align}
```

Solving the quadratic equation we obtain the minimum and maximum time we can be accelerating which allows our car to **surpass** the given distance.

```math
\begin{cases}
t_1 = \dfrac{t + \sqrt{\strut t^2 - 4\cdot d}}{2} \\[8pt]
t_2 = \dfrac{t - \sqrt{\strut\displaystyle t^2 - 4\cdot d}}{2} \\
\end{cases}
```

All that's left is implementing that into code and profit.

### Day 07
On my first read I thought it was more complicated than it actually was. I had a clear picture I would be using a [`BtreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), so I could use the built-in ordering keys.
I had understood, on that first read, that cards would need to be sorted in terms of strength so they could be compared afterwards, but actually
what mattered was the strength AND the position in the hand for comparison, so no reordering cards on the hand. With that in mind I could have replaced the 
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) but it wouldn't matter.

For the comparison I almost implemented some of the traits, but realised it wasn't even necessary, as it was quite simple. The first option was to
call [`sort_by_key`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.sorted_by_key), but it added some extra complexity with memory handling which
I didn't want to deal with and ended up using `sort_by` and implemented the closure.

As for the second part, I made a _naïve_ solution just checking inside [`determine_play`](https://github.com/jrpinteno/AdventOfCode-2023/blob/main/src/day07.rs#L46)'s `match` 
whether the joker card was present and acted accordingly on the different cases.


### Day 08
Challenge today was to follow a set of instructions (left or right) from a starting node, thus falling onto a different node. Rinse and repeat until we reach node `ZZZ`. 
`HashMap<String, Node>` to the rescue, where the `Node` contains two fields (one per instruction left/right) with a new string.
First part was fairly easy, and I could see the cycles coming to haunt like ghosts for the second part.

And I was right, now we have a map for ghosts. Instead of a single starting node, we now have a few of them (all ending with `A`). Now the condition
to end is that all the ghosts are at the sime time in nodes ending with `Z`. Bruteforcing this would have been insane.
Luckily, since it looks we have cycles, and they should converge at some point (as in, there is a bigger cycle encompassing all of them, we can use LCM.

That said... I was puzzled for more than 30 minutes. Execution was not ending. I stopped it, added good old prints... and it was working, but neverending.
As it turns out, I had a nasty bug. I was using the same parsed [instructions](https://github.com/jrpinteno/AdventOfCode-2023/blob/main/src/day08.rs#L106) for both parts. One after the other.
And since I had the instructions on a [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) (to get and push back the current instruction), that object was mutable.
After adding `.clone()`, everything went smoothly. Had I split the parts, I wouldn't have notice that issue, so it was a learning experience.

Could have avoided using `VecDeque` had I known about [`.iter().cycle()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle).
Instead of _popping_ / _pushing_ instructions, could have iterated using in combination with [`scan`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) until reaching the final condition, thus exiting the cycle.


### Day 09
Not much to say about this one. Quite straightforward (I was expecting it to be harder on a weekend). Predicting the next value on a series was just a matter
of continuosly subtracting the elements of the vector in pairs (learnt about [`.windows()`](https://doc.rust-lang.org/std/slice/struct.Windows.html)). 
Once all are zero, sum the last element of each intermediate vector and that's it.

For the second part, the idea is similar. Keep track of the first value of each intermediate vector and subtract from last to first the accumulated value.


### Day 10
Here comes the grid. I started the day adding format to my grid in order to be able to properly visualize and debug.

In the first part we need to tell the shortest distance to the farthest point in the _cycle_ (and that is a keyword for this part). Initially I was really tempted (and confused perhaps)
to use BFS and find my path there. But then I realized the path is alredy defined, we only needed to count the steps. On each cell of the grid we have a char which defines a double-entry pipe.

First we need to find where do we even begin our cycle (marked by `S` in the input). For that matter I added the [`.find()`](https://github.com/jrpinteno/AdventOfCode-2023/blob/main/src/utils/grid.rs#L77)
method to my `Grid`. Next comes the mapping of the different directions (`North`, `East`, `South`, `West`) to the different `char`s that contain them.
With the starting point, we can iterate over the directions and check which one comes first that we can follow (we have two of them since we are indeed in a cycle). After that,
keep walking, we keep track of which direction we have entered to the new pipe and have a clear exit. 

Once we have reached the starting point again we are done. The distance to the farthest point is half the steps taken.

While trying to figure out the second part, I made some visualization of the pipe traversal from the output of one of the tests

![pipes](https://github.com/jrpinteno/AdventOfCode-2023/assets/1807002/8abf3d05-0cc1-429d-a37c-eb0db581e0f0)

For that visualization I created a copy of the original grid, but empty. And that will come in handy for the second part (I didn't know at the moment).

I had some vague recollection that it was possible to compute the area of any polygon on a grid and get the amount of points within that area (not on the border). But didn't recall how it worked, I googled
a bit and found about [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem). So this is exactly what I needed, but was not
sure how to implement. I decided to take a different approach, the [Ray Casting algorithm](https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm). Cast the rays horizontally on the grid,
and flip a boolean whenever we have a vertical wall.



