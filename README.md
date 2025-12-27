# Advent of Code 2025 🎄in Rust 🦀

During Christmas 2025, I decided to learn Rust, and after getting to [chapter 9](https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html), I wanted to practice a little bit.

Since it was Christmas and I hadn't done [Advent of Code 2025](https://adventofcode.com/2025) yet at that point in time, it was the perfect fit.
The solutions do not require any complicated tooling or user input, so they are great practice.

I have tried to keep the history of how I learned visible by not touching my solutions at all after I have finished them.

## Solutions

For most of the solutions, I finished quickly, but for the problems on days 10 and 11, I ended up taking hints.

### Day 9

The solution for day 9 is cheating a little bit, I knew I need to calculate whether the large polygon contains the rectangle and wanted to implement [PiP](https://en.wikipedia.org/wiki/Point_in_polygon) raycasting and do the validations manualy, but I have already done that last year for similar problem, so instead I tried to explore a little bit what are the available crates for working with polygons in Rust... and lo and behold, there is a crate called `geo` which does exactly what I needed. So I have used it to implement the solution.

### day 10

For Day 10, I have done the first part pretty easily, but then I got stuck on the second part. I have converted the problem into equations and then implemented Gaussian elimination to reduce the linear equation system, but I was not able to get the correct answer; no matter what I tried, the fractions were killing me. After spending a few hours trying to debug my implementation, I have decided to take a hint and look at the subreddit, and found [this](https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)...

### Day 11

For Day 11, the first part was easy. For part 2, I have implemented almost exactly the same solution as the final one in this repo, but no matter what I did, the final result for the full input just would not be correct. I went to a subreddit for a different solution just to be able to compare what the actual number I was looking for was.

In the end, my mistake was just one missing addition. I have converted the graph to a dot file and visualized it in [Graphviz](https://magjac.com/graphviz-visual-editor/) which led me to believe that there are no paths between DAC and FFT, only from FFT to DAC. But clearly that was wrong, because adding this solved the problem. Not sure what I overlooked.

### Day 12

I have spent 2 days learning about bin packing, 2D packing algorithms, and even trying genetic programming, but there was just absolutely no way that my solutions would ever finish in time, so I have tried to optimize how many definitions I would have to solve by first checking if the gifts fit into the area without overlaps in their "bounding boxes". And guess what, the example was a trick, and there was no need for any packing... then I checked Reddit, and everyone solved it like that...

## Why is there just one commit?

Initially, this repo contained the full testing inputs and readmes for all the problems, but while solving Day 12, I learned about [the fact](https://adventofcode.com/2025/about#faq_copying) that the author does not wish for people to share the problem descriptions and inputs, so I have decided to scrub the whole git history.

## Resources used

- [Advent of Code 2025](https://adventofcode.com/2025)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Solution of Day 10](https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)
- [Graphviz](https://magjac.com/graphviz-visual-editor/)
- [Geo crate](https://docs.rs/geo/latest/geo/)

## License

See the [LICENSE](LICENSE) file for details.
