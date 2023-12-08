function gcd(a: number, b: number): number {
  while (a > 0 && b > 0) {
    if (a > b) {
      a %= b;
    } else {
      b %= a;
    }
  }
  return Math.max(a, b);
}

function solve(
  p1: [number, number],
  p2: [number, number]
): [number, number, number] {
  const a = p1[1];
  const b = p1[0];
  const c = p2[1];
  const d = p2[0];
  return [(a - c) / gcd(d, b), d / gcd(d, b), -b / gcd(d, b)];
}

type TExpr =
  | { type: "number"; value: number }
  | { type: "symbol"; value: string }
  | {
      type: "+";
      elements: TExpr[];
    }
  | {
      type: "*";
      elements: TExpr[];
    }
  | {
      type: "/";
      top: TExpr;
      bottom: TExpr;
    };
type TEquation = {
  type: "equation";
  left: TExpr;
  right: TExpr;
};

function renderEquation(t: TEquation): string {
  if (t.type === "equation") {
    return `${renderExpr(t.left)} = ${renderExpr(t.right)}`;
  }
  throw new Error("Cannot render equation: " + JSON.stringify(t));
}
function renderExpr(expr: TExpr): string {
  if (expr.type === "number") {
    return expr.value.toString();
  }
  if (expr.type === "+") {
    return expr.elements
      .map((x): string => {
        let r = renderExpr(x);
        return r.includes(" ") ? `(${r})` : r;
      })
      .join(" + ");
  }
  if (expr.type === "*") {
    return expr.elements.map(renderExpr).join("*");
  }
  if (expr.type === "symbol") {
    return expr.value;
  }
  if (expr.type === "/") {
    let top = renderExpr(expr.top);
    let bottom = renderExpr(expr.bottom);
    if (top.includes(" ") || top.includes("+")) {
      top = `(${top})`;
    }
    if (
      hasParenthesis(expr.bottom) ||
      bottom.includes("+") ||
      bottom.includes("*")
    ) {
      bottom = `(${bottom})`;
    }
    return `${top}/${bottom}`;
  }
  throw new Error("Cannot render expression: " + JSON.stringify(expr));
}

function expressionToSymbols(e: TExpr): string[] {
  const symbols: string[] = [];

  traverse(e, {
    onSymbol(expr) {
      symbols.push(expr.value);
    },
  });

  return symbols;
}

function equationToSymbols(equation: TEquation) {
  return [
    ...new Set([
      ...expressionToSymbols(equation.left),
      ...expressionToSymbols(equation.right),
    ]),
  ];
}

function hasParenthesis(expr: TExpr): boolean {
  if (expr.type === "*") {
    return expr.elements.some((x) => x.type === "+" || hasParenthesis(x));
  }
  if (expr.type === "/") {
    return hasParenthesis(expr.top) || hasParenthesis(expr.bottom);
  }
  return false;
}

/**
 * If the expression does not have a symbol it remains unchanged
 * If the symbol is present - it will be moved to the top level of the expression
 * If the expression is product - the symbol will be moved to the last position of the product
 **/
function moveSymbolToTop(symbol: string, expr: TExpr): TExpr {
  if (!expressionToSymbols(expr).includes(symbol)) {
    return expr;
  }
  if (expr.type === "symbol") {
    return expr;
  }
  if (expr.type === "+") {
    const flattenElements: TExpr[] = [];
    for (const el of expr.elements) {
      const newEl = moveSymbolToTop(symbol, el);
      if (newEl.type === "+") {
        flattenElements.push(...newEl.elements);
      } else {
        flattenElements.push(newEl);
      }
    }

    flattenElements.sort((a, b) => {
      const aHasSymbol = expressionHasSymbol(symbol, a);
      const bHasSymbol = expressionHasSymbol(symbol, b);
      if (aHasSymbol && !bHasSymbol) return -1;
      if (!aHasSymbol && bHasSymbol) return 1;
      return 0;
    });

    return {
      type: "+",
      elements: flattenElements,
    };
  }
  if (expr.type === "*") {
    if (!hasParenthesis(expr)) {
      return {
        type: "*",
        elements: [
          ...expr.elements.filter(
            (x) => x.type !== "symbol" || x.value !== symbol
          ),
          ...expr.elements.filter(
            (x) => x.type === "symbol" && x.value === symbol
          ),
        ],
      };
    }
  }
  throw new Error(
    `Cannot move symbol "${symbol}" to top of:\n  ${renderExpr(expr)}`
  );
}

