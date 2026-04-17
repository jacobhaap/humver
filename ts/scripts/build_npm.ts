import { build, emptyDir } from "jsr:@deno/dnt@0.42.3";

await emptyDir("./npm");

await build({
	entryPoints: [
		"./src/version.ts",
	],
	outDir: "./npm",
	shims: {
		deno: true
	},
	test: false,
	package: {
		// package.json properties
		name: Deno.args[0],
		version: Deno.args[1],
		description: "Semantic Human Versioning.",
		license: "MIT",
		homepage: "https://github.com/jacobhaap/humver/ts",
		repository: {
			type: "git",
			url: "git+https://gitlab.com/jacobhaap/humver.git"
		},
		bugs: {
			url: "https://github.com/jacobhaap/humver/issues"
		},
		author: {
			name: "Jacob V. B. Haap",
			url: "https://iacobus.xyz/"
		},
		keywords: [
			"human",
			"version",
			"semver"
		]
	},
	postBuild() {
		// steps to run after building and before running the tests
		Deno.copyFileSync("LICENSE", "npm/LICENSE");
		Deno.copyFileSync("README.md", "npm/README.md");
	}
});
