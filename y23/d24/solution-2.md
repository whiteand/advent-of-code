# Solution for Part 2

## Example 

```
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
```

Let's reimagine the world so that our thrown rock will be stand still and all other hailstones will be moving.

$$
\vec{V}_i \text{ - is the velocity of the $i$-th hailstone} \\ 
\vec{P}_i \text{ - is the absolute position of the $i$-th hailstone at t = 0} \\
\vec{u}_i \text{ - is the velocity of the $i$-th hailstone relative to the thrown rock} \\
\vec{p}_i \text{ - is the relative posiition of the $i$-th hailstone relative to the thrown rock} \\ 
\vec{v} \text{ - is the absolute velocity of the thrown rock} \\
\vec{p} \text{ - is the absolute position of the thrown rock at t = 0} \\
t_i \text{ - is the time when the thrown rock hits $i$-th hailstone } \\
$$

We have a system of equations:

$$
\begin{cases}
\vec{u}_i = \vec{V}_i - \vec{v} \\
\vec{p}_i = \vec{P}_i - \vec{p} \\
\vec{p}_i + t_i \vec{u}_i = \vec{0} \\
\end{cases}
$$

From last equation:

$$
\vec{p}_i = -t_i \vec{u}_i \\
$$

Inserting into first two:


$$
\begin{cases}
\vec{u}_i = \vec{V}_i - \vec{v} \\
t_i \vec{u}_i = \vec{p} - \vec{P}_i  \\
\end{cases}
$$

From first:

$$
\vec{u}_i = \vec{V}_i - \vec{v} 
$$

Inserting into second:

$$
t_i (\vec{V}_i - \vec{v}) = \vec{p} - \vec{P}_i  \\

