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

	it("throws when accepting state is not in states", () => {
		expect(() => {
			new DFA({
				states: ["q0", "q1"],
				alphabet: ["0", "1"],
				transitions: {
					q0: { "0": "q1", "1": "q0" },
					q1: { "0": "q1", "1": "q0" },
				},
				startState: "q0",
				acceptingStates: ["q2"],
			});
		}).toThrow(DFAValidationError);
	});

	it('throws when a state has no transitions', () => {
		expect(() => {
			new DFA({
				states: ["q0", "q1"],
				alphabet: ["0", "1"],
				transitions: {
					q0: { "0": "q1", "1": "q0" },
				},
				startState: "q0",
				acceptingStates: ["q1"],
			});
		}).toThrow(DFAValidationError);
	});

	it('throws when transition is missing for a symbol', () => {
		expect(() => {
			new DFA({
				states: ["q0", "q1"],
				alphabet: ["0", "1"],
				transitions: {
					q0: { "0": "q1" },
					q1: { "0": "q1", "1": "q0" },
				},
				startState: "q0",
				acceptingStates: ["q1"],
			});
		}).toThrow(DFAValidationError);
	});

	it('throws when transition leads to unknown state', () => {
		expect(() => {
			new DFA({
				states: ["q0", "q1"],
				alphabet: ["0", "1"],
				transitions: {
					q0: { "0": "q2", "1": "q0" },
					q1: { "0": "q1", "1": "q0" },
				},
				startState: "q0",
				acceptingStates: ["q1"],
			});
		}).toThrow(DFAValidationError);
	});

  it('accepts valid DFA configuration', () => {
    expect(() => {
      new DFA({
        states: ["q0", "q1"],
        alphabet: ["0", "1"],
        transitions: {
          q0: { "0": "q1", "1": "q0" },
          q1: { "0": "q1", "1": "q0" },
        },
        startState: "q0",
        acceptingStates: ["q1"],
      });
    }).not.toThrow();
  });

  it('', () => {
  });
});
