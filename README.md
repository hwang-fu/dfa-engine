# dfa-engine

A TypeScript implementation of a Deterministic Finite Automaton (DFA) engine.

[![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue.svg)](https://www.typescriptlang.org/)
[![Bun](https://img.shields.io/badge/Bun-1.0-black.svg)](https://bun.sh/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## What is a DFA?

A **Deterministic Finite Automaton** is a theoretical model of computation defined by the 5-tuple:

**DFA = (Q, Σ, δ, q₀, F)**

| Symbol | Meaning | Description |
|--------|---------|-------------|
| Q | States | Finite set of states |
| Σ | Alphabet | Finite set of input symbols |
| δ | Transition function | Q × Σ → Q |
| q₀ | Start state | Initial state (q₀ ∈ Q) |
| F | Accepting states | Set of final states (F ⊆ Q) |

## Installation

Clone and install locally:
```bash
git clone https://github.com/hwang-fu/dfa-engine.git
cd dfa-engine
bun install
```

## Usage
```typescript
import { DFA } from "dfa-engine";

// DFA that accepts binary strings ending in "0" (even numbers)
const evenBinaryDFA = new DFA({
  states: ["q0", "q1"],
  alphabet: ["0", "1"],
  transitions: {
    q0: { "0": "q1", "1": "q0" },
    q1: { "0": "q1", "1": "q0" },
  },
  startState: "q0",
  acceptingStates: ["q1"],
});

console.log(evenBinaryDFA.run("1010")); // "accepted"
console.log(evenBinaryDFA.run("1011")); // "rejected"
```

## Debugging with Execution Trace
```typescript
const trace = evenBinaryDFA.runWithTrace("10");

console.log(trace);
// {
//   input: "10",
//   startState: "q0",
//   steps: [
//     { fromState: "q0", symbol: "1", toState: "q0" },
//     { fromState: "q0", symbol: "0", toState: "q1" }
//   ],
//   finalState: "q1",
//   result: "accepted"
// }
```


