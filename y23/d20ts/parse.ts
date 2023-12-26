import { Broadcaster, Conjunction, FlipFlop, IModule, Output } from "./modules";

export function parse(input: string): Record<string, IModule> {
  const inputLines = input.split("\n");

  const modules: Record<string, IModule> = {};

  for (const line of inputLines) {
    const [left, right] = line.split(" -> ");
    const outputs = right.split(", ");
    switch (left[0]) {
      case "%":
        const name = left.slice(1);
        modules[name] = new FlipFlop(name, outputs);
        break;
      case "&":
        const conName = left.slice(1);
        modules[conName] = new Conjunction(conName, outputs);
        break;
      default:
        modules[left] = new Broadcaster(left, outputs);
        break;
    }
  }

  for (const [name, mod] of Object.entries(modules)) {
    const outputs = mod.getOutputs();
    for (const out of outputs) {
      const outMod = modules[out];
      if (!outMod) {
        modules[out] = new Output(out);
        modules[out].onInputAttached(name);
        continue;
      }
      modules[out].onInputAttached(name);
    }
  }

  modules["broadcaster"].onInputAttached("button");

  return modules;
}
