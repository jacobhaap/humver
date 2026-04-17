/**
 * A TypeScript implementation of Human Versions.
 *
 * Provides a semantic human versioning system, representing human age at
 * different time scales. Major versions represent the number of decades
 * lived, incrementing every 10 years. Minor versions represent the number
 * of years lived within the current decade, and patch versions represent
 * the number of months lived within the current year.
 * @module
 * @author Jacob V. B. Haap <iacobus.xyz>
 * @license MIT
 */

/**
 * Human Version number representing age in a `major.minor.patch` format.
 *
 * Age as a semantic human version, where the components represent different
 * time scales:
 * - `major`: Number of decades lived (increments every 10 years).
 * - `minor`: Years within the current decade (0-9).
 * - `patch`: Months within the current year (0-11).
 */
export class Version {
	private _major: number;
	private _minor: number;
	private _patch: number;

	private constructor(major: number, minor: number, patch: number) {
		this._major = major;
		this._minor = minor;
		this._patch = patch;
	}

	/** Create a Human Version from a birth and as-of date. */
	static fromDates(birth: Date, asOf: Date): Version {
		if (asOf < birth) {
			throw new RangeError("age cannot be negative");
		}

		const totalMonths = Math.max(
			0,
			(asOf.getFullYear() - birth.getFullYear()) * 12 + asOf.getMonth() -
				birth.getMonth() - (asOf.getDate() < birth.getDate() ? 1 : 0)
		);

		const major = Math.floor(totalMonths / 120);
		const minor = Math.floor((totalMonths % 120) / 12);
		const patch = totalMonths % 12;

		return new Version(major, minor, patch);
	}

	/** Parse a Human Version from a string. */
	static parse(str: string): Version {
		const parts = str.split(".");

		if (parts.length < 1 || parts.length > 3) {
			throw new SyntaxError(`invalid version format: ${str}`);
		}

		const major = parseInt(parts[0]);
		const minor = parts[1] ? parseInt(parts[1]) : 0;
		const patch = parts[2] ? parseInt(parts[2]) : 0;

		if (isNaN(major) || isNaN(minor) || isNaN(patch)) {
			throw new SyntaxError(`invalid version format: ${str}`);
		}

		if (minor >= 10) {
			throw new RangeError("minor version must be 0-9");
		}
		if (patch >= 12) {
			throw new RangeError("patch version must be 0-11");
		}

		return new Version(major, minor, patch);
	}

	/** Returns an age in years from the Human Version. */
	toYears(): number {
		return this._major * 10 + this._minor;
	}

	/** Returns an age in months from the Human Version. */
	toMonths(): number {
		return this._major * 120 + this._minor * 12 + this._patch;
	}

	/** Human major version. */
	get major(): number {
		return this._major;
	}

	/**
	 * Human minor version.
	 *
	 * The value ranges from 0 to 9.
	 */
	get minor(): number {
		return this._minor;
	}

	/**
	 * Human patch version.
	 *
	 * The value ranges from 0 to 11.
	 */
	get patch(): number {
		return this._patch;
	}

	/** Convert the Human Version to a string. */
	toString(): string {
		return `${this._major}.${this._minor}.${this._patch}`;
	}
}
