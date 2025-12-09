import { DFAValidationError } from "./errors";
import type { DFAConfig, DFAResult, ExecutionTrace } from "./types";

export class DFA {
	private readonly states: Set<string>;
	private readonly alphabet: Set<string>;
	private readonly transitions: Map<string, Map<string, string>>;
	private readonly startState: string;
	private readonly acceptingStates: Set<string>;

	constructor(config: DFAConfig) {
		this.states = new Set(config.states);
		this.alphabet = new Set(config.alphabet);
		this.transitions = this.buildTransitionMap(config.transitions);
		this.startState = config.startState;
		this.acceptingStates = new Set(config.acceptingStates);

		this.validate();
	}

	private buildTransitionMap(
		transitions: Record<string, Record<string, string>>,
	): Map<string, Map<string, string>> {
		const map = new Map<string, Map<string, string>>();

		for (const [state, stateTransitions] of Object.entries(transitions)) {
			map.set(state, new Map(Object.entries(stateTransitions)));
		}

		return map;
	}

	private validate(): void {
		// check start state belongs to the Q
		if (!this.states.has(this.startState)) {
			throw new DFAValidationError(
				`Start state "${this.startState}" is not in the set of states`,
			);
		}

		// check accepting / final states is a subset to the Q
		for (const acceptingState of this.acceptingStates) {
			if (!this.states.has(acceptingState)) {
				throw new DFAValidationError(
					`Accepting state "${acceptingState}" is not in the set of states`,
				);
			}
		}

		// check transitions are total: every state has transitions for every symbol
		for (const state of this.states) {
			const stateTransitions = this.transitions.get(state);
			if (stateTransitions === undefined) {
				throw new DFAValidationError(
					`State "${state}" has no transitions defined`,
				);
			}

			for (const symbol of this.alphabet) {
				const targetState = stateTransitions.get(symbol);
				if (targetState === undefined) {
					throw new DFAValidationError(
						`Missing transition for state "${state}" on symbol "${symbol}"`,
					);
				}

				if (!this.states.has(targetState)) {
					throw new DFAValidationError(
						`Transition from "${state}" on "${symbol}" leads to unknown state "${targetState}"`,
					);
				}
			}
		}
	}

	run(input: string): DFAResult {
		let currentState = this.startState;

		for (const symbol of input) {
			if (!this.alphabet.has(symbol)) {
				return "rejected";
			}

			const stateTransitions = this.transitions.get(currentState);
			const nextState = stateTransitions?.get(symbol);
			if (nextState === undefined) {
				return "rejected";
			}

			currentState = nextState;
		}

		return this.acceptingStates.has(currentState) ? "accepted" : "rejected";
	}

	runWithTrace(input: string): ExecutionTrace {
		const steps: ExecutionTrace["steps"] = [];
		let currentState = this.startState;

		for (const symbol of input) {
			const stateTransitions = this.transitions.get(currentState);
			const nextState = stateTransitions?.get(symbol);
			if (!this.alphabet.has(symbol) || nextState === undefined) {
				return {
					input,
					startState: this.startState,
					steps,
					finalState: currentState,
					result: "rejected",
				};
			}

			steps.push({
				fromState: currentState,
				toState: nextState,
				symbol,
			});

			currentState = nextState;
		}

		return {
			input,
			startState: this.startState,
			steps,
			finalState: currentState,
			result: this.acceptingStates.has(currentState) ? "accepted" : "rejected",
		};
	}
}