function expressionHasSymbol(symbol: string, expr: TExpr): boolean {
  return (
    traverse(expr, {
      onSymbol(expr) {
        if (expr.value === symbol) {
          return { done: true, value: true };
        }
      },
    }) || false
  );
}

function splitBySymbol(symbol: string, expr: TExpr): [TExpr, TExpr] {
  if (
    expr.type === "symbol" ||
    expr.type === "number" ||
    expr.type === "*" ||
    expr.type === "/"
  ) {
    if (expressionHasSymbol(symbol, expr)) {
      return [expr, { type: "+", elements: [] }];
    } else {
      return [{ type: "+", elements: [] }, expr];
    }
  }
  const hasSymbol: TExpr = {
    type: "+",
    elements: [],
  };
  const hasNoSymbol: TExpr = {
    type: "+",
    elements: [],
  };
  for (const el of expr.elements) {
    if (expressionHasSymbol(symbol, el)) {
      appendToElements(hasSymbol, el);
    } else {
      appendToElements(hasNoSymbol, el);
    }
  }

  return [hasSymbol, hasNoSymbol];
}

function appendToElements(sum: TExpr & { type: "+" | "*" }, expr: TExpr): void {
  const num = sum.elements.find((x) => x.type === "number");
  if (expr.type === "*" && sum.type === "*") {
    for (const x of expr.elements) {
      if (x.type === "number" && x.value === 1) {
        continue;
      }
      if (num && num.type === "number" && x.type === "number") {
        num.value *= x.value;
      } else {
        sum.elements.push(x);
      }
    }
    return;
  }
  if (expr.type === "+" && sum.type === "+") {
    for (const x of expr.elements) {
      if (x.type === "number" && x.value === 0) {
        continue;
      }
      if (num && num.type === "number" && x.type === "number") {
        num.value += x.value;
      } else {
        sum.elements.push(x);
      }
    }
    return;
  }
  if (
    num &&
    num.type === "number" &&
    sum.type === "+" &&
    expr.type === "number"
  ) {
    num.value += expr.value;
    return;
  }
  if (
    num &&
    num.type === "number" &&
    sum.type === "*" &&
    expr.type === "number"
  ) {
    num.value *= expr.value;
    return;
  }
  sum.elements.push(expr);
}

function evaluateExpression(expr: TExpr, symbols: Map<string, TExpr>): TExpr {
  if (expr.type === "number") {
    return expr;
  }
  if (expr.type === "symbol") {
    return symbols.get(expr.value) || expr;
  }
  if (expr.type === "+") {
    if (expr.elements.length === 0) {
      return { type: "number", value: 0 };
    }
    const res: TExpr & { type: "+" } = {
      type: "+",
      elements: [],
    };
    for (const el of expr.elements) {
      appendToElements(res, evaluateExpression(el, symbols));
    }
    return simplifyUnsafe(res);
  }
  if (expr.type === "*") {
    if (expr.elements.length === 0) {
      return { type: "number", value: 1 };
    }
    const res: TExpr & { type: "*" } = {
      type: "*",
      elements: [],
    };
    for (const el of expr.elements) {
      appendToElements(res, evaluateExpression(el, symbols));
    }
    return simplifyUnsafe(res);
  }
  if (expr.type === "/") {
    const t = evaluateExpression(expr.top, symbols);
    const b = evaluateExpression(expr.bottom, symbols);
    return simplify({
      type: "/",
      top: t,
      bottom: b,
    });
  }
  throw new Error(
    `Cannot evaluate expression: ${renderExpr(
      expr
    )} with symbols: ${JSON.stringify(
      Object.fromEntries([...symbols.entries()]),
      null,
      2
    )}`
  );
}

type TraverseResult<T> =
  | {
      done: true;
      value: T;
    }
  | {
      done: false;
      skipChildren?: boolean;
    };

