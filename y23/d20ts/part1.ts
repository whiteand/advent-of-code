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

export function solvePart1(
  modules: Record<string, IModule>,
  buttonClicked: number
): { low: number; high: number } {
  const signals: TSignal[] = [];
  const visitor = new Visitor("button", signals);
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
  }

  return {
    low: visitor.getTotalLow(),
    high: visitor.getTotalHigh(),
  };
}
