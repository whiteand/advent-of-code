import { FlipFlop, IModule, IModuleVisitor } from "./modules";

class Visitor implements IModuleVisitor {
  private target: string;
  private totalLow: number = 0;
  private totalHigh: number = 0;
  constructor(private _source: string, private signals: TSignal[]) {
    this.target = "broadcaster";
  }
  getTotalLow(): number {
    return this.totalLow;
  }
  getTotalHigh(): number {
    return this.totalHigh;
  }
  low(name: string): void {
    this.totalLow += 1;
    this.signals.push({ type: "low", source: this.target, output: name });
  }
  high(name: string): void {
    this.totalHigh += 1;
    this.signals.push({ type: "high", source: this.target, output: name });
  }
  setTarget(t: string) {
    this.target = t;
  }
  setSource(t: string) {
    this._source = t;
  }
  source() {
    return this._source;
  }
}

type TSignal = {
  type: "low" | "high";
  source: string;
  output: string;
};

const ORDER = [
  "bd",
  "bk",
  "fl",
  "fr",
  "fs",
  "gb",
  "jg",
  "ks",
  "lg",
  "lh",
  "mf",
  "mp",
  "nv",
  "pz",
  "qj",
  "rp",
  "tp",
  "vk",
  "vq",
  "xk",
  "jk",
  "qv",
  "tk",
  "xl",
  "cx",
  "gk",
  "lz",
  "pc",
  "jp",
  "lj",
  "ml",
  "pg",
  "bh",
  "qd",
  "rn",
  "vs",
  "sk",
  "th",
  "xg",
  "hv",
  "mj",
  "qn",
  "zq",
  "kb",
  "ct",
  "ft",
  "hr",
  "qm",
];
const ORDER_ALL = [
  "bd",
  "bh",
  "bk",
  "broadcaster",
  "cl",
  "cm",
  "ct",
  "cx",
  "dt",
  "fl",
  "fr",
  "fs",
  "ft",
  "gb",
  "gk",
  "hr",
  "hv",
  "jg",
  "jk",
  "jp",
  "js",
  "kb",
  "kd",
  "ks",
  "lg",
  "lh",
  "lj",
  "lz",
  "mf",
  "mh",
  "mj",
  "ml",
  "mp",
  "nv",
  "pc",
  "pg",
  "pz",
  "qd",
  "qj",
  "qm",
  "qn",
  "qs",
  "qv",
  "rn",
  "rp",
  "rx",
  "sk",
  "th",
  "tk",
  "tp",
  "ts",
  "vk",
  "vq",
  "vs",
  "xg",
  "xk",
  "xl",
  "zq",
  "zz",
];

export function solvePart2(modules: Record<string, IModule>): {
  low: number;
  high: number;
} {
  const signals: TSignal[] = [];
  const visitor = new Visitor("button", signals);
  const buttonClicked = 1024;
  console.log("1016    " + ORDER.join(" "));

  const statesByButtonClick: number[][] = [];

  buttons: for (let i = 0; i < buttonClicked; i++) {
    visitor.setSource("button");
    visitor.setTarget("broadcaster");
    visitor.low("broadcaster");
    const signalTypes = [];

    while (signals.length > 0) {
      const signal = signals.shift();
      if (!signal) {
        continue;
      }
      if (signal.type === "high") {
        signalTypes.push(1);
      } else {
        signalTypes.push(0);
      }
      visitor.setSource(signal.source);
      visitor.setTarget(signal.output);

      const { type, output } = signal;
      const mod = modules[output];
      if (type === "low") {
        mod.onLow(visitor);
      } else {
        mod.onHigh(visitor);
      }
    }

    const states: number[] = [];
    for (const name of ORDER) {
      if (!modules[name]) {
        throw new Error("Unknown module: " + name);
      }
      const mod = modules[name];
      if (mod instanceof FlipFlop) {
        if (mod.state === "on") {
          states.push(1);
        } else {
          states.push(0);
        }
      }
    }
    statesByButtonClick.push(states);
  }

  for (let n = statesByButtonClick[0].length, j = 0; j < n; j++) {
    const row = [];
    row.push(ORDER[j] + " ");
    let cnt = 0;
    let last = 0;
    let first = -1;
    for (let i = 0; i < statesByButtonClick.length; i++) {
      const curr = statesByButtonClick[i][j];
      if (curr == last) {
        cnt++;
        continue;
      }
      if (cnt > 0) {
        row.push(" (" + [last, cnt].join(", ") + ")");
      }
      cnt = 1;
      last = curr;
      first = i;
    }
    if (cnt > 0) {
      row.push(" (" + [last, cnt].join(", ") + ")");
    }
    console.log(row.join(""));
  }

  return {
    low: visitor.getTotalLow(),
    high: visitor.getTotalHigh(),
  };
}