function traverse<T>(
  expr: TExpr,
  visitor: {
    onNumber?: (expr: TExpr & { type: "number" }) => TraverseResult<T> | void;
    onSymbol?: (expr: TExpr & { type: "symbol" }) => TraverseResult<T> | void;
    onPlus?: (expr: TExpr & { type: "+" }) => TraverseResult<T> | void;
    onMultiply?: (expr: TExpr & { type: "*" }) => TraverseResult<T> | void;
    onDivide?: (expr: TExpr & { type: "/" }) => TraverseResult<T> | void;
    onExpression?: (expr: TExpr) => TraverseResult<T> | void;
  },
  type: "dfs" | "bfs" = "dfs"
): T | undefined {
  const tasks: TExpr[] = [expr];
  while (tasks.length > 0) {
    const task = type == "dfs" ? tasks.pop() : tasks.shift();
    if (task == null) continue;
    if (visitor.onExpression) {
      const res = visitor.onExpression(task);
      if (res && res.done) {
        return res.value;
      }
    }
    if (task.type === "number") {
      if (visitor.onNumber) {
        const res = visitor.onNumber(task);
        if (res && res.done) {
          return res.value;
        }
      }
      continue;
    }
    if (task.type === "symbol") {
      if (visitor.onSymbol) {
        const res = visitor.onSymbol(task);
        if (res && res.done) {
          return res.value;
        }
      }
      continue;
    }
    if (task.type === "+") {
      if (visitor.onPlus) {
        const res = visitor.onPlus(task);
        if (res && res.done) {
          return res.value;
        }
      }
      tasks.push(...task.elements);
      continue;
    }
    if (task.type === "*") {
      if (visitor.onMultiply) {
        const res = visitor.onMultiply(task);
        if (res && res.done) {
          return res.value;
        }
      }
      tasks.push(...task.elements);
      continue;
    }
    if (task.type === "/") {
      if (visitor.onDivide) {
        const res = visitor.onDivide(task);
        if (res && res.done) {
          return res.value;
        }
      }
      tasks.push(task.top, task.bottom);
      continue;
    }
    throw new Error("Cannot travers expr: " + JSON.stringify(task));
  }
}

let simplifyCtx: { unsafe: boolean; debug: boolean; checking: boolean } | null =
  null;

