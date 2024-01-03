# Part 2

## Example

```
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
```

### **Idea**

Firstly we need to find the exact line that will come through all other lines.

Line is define through initial position and the direction.

When the line intersects the other line it has distance 0 with that line.

$$
D = \frac{|((P_i - p), U_i, v)|}{U_i\cdot v} = 0
$$

I will assume that the hailstone trajectory will not be perpendicular to the thrown rock.

So that I can just assume that only denominator is equal to $0$.

$$
((P_i - p), U_i, v) = 0
$$

$$
\begin{vmatrix}
P_{ix} - p_x & P_{iy} - p_y & P_{iz} - p_z \\
U_{ix} & U_{iy} & U_{iz}  \\
v_x  & v_y & v_z \\
\end{vmatrix} = 0
$$
$$
(P_{ix} - p_x)U_{iy}v_z + (P_{iy} - p_y)U_{iz}v_x + (P_{iz} - p_z)U_{ix}v_y \\- (P_{iz} - p_z)U_{iy}v_x - (P_{iy} - p_y)U_{ix}v_z - (P_{ix} - p_x)U_{iz}v_y = 0
$$

Let's see the example written in this form:

```
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
```

$$
\begin{cases}
-v_x*(30 - p_z) + v_x*(2*p_y - 26) - v_y*(2*p_x - 38) + v_y*(2*p_z - 60) + v_z*(19 - p_x) - v_z*(2*p_y - 26)\\
v_x*(2*p_y - 38) - v_x*(p_z - 22) - v_y*(2*p_x - 36) + v_y*(p_z - 22) + v_z*(p_x - 18) - v_z*(p_y - 19)\\
v_x*(4*p_y - 100) - v_x*(2*p_z - 68) - v_y*(4*p_x - 80) + v_y*(2*p_z - 68) + v_z*(2*p_x - 40) - v_z*(2*p_y - 50)\\
v_x*(p_y - 31) - v_x*(2*p_z - 56) - v_y*(p_x - 12) + v_y*(p_z - 28) + v_z*(2*p_x - 24) - v_z*(p_y - 31)\\
v_x*(3*p_y - 57) - v_x*(5*p_z - 75) + v_y*(15 - p_z) - v_y*(3*p_x - 60) - v_z*(19 - p_y) + v_z*(5*p_x - 100)
\end{cases}

$$
```python
import sympy as sym

data = [
    ((19, 13, 30), (-2,  1, -2)),
    ((18, 19, 22), (-1, -1, -2)),
    ((20, 25, 34), (-2, -2, -4)),
    ((12, 31, 28), (-1, -2, -1)),
    ((20, 19, 15), ( 1, -5, -3))
]
px, py, pz = sym.symbols('p_x, p_y, p_z')
vx, vy, vz = sym.symbols('v_x, v_y, v_z')

equations = [
    (pix - px)*uiy*vz + (piy - py)*uiz*vx + (piz - pz)*uix*vy - (piz - pz)*uiy*vx - (piy - py)*uix*vz - (pix - px)*uiz*vy
    for (pix, piy, piz), (uix, uiy, uiz) in data
]

```