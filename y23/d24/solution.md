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

To rmove unnecessary indices lets introduce such values:
$$
\vec{v}_{0} = \vec{U}\\
\vec{v}_{1} = \vec{W}\\
\vec{v}_{2} = \vec{S}\\
\vec{v}_{3} = \vec{J}\\
\vec{v}_{4} = \vec{Y}\\
\vec{p}_{0} = \vec{A}\\
\vec{p}_{1} = \vec{B}\\
\vec{p}_{2} = \vec{C}\\
\vec{p}_{3} = \vec{D}\\
\vec{p}_{4} = \vec{E}\\
$$

$$
\begin{cases}
t_{0} * v_{x} - t_{0} * U_{x} = A_{x} - p_{x}\\
t_{0} * v_{y} - t_{0} * U_{y} = A_{y} - p_{y}\\
t_{0} * v_{z} - t_{0} * U_{z} = A_{z} - p_{z}\\
t_{1} * v_{x} - t_{1} * W_{x} = B_{x} - p_{x}\\
t_{1} * v_{y} - t_{1} * W_{y} = B_{y} - p_{y}\\
t_{1} * v_{z} - t_{1} * W_{z} = B_{z} - p_{z}\\
t_{2} * v_{x} - t_{2} * S_{x} = C_{x} - p_{x}\\
t_{2} * v_{y} - t_{2} * S_{y} = C_{y} - p_{y}\\
t_{2} * v_{z} - t_{2} * S_{z} = C_{z} - p_{z}\\
\end{cases}
$$
$$
\begin{cases}
t_{0} * v_{x} - t_{0} * U_{x} + p_{x} = A_{x}\\
t_{0} * v_{y} - t_{0} * U_{y} + p_{y} = A_{y}\\
t_{0} * v_{z} - t_{0} * U_{z} + p_{z} = A_{z}\\
t_{1} * v_{x} - t_{1} * W_{x} + p_{x} = B_{x}\\
t_{1} * v_{y} - t_{1} * W_{y} + p_{y} = B_{y}\\
t_{1} * v_{z} - t_{1} * W_{z} + p_{z} = B_{z}\\
t_{2} * v_{x} - t_{2} * S_{x} + p_{x} = C_{x}\\
t_{2} * v_{y} - t_{2} * S_{y} + p_{y} = C_{y}\\
t_{2} * v_{z} - t_{2} * S_{z} + p_{z} = C_{z}\\
\end{cases}
$$

From first three equations:

$$
\begin{cases}
p_{x} = A_{x} - t_{0} * v_{x} + t_{0} * U_{x}\\
p_{y} = A_{y} - t_{0} * v_{y} + t_{0} * U_{y}\\
p_{z} = A_{z} - t_{0} * v_{z} + t_{0} * U_{z}\\
\end{cases}
$$


Substituting into the rest of the equations:

$$
\begin{cases}
t_{1} * v_{x} - t_{1} * W_{x} + A_{x} - t_{0} * v_{x} + t_{0} * U_{x} = B_{x}\\
t_{1} * v_{y} - t_{1} * W_{y} + A_{y} - t_{0} * v_{y} + t_{0} * U_{y} = B_{y}\\
t_{1} * v_{z} - t_{1} * W_{z} + A_{z} - t_{0} * v_{z} + t_{0} * U_{z} = B_{z}\\
t_{2} * v_{x} - t_{2} * S_{x} + A_{x} - t_{0} * v_{x} + t_{0} * U_{x} = C_{x}\\
t_{2} * v_{y} - t_{2} * S_{y} + A_{y} - t_{0} * v_{y} + t_{0} * U_{y} = C_{y}\\
t_{2} * v_{z} - t_{2} * S_{z} + A_{z} - t_{0} * v_{z} + t_{0} * U_{z} = C_{z}\\
\end{cases}
$$

From First three equations:

$$
\begin{cases}
v_{x}  = \frac{B_{x} + t_{1} * W_{x} - t_{0} * U_{x} - A_{x}}{t_{1} - t_{0}}\\
v_{y}  = \frac{B_{y} + t_{1} * W_{y} - t_{0} * U_{y} - A_{y}}{t_{1} - t_{0}}\\
v_{z}  = \frac{B_{z} + t_{1} * W_{z} - t_{0} * U_{z} - A_{z}}{t_{1} - t_{0}}\\
\end{cases}
$$

Substitution into the rest of the equations:

$$
\begin{cases}
t_{2} * \frac{B_{x} + t_{1} * W_{x} - t_{0} * U_{x} - A_{x}}{t_{1} - t_{0}} - t_{2} * S_{x} + A_{x} - t_{0} * \frac{B_{x} + t_{1} * W_{x} - t_{0} * U_{x} - A_{x}}{t_{1} - t_{0}} + t_{0} * U_{x} = C_{x}\\
t_{2} * \frac{B_{y} + t_{1} * W_{y} - t_{0} * U_{y} - A_{y}}{t_{1} - t_{0}} - t_{2} * S_{y} + A_{y} - t_{0} * \frac{B_{y} + t_{1} * W_{y} - t_{0} * U_{y} - A_{y}}{t_{1} - t_{0}} + t_{0} * U_{y} = C_{y}\\
t_{2} * \frac{B_{z} + t_{1} * W_{z} - t_{0} * U_{z} - A_{z}}{t_{1} - t_{0}} - t_{2} * S_{z} + A_{z} - t_{0} * \frac{B_{z} + t_{1} * W_{z} - t_{0} * U_{z} - A_{z}}{t_{1} - t_{0}} + t_{0} * U_{z} = C_{z}\\
\end{cases}
$$