function simplify(expr: TExpr, unsafe?: boolean, debug?: boolean): TExpr {
  if (!simplifyCtx) {
    simplifyCtx = {
      unsafe: unsafe || false,
      debug: debug || false,
      checking: false,
    };
  }
  if (unsafe && !simplifyCtx.unsafe) {
    simplifyCtx.unsafe = true;
  }
  if (debug && !simplifyCtx.debug) {
    simplifyCtx.debug = true;
  }

  if (simplifyCtx.unsafe && !simplifyCtx.debug) {
    return simplifyUnsafe(expr);
  }

  const res = simplifyUnsafe(expr);

  return res;
}
function simplifyUnsafe(expr: TExpr): TExpr {
  if (expr.type === "number") {
    return expr;
  }

  if (expr.type === "*" && expr.elements.length === 0) {
    return {
      type: "number",
      value: 1,
    };
  }

  if (expr.type === "symbol") return expr;
  if ((expr.type === "*" || expr.type === "+") && expr.elements.length === 1) {
    if (simplifyCtx?.debug)
      console.log("unwraping expression: " + renderExpr(expr));
    return simplify(expr.elements[0]);
  }
  if (expr.type === "+") {
    const res: TExpr & { type: "+" } = {
      type: "+",
      elements: [],
    };
    for (const el of expr.elements) {
      const simplifiedEl = simplify(el);
      if (simplifiedEl.type === "number" && simplifiedEl.value === 0) {
        if (simplifyCtx?.debug) console.log("skipping zero");
        continue;
      }
      appendToElements(res, simplifiedEl);
    }
    if (res.elements.length === 0) {
      if (simplifyCtx?.debug) console.log("Replacing empty sum with 0");
      return {
        type: "number",
        value: 0,
      };
    }
    return res;
  }
  if (expr.type === "*") {
    const numbers = expr.elements.filter((x) => x.type === "number") as Array<
      TExpr & { type: "number" }
    >;

    if (numbers.length > 1) {
      if (simplifyCtx?.debug) console.log(`Collapsing numbers in the product`);
      const newNumbers: TExpr = {
        type: "number",
        value: numbers.reduce((acc, x) => {
          acc *= x.value;
          return acc;
        }, 1),
      };
      return simplify({
        type: "*",
        elements: [
          newNumbers,
          ...expr.elements.filter((x) => x.type !== "number"),
        ],
      });
    }
    const sums = expr.elements.filter((x) => x.type === "+") as Array<
      TExpr & { type: "+" }
    >;

    if (sums.length > 0) {
      const res: TExpr & { type: "+" } = {
        type: "+",
        elements: [],
      };
      const productExprs: TExpr[] = [
        ...expr.elements.filter((x) => x.type !== "+"),
      ];
      if (simplifyCtx?.debug)
        console.log("transitive property: " + renderExpr(expr));
      for (const sum of sums) {
        const simplifiedSumItem = simplify(sum);
        if (
          simplifiedSumItem.type === "number" &&
          simplifiedSumItem.value === 0
        ) {
          if (simplifyCtx?.debug) console.log("skipping 0 sum");
          continue;
        }
        const newProductItem: TExpr = {
          type: "*",
          elements: [...productExprs],
        };
        appendToElements(newProductItem, simplifiedSumItem);

        appendToElements(res, newProductItem);
      }
      return simplify(res);
    }
    const products = expr.elements.filter((x) => x.type === "*") as Array<
      TExpr & { type: "*" }
    >;
    if (products.length > 0) {
      const res: TExpr & { type: "*" } = {
        type: "*",
        elements: [...expr.elements.filter((x) => x.type !== "*")],
      };
      for (const p of products) {
        const simplified = simplify(p);
        appendToElements(res, simplified);
      }
      return simplify(res);
    }
    return expr;
  }
  if (expr.type === "/") {
    const top = simplify(expr.top);
    const bottom = simplify(expr.bottom);
    if (top.type === "number" && bottom.type === "number") {
      const g = gcd(Math.abs(top.value), Math.abs(bottom.value));
      if (g == bottom.value) {
        return { type: "number", value: top.value / g };
      }
      return {
        type: "/",
        top: { type: "number", value: top.value / g },
        bottom: { type: "number", value: bottom.value / g },
      };
    }

    if (top.type === "+") {
      return simplify({
        type: "+",
        elements: top.elements.map((x) =>
          simplify({
            type: "/",
            top: x,
            bottom: bottom,
          })
        ),
      });
    }
    if (expr.top.type === "*") {
      const num = expr.top.elements.find((x) => x.type === "number");
      if (!num) return expr;
      return simplify({
        type: "*",
        elements: [
          simplify({
            type: "/",
            top: num,
            bottom: expr.bottom,
          }),
          ...expr.top.elements.filter((x) => x !== num).map((t) => simplify(t)),
        ],
      });
    }
    throw new Error("Cannot simplify expression: " + renderExpr(expr));
  }
  throw new Error("Cannot simplify expression: " + renderExpr(expr));
}

function resolveSymbolFromEquation(symbol: string, eq: TEquation): TExpr {
  const right = moveSymbolToTop(symbol, eq.right);
  const left = moveSymbolToTop(symbol, eq.left);
  console.log(
    `Moved symbol "${symbol}" to top of both parts:\n    ${renderEquation({
      type: "equation",
      left,
      right,
    })}`
  );
  const [leftSymbolElements, leftNotSymbolElements] = splitBySymbol(
    symbol,
    left
  );
  const [rightSymbolElements, rightNotSymbolElements] = splitBySymbol(
    symbol,
    right
  );
  const newLeft: TExpr = {
    type: "+",
    elements: [
      leftSymbolElements,
      {
        type: "*",
        elements: [{ type: "number", value: -1 }, rightSymbolElements],
      },
    ],
  };
  const newRight: TExpr = {
    type: "+",
    elements: [
      rightNotSymbolElements,
      {
        type: "*",
        elements: [{ type: "number", value: -1 }, leftNotSymbolElements],
      },
    ],
  };
  const simplifiedLeft = simplify(newLeft);
  const simplifiedRight = simplify(newRight);
  console.log(
    `Move all appearance of "${symbol}" to left side, and all other parts to the right side:\n    ${renderEquation(
      {
        type: "equation",
        left: simplifiedLeft,
        right: simplifiedRight,
      }
    )}`
  );
  const coef = extractSymbol(symbol, simplifiedLeft);

  return simplify({
    type: "/",
    top: simplifiedRight,
    bottom: coef,
  });
}

