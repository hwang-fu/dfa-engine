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


