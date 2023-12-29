# Solution for Part 2

## Example 

```
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
```

$$
\vec{v} \text{ - is the velocity of the thrown rock}\\
\vec{p} \text{ - is the position of the thrown rock}\\
t_{i} \text{ - is the time when the thrown rock hits $i$-th hailstone }\\
$$

Therefore we have such system of equations:

$$
\begin{cases}
\vec{p} + t_{i}\vec{v} = \vec{p_{i}} + t_{i}\vec{v_{i}}\\
\end{cases}
$$

Combininig similar terms we get:

$$
\begin{cases}
t_{i}(\vec{v} - \vec{v_{i}}) = \vec{p_{i}} - \vec{p}\\
\end{cases}
$$

Lines to consider: 

| # Hails | Unknowns                          | # Eqs |
| ------- | --------------------------------- | ----- |
| 1       | $t_{i}$, $\vec{v}$, $\vec{p}$ = 7 | 3     |
| 2       | $t_{i}$, $\vec{v}$, $\vec{p}$ = 8 | 6     |
| 3       | $t_{i}$, $\vec{v}$, $\vec{p}$ = 9 | 9     |

We need 3 hailstones which are flying in non-colinear fashion.

So we will have 9 equations with 9 unknowns.

$$
\begin{cases}
t_{0} * v_{x} - t_{0} * (v_{0})_{x} = (p_{0})_{x} - p_{x}\\
t_{0} * v_{y} - t_{0} * (v_{0})_{y} = (p_{0})_{y} - p_{y}\\
t_{0} * v_{z} - t_{0} * (v_{0})_{z} = (p_{0})_{z} - p_{z}\\
t_{1} * v_{x} - t_{1} * (v_{1})_{x} = (p_{1})_{x} - p_{x}\\
t_{1} * v_{y} - t_{1} * (v_{1})_{y} = (p_{1})_{y} - p_{y}\\
t_{1} * v_{z} - t_{1} * (v_{1})_{z} = (p_{1})_{z} - p_{z}\\
t_{2} * v_{x} - t_{2} * (v_{2})_{x} = (p_{2})_{x} - p_{x}\\
t_{2} * v_{y} - t_{2} * (v_{2})_{y} = (p_{2})_{y} - p_{y}\\
t_{2} * v_{z} - t_{2} * (v_{2})_{z} = (p_{2})_{z} - p_{z}\\
\end{cases}
$$