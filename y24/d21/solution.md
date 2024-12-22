\[LANGUAGE: Rust\]    

Both parts are solved by the same algorithm, but with different parameter - an amount of intermediate robots (`robots_amount`).

Main ideas:
- If you want to go from a key `x` to key `y` you will go there
  - using a trajectory straight line `x..y` (if `x` and `y` are placed on the same row or column)
  - using a trajectory `x`..`p` + `p`..`y`. Where `p` is one of angles of rectangle with opposite corners placed at `x` and `y`.

    x..p1
    .  .
    .  .
    p2.y

- After you get some trajectory, for example `v<<A>>^A<A>AvA<^AA>A<vAAA>^A`. The resulting min_steps for these instructions will be the sum of the min_steps of the "sub-trajectories" ending with `A`:

    min_steps("v<<A>>^A<A>AvA<^AA>A<vAAA>^A") =  
        min_steps("v<<A") +  
        min_steps(">>^A") +  
        min_steps("<A") +  
        min_steps(">A") +  
        min_steps("vA") +  
        min_steps("<^A") +  
        min_steps("A") +  
        min_steps(">A") +  
        min_steps("<vA") +  
        min_steps("A") +  
        min_steps("A") +  
        min_steps(">^A")  

If you want to see a Rust solution with:
- constant type parameters
- itertools trait
- usage of `Either` from itertools
- usage of custom iterators
- usage of FxHashMap for caching

You can see here: https://github.com/whiteand/advent-of-code/blob/7ff107f8ca3b9b9f0e322226fdaa60d8b40ab170/y24/d21/src/lib.rs

Performance:

    
    y24d21    fastest       │ slowest       │ median        │ mean          │ samples │ iters
    ├─ part1  8.791 µs      │ 22.45 µs      │ 9.228 µs      │ 9.401 µs      │ 100     │ 100
    ╰─ part2  33.16 µs      │ 87.7 µs       │ 33.66 µs      │ 35.6 µs       │ 100     │ 100