import { rolldown } from "rolldown";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const rootDir = join(__dirname, "..");
const entryPoint = join(rootDir, "node_modules/bibtex-tidy/bibtex-tidy.js");
const outputFile = join(rootDir, "src/bibtex_tidy_bundle.js");

console.log("Bundling bibtex-tidy with Rolldown (Oxc)...");

const bundle = await rolldown({
  input: entryPoint,
});

await bundle.write({
  file: outputFile,
  format: "iife",
  name: "BibTeXTidyModule",
  footer: `
globalThis.__bibtexTidyFormat = function(input, optionsJson) {
  var options = optionsJson ? JSON.parse(optionsJson) : {};
  var result = BibTeXTidyModule.tidy(input, options);
  return JSON.stringify({ bibtex: result.bibtex, warnings: result.warnings });
};
`,
});

console.log(`Bundle generated successfully at ${outputFile}`);
