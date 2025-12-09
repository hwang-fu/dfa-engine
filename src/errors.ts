export class DFAError extends Error {
	constructor(message: string) {
		super(message);
		this.name = "DFAError";
	}
}

export class DFAValidationError extends DFAError {
	constructor(message: string) {
		super(message);
		this.name = "DFAValidationError";
	}
}