function extractSymbol(symbol: string, expr: TExpr): TExpr {
  if (!expressionHasSymbol(symbol, expr)) {
    throw new Error(
      `There is no symbol "${symbol}" from expression:\n ${renderExpr(expr)}`
    );
  }
  if (expr.type === "*") {
    const withSymbol: TExpr[] = [];
    const withoutSymbol: TExpr[] = [];
    for (const el of expr.elements) {
      if (expressionHasSymbol(symbol, el)) {
        withSymbol.push(el);
      } else {
        withoutSymbol.push(el);
      }
    }
    if (withSymbol.length === 0) {
      throw new Error(
        "Cannot extract symbol from expression without symbol: " +
          renderExpr(expr)
      );
    }
    if (withSymbol.length === 1) {
      return {
        type: "*",
        elements: withoutSymbol,
      };
    }
    throw new Error("Too much symbols in expression: " + JSON.stringify(expr));
  }
  if (expr.type === "+") {
    const withSymbol: TExpr[] = [];
    const withoutSymbol: TExpr[] = [];
    for (const el of expr.elements) {
      if (expressionHasSymbol(symbol, el)) {
        withSymbol.push(el);
      } else {
        withoutSymbol.push(el);
      }
    }
    if (withoutSymbol.length > 0) {
      throw new Error(
        `Cannot extract symbol from "${symbol}" from expression:\n ${renderExpr(
          expr
        )}\n  ${JSON.stringify(expr, null, 2)}`
      );
    }
    if (withSymbol.length === 1) {
      return extractSymbol(symbol, withSymbol[0]);
    }
  }
  throw new Error(
    `Cannot extract symbol from "${symbol}" from expression:\n ${renderExpr(
      expr
    )}\n  ${JSON.stringify(expr, null, 2)}`
  );
}

for (let i = 0; i < 80; i++) {
  console.log();
}

class Equations {
  constructor(private equations: TEquation[]) {}
  display() {
    for (const equation of this.equations) {
      console.log(renderEquation(equation));
    }
  }
  resolveFromEquation(index: number, symbol: string): TExpr {
    return resolveSymbolFromEquation(symbol, this.equations[index]);
  }
  replace(ind: number, symbol: string, value: TExpr) {
    const eq = this.equations[ind];
    const newLeft = evaluateExpression(eq.left, new Map([[symbol, value]]));
    const newRight = evaluateExpression(eq.right, new Map([[symbol, value]]));
    this.equations[ind] = {
      type: "equation",
      left: newLeft,
      right: newRight,
    };
  }
}

simplifyCtx = {
  debug: false,
  unsafe: false,
  checking: false,
};

const params: Array<[number, number]> = [
  [19199, 19198],
  [11309, 11308],
  [17621, 17620],
  [20777, 20776],
  [16043, 16042],
  [15517, 15516],
];

let letters = "efghkmnpqrs";
const solutions: Array<[string, string, number, number, number]> = [];
for (let i = 0; i < params.length - 1; i++) {
  const x = letters[i];
  const y = letters[i + 1];
  const [d, k1, k2] = solve(params[i], params[i + 1]);
  solutions.push([x, y, d, k1, k2]);
}

const equations = new Equations(
  solutions.map(([x, y, d, k1, k2]): TEquation => {
    const left: TExpr = { type: "number", value: d };
    const right: TExpr = {
      type: "+",
      elements: [
        {
          type: "*",
          elements: [
            {
              type: "number",
              value: k1,
            },
            { type: "symbol", value: x },
          ],
        },
        {
          type: "*",
          elements: [
            {
              type: "number",
              value: k2,
            },
            {
              type: "symbol",
              value: y,
            },
          ],
        },
      ],
    };
    return {
      type: "equation",
      left,
      right,
    };
  })
);

equations.display();
const symbols = new Map<string, TExpr>();
const f_expr = equations.resolveFromEquation(0, "f");
equations.replace(1, "f", f_expr);
equations.display();
console.log(`f = ${renderExpr(f_expr)}`);
console.log("done");

// 30  = 43e - 73f; f  = (43e - 30) / 73;
// -24 = 67f - 43g;
// -12 = 79g - 67h;
// 18  = 61h - 79k;
// 2   = 59k - 61m;
