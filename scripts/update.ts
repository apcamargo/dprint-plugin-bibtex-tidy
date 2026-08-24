import { $ } from "automation";

// 1. Query npm registry for newest bibtex-tidy
const res = await fetch("https://registry.npmjs.org/bibtex-tidy/latest");
const data = await res.json();
const latestVersion = data.version as string;

console.log(`Latest bibtex-tidy on npm: ${latestVersion}`);

// 2. Check current package.json
const pkgJson = JSON.parse(await Deno.readTextFile("package.json"));
const currentVersion = pkgJson.dependencies["bibtex-tidy"]?.replace(/[\^~]/, "");

if (currentVersion === latestVersion) {
  console.log(`Already on latest bibtex-tidy version (${currentVersion}).`);
  Deno.exit(0);
}

console.log(`Updating bibtex-tidy: ${currentVersion} -> ${latestVersion}`);
pkgJson.dependencies["bibtex-tidy"] = `^${latestVersion}`;
await Deno.writeTextFile("package.json", JSON.stringify(pkgJson, null, 2) + "\n");
await $`npm install`;

// 3. Re-bundle with Rolldown
await $`node ./scripts/build_bundle.js`;

// 4. Verification pipeline
await $`cargo test --locked`;
await $`cargo clippy --locked --all-targets --all-features -- -D warnings`;
await $`cargo build --locked --target wasm32-unknown-unknown --features wasm --release`;

console.log("Update succeeded!");
