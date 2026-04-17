import { Version } from "@jacobhaap/humver";
import { assertEquals, assertThrows } from "jsr:@std/assert@1.0.19";

Deno.test("fromDates", () => {
	const birth = new Date(2003, 3, 17);
	const asOf = new Date(2026, 3, 17);

	const version = Version.fromDates(birth, asOf);
	assertEquals(version.major, 2);
	assertEquals(version.minor, 3);
	assertEquals(version.patch, 0);
});

Deno.test("fromDates negative age", () => {
	const birth = new Date(2000, 0, 1);
	const asOf = new Date(1970, 0, 1);

	assertThrows(
		() => Version.fromDates(birth, asOf),
		RangeError,
		"age cannot be negative"
	);
});

Deno.test("fromDates same year negative", () => {
	const birth = new Date(2000, 11, 12);
	const asOf = new Date(2000, 0, 1);

	assertThrows(
		() => Version.fromDates(birth, asOf),
		RangeError,
		"age cannot be negative"
	);
});

Deno.test("toYears", () => {
	const birth = new Date(2003, 3, 17);
	const asOf = new Date(2026, 3, 17);

	const version = Version.fromDates(birth, asOf);
	assertEquals(version.toYears(), 23);
});

Deno.test("toMonths", () => {
	const birth = new Date(2003, 3, 17);
	const asOf = new Date(2026, 3, 17);

	const version = Version.fromDates(birth, asOf);
	assertEquals(version.toMonths(), 276);
});

Deno.test("toString", () => {
	const birth = new Date(2003, 3, 17);
	const asOf = new Date(2026, 3, 17);

	const version = Version.fromDates(birth, asOf);
	assertEquals(version.toString(), "2.3.0");
});

Deno.test("parse string", () => {
	const v = Version.parse("1.2.3");
	assertEquals(v.major, 1);
	assertEquals(v.minor, 2);
	assertEquals(v.patch, 3);
});

Deno.test("parse string no patch", () => {
	const v = Version.parse("1.2");
	assertEquals(v.major, 1);
	assertEquals(v.minor, 2);
	assertEquals(v.patch, 0);
});

Deno.test("parse string major only", () => {
	const v = Version.parse("1");
	assertEquals(v.major, 1);
	assertEquals(v.minor, 0);
	assertEquals(v.patch, 0);
});

Deno.test("parse string empty", () => {
	assertThrows(
		() => Version.parse(""),
		SyntaxError,
		"invalid version format"
	);
});

Deno.test("parse string too many", () => {
	assertThrows(
		() => Version.parse(""),
		SyntaxError,
		"invalid version format"
	);
});

Deno.test("parse string alpha", () => {
	assertThrows(
		() => Version.parse("a.b.c"),
		SyntaxError,
		"invalid version format"
	);
});

Deno.test("parse invalid minor", () => {
	assertThrows(
		() => Version.parse("1.10.0"),
		RangeError,
		"minor version must be 0-9"
	);
});

Deno.test("parse invalid patch", () => {
	assertThrows(
		() => Version.parse("1.0.12"),
		RangeError,
		"patch version must be 0-11"
	);
});
