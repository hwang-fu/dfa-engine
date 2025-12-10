/**
 * DFA = (Q, Σ, δ, q₀, F)
 *
 * Q  = Set of states
 * Σ  = Input alphabet
 * δ  = Transition function: Q × Σ → Q
 * q₀ = Start state
 * F  = Set of accepting states
 */
export interface DFAConfig {
	states: readonly string[];

	alphabet: readonly string[];

	transitions: Record<string, Record<string, string>>;

	startState: string;

	acceptingStates: readonly string[];
}

export type DFAResult = "accepted" | "rejected";

export interface ExecutionStep {
	fromState: string;
	toState: string;
	symbol: string;
}

export interface ExecutionTrace {
	input: string;
	startState: string;
	steps: ExecutionStep[];
	finalState: string;
	result: DFAResult;
}