$$
\begin{cases}
(t_{2} - t_{0}) * \frac{B_{x} + t_{1} * W_{x} - t_{0} * U_{x} - A_{x}}{t_{1} - t_{0}} - t_{2} * S_{x} + t_{0} * U_{x} = C_{x} - A_{x}\\
(t_{2} - t_{0}) * \frac{B_{y} + t_{1} * W_{y} - t_{0} * U_{y} - A_{y}}{t_{1} - t_{0}} - t_{2} * S_{y} + t_{0} * U_{y} = C_{y} - A_{y}\\
(t_{2} - t_{0}) * \frac{B_{z} + t_{1} * W_{z} - t_{0} * U_{z} - A_{z}}{t_{1} - t_{0}} - t_{2} * S_{z} + t_{0} * U_{z} = C_{z} - A_{z}\\
\end{cases}
$$

Let's itoduce $q_{21} = \frac{t_2 - t_0}{t_1 - t_0}$

$$
\begin{cases}
q_{21} (B_{x} + t_{1} * W_{x} - t_{0} * U_{x} - A_{x}) - t_{2} * S_{x} + t_{0} * U_{x} = C_{x} - A_{x}\\
q_{21} (B_{y} + t_{1} * W_{y} - t_{0} * U_{y} - A_{y}) - t_{2} * S_{y} + t_{0} * U_{y} = C_{y} - A_{y}\\
q_{21} (B_{z} + t_{1} * W_{z} - t_{0} * U_{z} - A_{z}) - t_{2} * S_{z} + t_{0} * U_{z} = C_{z} - A_{z}\\
\end{cases}
$$

Let's simplify using vectors

$$
\begin{cases}
q_{21} (AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}) - t_{2} * S_{x} + t_{0} * U_{x} = AC_{x}\\
q_{21} (AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}) - t_{2} * S_{y} + t_{0} * U_{y} = AC_{y}\\
q_{21} (AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}) - t_{2} * S_{z} + t_{0} * U_{z} = AC_{z}\\
\end{cases}
$$

Since I've added $q_{21}$ I now have 4 unknowns and 3 equations. Likely I have additional points to consider.

$$
\begin{cases}
q_{21} (AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}) - t_{2} * S_{x} + t_{0} * U_{x} = AC_{x}\\
q_{21} (AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}) - t_{2} * S_{y} + t_{0} * U_{y} = AC_{y}\\
q_{21} (AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}) - t_{2} * S_{z} + t_{0} * U_{z} = AC_{z}\\
q_{31} (AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}) - t_{3} * J_{x} + t_{0} * U_{x} = AD_{x}\\
q_{31} (AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}) - t_{3} * J_{y} + t_{0} * U_{y} = AD_{y}\\
q_{31} (AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}) - t_{3} * J_{z} + t_{0} * U_{z} = AD_{z}\\
q_{41} (AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}) - t_{4} * Y_{x} + t_{0} * U_{x} = AE_{x}\\
q_{41} (AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}) - t_{4} * Y_{y} + t_{0} * U_{y} = AE_{y}\\
q_{41} (AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}) - t_{4} * Y_{z} + t_{0} * U_{z} = AE_{z}\\
\end{cases}
$$

Now I have 9 equations and unknowns:
$$
q_{21}, q_{31}, q_{41}, t_0, t_1, t_2, t_3, t_4
$$

From first three equations:

$$
\begin{cases}
q_{21} = \frac{AC_{x} + t_{2} * S_{x} - t_{0} * U_{x}}{AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}}\\
q_{21} = \frac{AC_{y} + t_{2} * S_{y} - t_{0} * U_{y}}{AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}}\\
q_{21} = \frac{AC_{z} + t_{2} * S_{z} - t_{0} * U_{z}}{AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}}\\
\end{cases}
$$

Remove second and third from first

$$
\begin{cases}
\frac{AC_{x} + t_{2} * S_{x} - t_{0} * U_{x}}{AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}} -  \frac{AC_{y} + t_{2} * S_{y} - t_{0} * U_{y}}{AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}} = 0 \\ 
\frac{AC_{x} + t_{2} * S_{x} - t_{0} * U_{x}}{AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}} - \frac{AC_{z} + t_{2} * S_{z} - t_{0} * U_{z}}{AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}} = 0 \\ 
\end{cases}
$$
$$
\begin{cases}
(AC_{x} + t_{2} * S_{x} - t_{0} * U_{x})(AB_{y} + t_{1} * W_{y} - t_{0} * U_{y}) -  (AC_{y} + t_{2} * S_{y} - t_{0} * U_{y})(AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}) = 0 \\ 
\frac{AC_{x} + t_{2} * S_{x} - t_{0} * U_{x}}{AB_{x} + t_{1} * W_{x} - t_{0} * U_{x}} - \frac{AC_{z} + t_{2} * S_{z} - t_{0} * U_{z}}{AB_{z} + t_{1} * W_{z} - t_{0} * U_{z}} = 0 \\ 
\end{cases}
$$



