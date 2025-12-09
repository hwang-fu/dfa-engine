import { describe, expect, it } from "bun:test";
import { DFA } from "../dfa";
import { DFAValidationError } from "../errors";

describe("DFA Validation", () => {
	it("throws when start state is not in states", () => {
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

	it("throws when a state has no transitions", () => {
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

	it("throws when transition is missing for a symbol", () => {
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

	it("throws when transition leads to unknown state", () => {
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

	it("accepts valid DFA configuration", () => {
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
});

describe("DFA Execution - Binary divisible by 2", () => {
	// Accepts binary strings ending in 0 (even numbers)
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

	it("accepts '0'", () => {
		expect(evenBinaryDFA.run("0")).toBe("accepted");
	});

	it("accepts '10'", () => {
		expect(evenBinaryDFA.run("10")).toBe("accepted");
	});

	it("accepts '1010'", () => {
		expect(evenBinaryDFA.run("1010")).toBe("accepted");
	});

	it("rejects '1'", () => {
		expect(evenBinaryDFA.run("1")).toBe("rejected");
	});

	it("rejects '11'", () => {
		expect(evenBinaryDFA.run("11")).toBe("rejected");
	});

	it("rejects '1011'", () => {
		expect(evenBinaryDFA.run("1011")).toBe("rejected");
	});

	it("rejects empty string", () => {
		expect(evenBinaryDFA.run("")).toBe("rejected");
	});

	it("rejects string with invalid symbol", () => {
		expect(evenBinaryDFA.run("102")).toBe("rejected");
	});
});

describe("DFA Execution - Contains 'ab' substring", () => {
	const containsAB = new DFA({
		states: ["q0", "q1", "q2"],
		alphabet: ["a", "b"],
		transitions: {
			q0: { a: "q1", b: "q0" },
			q1: { a: "q1", b: "q2" },
			q2: { a: "q2", b: "q2" },
		},
		startState: "q0",
		acceptingStates: ["q2"],
	});

	it("accepts 'ab'", () => {
		expect(containsAB.run("ab")).toBe("accepted");
	});

	it("accepts 'aab'", () => {
		expect(containsAB.run("aab")).toBe("accepted");
	});

	it("accepts 'bab'", () => {
		expect(containsAB.run("bab")).toBe("accepted");
	});

	it("accepts 'abbb'", () => {
		expect(containsAB.run("abbb")).toBe("accepted");
	});

	it("rejects 'a'", () => {
		expect(containsAB.run("a")).toBe("rejected");
	});

	it("rejects 'ba'", () => {
		expect(containsAB.run("ba")).toBe("rejected");
	});

	it("rejects 'bbb'", () => {
		expect(containsAB.run("bbb")).toBe("rejected");
	});

	it("rejects empty string", () => {
		expect(containsAB.run("")).toBe("rejected");
	});
});

describe("DFA runWithTrace", () => {
	const dfa = new DFA({
		states: ["q0", "q1"],
		alphabet: ["0", "1"],
		transitions: {
			q0: { "0": "q1", "1": "q0" },
			q1: { "0": "q1", "1": "q0" },
		},
		startState: "q0",
		acceptingStates: ["q1"],
	});

	it("returns correct trace for accepted input", () => {
		const trace = dfa.runWithTrace("10");

		expect(trace.input).toBe("10");
		expect(trace.startState).toBe("q0");
		expect(trace.result).toBe("accepted");
		expect(trace.finalState).toBe("q1");
		expect(trace.steps).toEqual([
			{ fromState: "q0", symbol: "1", toState: "q0" },
			{ fromState: "q0", symbol: "0", toState: "q1" },
		]);
	});

	it("returns correct trace for rejected input", () => {
		const trace = dfa.runWithTrace("11");

		expect(trace.result).toBe("rejected");
		expect(trace.finalState).toBe("q0");
	});

	it("handles empty input", () => {
		const trace = dfa.runWithTrace("");

		expect(trace.steps).toEqual([]);
		expect(trace.finalState).toBe("q0");
		expect(trace.result).toBe("rejected");
	});
});
