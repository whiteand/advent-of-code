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
-2*p_x*sin(\phi)*sin(\theta) - p_x*cos(\theta) + 2*p_y*sin(\theta)*cos(\phi) - 2*p_y*cos(\theta) + 2*p_z*sin(\phi)*sin(\theta) + p_z*sin(\theta)*cos(\phi) - 22*sin(\phi)*sin(\theta) - 56*sin(\theta)*cos(\phi) + 45*cos(\theta) = 0 \\
-2*p_x*sin(\phi)*sin(\theta) + p_x*cos(\theta) + 2*p_y*sin(\theta)*cos(\phi) - p_y*cos(\theta) + p_z*sin(\phi)*sin(\theta) - p_z*sin(\theta)*cos(\phi) + 14*sin(\phi)*sin(\theta) - 16*sin(\theta)*cos(\phi) + cos(\theta) = 0 \\
-4*p_x*sin(\phi)*sin(\theta) + 2*p_x*cos(\theta) + 4*p_y*sin(\theta)*cos(\phi) - 2*p_y*cos(\theta) + 2*p_z*sin(\phi)*sin(\theta) - 2*p_z*sin(\theta)*cos(\phi) + 12*sin(\phi)*sin(\theta) - 32*sin(\theta)*cos(\phi) + 10*cos(\theta) = 0 \\
-p_x*sin(\phi)*sin(\theta) + 2*p_x*cos(\theta) + p_y*sin(\theta)*cos(\phi) - p_y*cos(\theta) + p_z*sin(\phi)*sin(\theta) - 2*p_z*sin(\theta)*cos(\phi) - 16*sin(\phi)*sin(\theta) + 25*sin(\theta)*cos(\phi) + 7*cos(\theta) = 0 \\
-3*p_x*sin(\phi)*sin(\theta) + 5*p_x*cos(\theta) + 3*p_y*sin(\theta)*cos(\phi) + p_y*cos(\theta) - p_z*sin(\phi)*sin(\theta) - 5*p_z*sin(\theta)*cos(\phi) + 75*sin(\phi)*sin(\theta) + 18*sin(\theta)*cos(\phi) - 119*cos(\theta) = 0 \\
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

def print_equations(data):
    px, py, pz = sym.symbols('p_x, p_y, p_z')
    phi, theta = sym.symbols('\\phi, \\theta')
    vx = sym.cos(phi)*sym.sin(theta)
    vy = sym.sin(phi)*sym.sin(theta)
    vz = sym.cos(theta)
    equations = []
    print("\\begin{cases}")
    for (pix, piy, piz), (uix, uiy, uiz) in data:
        eq = sym.expand((pix - px)*uiy*vz + (piy - py)*uiz*vx + (piz - pz)*uix*vy - (piz - pz)*uiy*vx - (piy - py)*uix*vz - (pix - px)*uiz*vy)
        print(
            eq
            , '= 0 \\\\'
        )
        equations.append(eq)
    print("\\end{cases}")
    return (equations, [px, py, pz, phi, theta])

eqs, variables = print_equations(data)
print(sym.nonlinsolve(eqs, variables))
```
