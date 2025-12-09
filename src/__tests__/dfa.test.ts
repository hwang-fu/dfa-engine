import { describe, expect, it } from 'bun:test';
import { DFA } from '../dfa';
import { DFAValidationError } from '../errors';

describe('DFA Validation', () => {
  it('throws when start state is not in states', () => {
    expect(() => {
      new DFA({
				states: ["q0", "q1"],
				alphabet: ["0", "1"],
				transitions: {
					q0: { "0": "q1", "1": "q0" },
					q1: { "0": "q1", "1": "q0" },
				},
				startState: "q2",
				acceptingStates: ["q1"],
      });
    }).toThrow(DFAValidationError);
  });
});
