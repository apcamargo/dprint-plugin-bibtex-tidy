# dprint-plugin-bibtex-tidy

[![CI](https://github.com/apcamargo/dprint-plugin-bibtex-tidy/workflows/CI/badge.svg)](https://github.com/apcamargo/dprint-plugin-bibtex-tidy/actions?query=workflow%3ACI)

Adapter plugin for [dprint](https://github.com/dprint/dprint) that formats BibTeX files via [bibtex-tidy](https://github.com/FlamingTempura/bibtex-tidy).

## Install

Add the plugin release URL to your `dprint.json`:

```jsonc
{
  "bibtex-tidy": {
    // bibtex-tidy's config goes here
  },
  "plugins": [
    "https://plugins.dprint.dev/apcamargo/bibtex-tidy-0.1.0.wasm"
  ]
}
```

## Configuration

To add configuration, specify a `"bibtex-tidy"` key in your `dprint.json`:

```jsonc
{
  "bibtex-tidy": {
    "curly": true,
    "numeric": true,
    "align": 14,
    "sort": ["-year", "title"],
    "trailingCommas": true,
    "lineEnding": "lf"
  },
  "plugins": [
    // ...etc...
  ]
}
```

### Properties

| Property                | Type                  | Default                           | Description                                                                                                              |
| ----------------------- | --------------------- | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `lineWidth`             | `integer`             | unset                             | When configured locally or in dprint globals, wraps at this column (numeric `--wrap`).                                   |
| `indentWidth`           | `integer`             | `2`                               | Number of spaces per indentation level (`--space`).                                                                      |
| `useTabs`               | `boolean`             | `false`                           | Indent with tabs instead of spaces (`--tab`).                                                                            |
| `space`                 | `integer`             | `2`                               | Alias for `indentWidth` (CLI parity).                                                                                    |
| `tab`                   | `boolean`             | `false`                           | Alias for `useTabs` (CLI parity).                                                                                        |
| `align`                 | `integer \| boolean`  | `14`                              | Align field values at column (or `false` / `1` to disable).                                                              |
| `blankLines`            | `boolean`             | `false`                           | Insert an empty line between each bibliography entry.                                                                    |
| `curly`                 | `boolean`             | `false`                           | Enclose all property values in braces (`"..."` -> `{...}`).                                                              |
| `numeric`               | `boolean`             | `false`                           | Strip braces and quotes from numeric and month values.                                                                   |
| `months`                | `boolean`             | `false`                           | Convert month names to standard 3-letter macros (`jan`, `feb`, ...).                                                     |
| `sort`                  | `boolean \| string[]` | `false`                           | Sort entries by citation key or specified fields (`-` for descending).                                                   |
| `duplicates`            | `boolean \| string[]` | `false`                           | Check/warn for duplicates (`doi`, `key`, `abstract`, `citation`).                                                        |
| `merge`                 | `boolean \| string`   | `false`                           | Merge duplicate entries (`first`, `last`, `combine`, `overwrite`).                                                       |
| `stripEnclosingBraces`  | `boolean`             | `false`                           | Strip double braces around entire values (`{{...}}` -> `{...}`).                                                         |
| `dropAllCaps`           | `boolean`             | `false`                           | Convert all-caps fields to Title Case preserving Roman numerals.                                                         |
| `escape`                | `boolean \| string`   | `true`                            | Escape special characters to LaTeX macros (`true`, `false`, `"new"`).                                                    |
| `unescape`              | `boolean`             | `false`                           | Convert LaTeX escapes back to Unicode characters.                                                                        |
| `sortFields`            | `boolean \| string[]` | `false`                           | Sort fields within entries by standard or custom order.                                                                  |
| `stripComments`         | `boolean`             | `false`                           | Remove all comments from the BibTeX source.                                                                              |
| `tidyComments`          | `boolean`             | `true`                            | Normalize whitespace surrounding comments.                                                                               |
| `trailingCommas`        | `boolean`             | `false`                           | Ensure the last field in each entry ends with a trailing comma.                                                          |
| `encodeUrls`            | `boolean`             | `false`                           | Percent-encode special characters in URLs.                                                                               |
| `removeEmptyFields`     | `boolean`             | `false`                           | Remove fields with empty values.                                                                                         |
| `removeDuplicateFields` | `boolean`             | `true`                            | Keep only one instance of each field per entry.                                                                          |
| `generateKeys`          | `boolean \| string`   | `false`                           | Replace citation keys using JabRef citation key templates.                                                               |
| `maxAuthors`            | `integer`             | `null`                            | Truncate author lists exceeding N authors to `and others`.                                                               |
| `lowercase`             | `boolean`             | `true`                            | Lowercase field names and entry types.                                                                                   |
| `enclosingBraces`       | `boolean \| string[]` | `false`                           | Enclose specified fields in double braces (`{{...}}`).                                                                   |
| `removeBraces`          | `boolean \| string[]` | `false`                           | Strip curly braces inside field values.                                                                                  |
| `wrap`                  | `integer \| boolean`  | unset                             | Matches no `--wrap` flag. A number sets the column; `true` uses 80; `false` disables wrapping.                           |
| `omit`                  | `string[]`            | `[]`                              | Remove specified fields from bibliography entries.                                                                       |
| `lineEnding`            | `string`              | dprint global (`"lf"` when unset) | Line ending style (`"lf"` or `"crlf"`); overrides global `newLineKind`, while global `"auto"` preserves the input style. |

## Versioning

This plugin is released independently from bibtex-tidy. The version of `bibtex-tidy` used by the plugin is listed in [package.json](https://github.com/apcamargo/dprint-plugin-bibtex-tidy/blob/main/package.json) and is checked for updates by this repository's monthly update workflow.

## Development

Build and test locally:

```sh
# Install the locked JS dependencies
npm ci

# Bundle bibtex-tidy via Rolldown (Oxc)
npm run build:bundle

# Run unit and spec tests
cargo test

# Run clippy checks
cargo clippy --all-targets --all-features -- -D warnings

# Build Wasm module
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --features wasm --release
```
