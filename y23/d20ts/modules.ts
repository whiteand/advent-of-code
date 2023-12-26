export interface IModuleVisitor {
  source(): string;
  low(name: string): void;
  high(name: string): void;
}
export type TCondition =
  | { type: "always" }
  | { type: "never" }
  | {
      type: "not";
      cond: TCondition;
    }
  | { type: "divisible"; divisibleBy: number; remainder: number };

function and(...conditions: TCondition[]): TCondition {
  return conditions.reduce(
    (cond, cur): TCondition => {
      throw new Error(
        `Cannot AND conditions: ${JSON.stringify(cond)} and ${JSON.stringify(
          cur
        )}`
      );
    },
    {
      type: "always",
    }
  );
}
function or(...conditions: TCondition[]): TCondition {
  return conditions.reduce(
    (cond, cur): TCondition => {
      throw new Error(
        `Cannot OR conditions: ${JSON.stringify(cond)} and ${JSON.stringify(
          cur
        )}`
      );
    },
    {
      type: "never",
    }
  );
}

export interface IConditionVisitor {
  getLowCondition(name: string): TCondition | null;
  getHighCondition(name: string): TCondition | null;
  handleResult(
    type: "low" | "high" | "both",
    name: string,
    lowRes: TConditionResult | null,
    highRes: TConditionResult | null
  ): void;
}

export type TUnknownCondition = { type: "low" | "high" | "both"; name: string };

export type TConditionResult =
  | {
      type: "ok";
      value: TCondition;
    }
  | {
      type: "err";
      unknowns: TUnknownCondition[];
    };

export interface IModule {
  onLow(visitor: IModuleVisitor): void;
  onHigh(visitor: IModuleVisitor): void;
  onInputAttached(name: string): void;
  getOutputs(): string[];
  getName(): string;
  getLowCondition(visitor: IConditionVisitor): TConditionResult;
  getHighCondition(visitor: IConditionVisitor): TConditionResult;
  getInputs(): string[];
}

export class FlipFlop implements IModule {
  private name: string;
  state: "on" | "off";
  private outputs: string[];
  private inputs: string[] = [];
  constructor(name: string, outputs: string[] = []) {
    this.name = name;
    this.state = "off";
    this.outputs = outputs;
  }
  getInputs(): string[] {
    return [...this.inputs];
  }
  getLowCondition(visitor: IConditionVisitor): TConditionResult {
    const parentLowConditions: TCondition[] = [];
    for (const input of this.inputs) {
      const lowCond = visitor.getLowCondition(input);
      if (!lowCond) {
        return {
          type: "err",
          unknowns: [{ type: "low", name: input }],
        };
      }
      parentLowConditions.push(lowCond);
    }
    if (parentLowConditions.length === 0) {
      return {
        type: "ok",
        value: {
          type: "never",
        },
      };
    }
    throw new Error(
      "Cannot get low condition for flip flop with input conditions: " +
        JSON.stringify(parentLowConditions)
    );
  }
  getHighCondition(visitor: IConditionVisitor): TConditionResult {
    const parentLowConditions: TCondition[] = [];
    for (const input of this.inputs) {
      const lowCond = visitor.getLowCondition(input);
      if (!lowCond) {
        return {
          type: "err",
          unknowns: [{ type: "low", name: input }],
        };
      }
      parentLowConditions.push(lowCond);
    }
    if (parentLowConditions.length === 0) {
      return {
        type: "ok",
        value: {
          type: "never",
        },
      };
    }
    throw new Error(
      "Cannot get high condition for flip flop with input conditions: " +
        JSON.stringify(parentLowConditions)
    );
  }
  getOutputs(): string[] {
    return [...this.outputs];
  }

  getName(): string {
    return this.name;
  }
  onLow(visitor: IModuleVisitor): void {
    if (this.state === "on") {
      this.state = "off";
      for (const out of this.outputs) {
        visitor.low(out);
      }
    } else {
      this.state = "on";
      for (const out of this.outputs) {
        visitor.high(out);
      }
    }
  }
  onInputAttached(name: string): void {
    this.inputs.push(name);
  }
  onHigh(_visitor: IModuleVisitor): void {
    // ignores
  }
}

export class Broadcaster implements IModule {
  private name: string;
  private outputs: string[];
  private inputs: string[] = [];
  constructor(name: string, outputs: string[]) {
    this.name = name;
    this.outputs = outputs;
  }
  getInputs(): string[] {
    return [...this.inputs];
  }
  getOutputs(): string[] {
    return [...this.outputs];
  }
  getName(): string {
    return this.name;
  }
  getLowCondition(visitor: IConditionVisitor): TConditionResult {
    return {
      type: "ok",
      value: {
        type: "always",
      },
    };
  }
  getHighCondition(visitor: IConditionVisitor): TConditionResult {
    return {
      type: "ok",
      value: {
        type: "never",
      },
    };
  }
  onLow(visitor: IModuleVisitor): void {
    for (const out of this.outputs) {
      visitor.low(out);
    }
  }
  onHigh(visitor: IModuleVisitor): void {
    for (const out of this.outputs) {
      visitor.low(out);
    }
  }
  onInputAttached(name: string): void {
    this.inputs.push(name);
  }
}

export class Output implements IModule {
  private name: string;
  private highs: number;
  private lows: number;
  private inputs: string[];
  constructor(name: string) {
    this.name = name;
    this.highs = 0;
    this.lows = 0;
    this.inputs = [];
  }
  getInputs(): string[] {
    return [...this.inputs];
  }
  onLow(visitor: IModuleVisitor): void {
    this.lows += 1;
  }
  onHigh(visitor: IModuleVisitor): void {
    this.highs += 1;
  }
  getLowCondition(visitor: IConditionVisitor): TConditionResult {
    return {
      type: "ok",
      value: {
        type: "never",
      },
    };
  }
  getHighCondition(visitor: IConditionVisitor): TConditionResult {
    return {
      type: "ok",
      value: {
        type: "never",
      },
    };
  }

  onInputAttached(name: string): void {
    this.inputs.push(name);
  }
  getOutputs(): string[] {
    return [];
  }
  getName(): string {
    return this.name;
  }
}

export class Conjunction implements IModule {
  private name: string;
  private inputs: string[];
  private lastSignal: Record<string, "high" | "low">;
  private outputs: string[];
  constructor(name: string, outputs: string[]) {
    this.name = name;
    this.inputs = [];
    this.lastSignal = {};
    this.outputs = outputs;
  }
  getInputs(): string[] {
    return [...this.inputs];
  }
  getLowCondition(visitor: IConditionVisitor): TConditionResult {
    const inputHighConditions: TCondition[] = [];
    for (const input of this.inputs) {
      const highCond = visitor.getHighCondition(input);
      if (!highCond) {
        return {
          type: "err",
          unknowns: [{ type: "high", name: input }],
        };
      }
      inputHighConditions.push(highCond);
    }
    return {
      type: "ok",
      value: and(...inputHighConditions),
    };
  }
  getHighCondition(visitor: IConditionVisitor): TConditionResult {
    const inputLowConditions: TCondition[] = [];
    for (const input of this.inputs) {
      const lowCond = visitor.getLowCondition(input);
      if (!lowCond) {
        return {
          type: "err",
          unknowns: [{ type: "low", name: input }],
        };
      }
      inputLowConditions.push(lowCond);
    }
    return {
      type: "ok",
      value: or(...inputLowConditions),
    };
  }
  onLow(visitor: IModuleVisitor): void {
    this.lastSignal[visitor.source()] = "low";
    this.execute(visitor);
  }
  private execute(visitor: IModuleVisitor) {
    if (this.inputs.every((input) => this.lastSignal[input] === "high")) {
      for (const out of this.outputs) {
        visitor.low(out);
      }
    } else {
      for (const out of this.outputs) {
        visitor.high(out);
      }
    }
  }
  onHigh(visitor: IModuleVisitor): void {
    this.lastSignal[visitor.source()] = "high";
    this.execute(visitor);
  }
  onInputAttached(name: string): void {
    if (this.inputs.includes(name)) {
      return;
    }
    this.inputs.push(name);
    this.lastSignal[name] = "low";
  }
  getOutputs(): string[] {
    return [...this.outputs];
  }
  getName(): string {
    return this.name;
  }
}
