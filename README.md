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
    "newLineKind": "auto"
  },
  "plugins": [
    // ...etc...
  ]
}
```

### Properties

| Property                | Type                  | Default                           | Description |
| ----------------------- | --------------------- | --------------------------------- | ----------- |
| `lineWidth`             | `integer`             | `80`                              | Column width to wrap at when `wrap` is `true`. Falls back to the global dprint `lineWidth`, then `80` if neither is set. Ignored when `wrap` is `false`. |
| `indentWidth`           | `integer`             | `2`                               | Spaces per indent level. Ignored when `useTabs` is `true`. |
| `newLineKind`           | `string`              | dprint global (`"lf"` when unset) | Line ending to use. `auto` keeps the file's existing ending, `system` matches the OS (CRLF on Windows, LF elsewhere). Falls back to the dprint global setting, then `lf`. |
| `useTabs`               | `boolean`             | `false`                           | Use tabs for indentation instead of spaces. |
| `align`                 | `integer \| boolean`  | `14`                              | Column to align field values at. `false` or `1` leaves a single space instead of aligning. |
| `blankLines`            | `boolean`             | `false`                           | Put an empty line between entries. |
| `curly`                 | `boolean`             | `false`                           | Wrap values in braces, so `"..."` becomes `{...}`. |
| `numeric`               | `boolean`             | `false`                           | Drop braces and quotes from numeric and month values. |
| `months`                | `boolean`             | `false`                           | Turn month names into the standard three-letter macros (`jan`, `feb`, and so on). |
| `sort`                  | `boolean \| string[]` | `false`                           | Sort entries. `true` sorts by citation key, or list the fields you want. Prefix a field with `-` for descending. |
| `duplicates`            | `boolean \| string[]` | `false`                           | Flag duplicate entries by `doi`, `key`, `abstract`, or `citation`. `true` checks all four. If unset and `merge` is on, defaults to `doi`, `citation`, `abstract`. |
| `merge`                 | `boolean \| string`   | `false`                           | Merge duplicate entries: `first`, `last`, `combine`, or `overwrite`. Turns on duplicate checking if it is not already enabled. |
| `stripEnclosingBraces`  | `boolean`             | `false`                           | Strip an outer pair of double braces, so `{{Journal}}` becomes `{Journal}`. |
| `dropAllCaps`           | `boolean`             | `false`                           | Turns all-caps values into title case, so `TITLE` becomes `Title`. Leaves Roman numerals such as `IV` unchanged. |
| `escape`                | `boolean \| string`   | `true`                            | Escape special characters to LaTeX macros. `true` uses the legacy macro list, `"new"` uses only package-independent escapes, `false` disables escaping. |
| `unescape`              | `boolean`             | `false`                           | Turn LaTeX escapes back into Unicode. |
| `sortFields`            | `boolean \| string[]` | `false`                           | Order fields inside each entry. `true` uses bibtex-tidy's standard order, or pass your own list. |
| `stripComments`         | `boolean`             | `false`                           | Delete all comments from the source. |
| `tidyComments`          | `boolean`             | `true`                            | Clean up whitespace around comments. |
| `trailingCommas`        | `boolean`             | `false`                           | Keep a trailing comma on the last field of each entry. |
| `encodeUrls`            | `boolean`             | `false`                           | Percent-encode invalid characters in URL values. |
| `removeEmptyFields`     | `boolean`             | `false`                           | Drop fields with an empty value. |
| `removeDuplicateFields` | `boolean`             | `true`                            | Keep only the first copy of a repeated field. |
| `generateKeys`          | `boolean \| string`   | `false`                           | Replace citation keys using a JabRef pattern, like `[auth][year]`. |
| `maxAuthors`            | `integer`             | `null`                            | Cut author lists to N names, appending `and others`. |
| `lowercase`             | `boolean`             | `true`                            | Lowercase entry types and field names. |
| `enclosingBraces`       | `boolean \| string[]` | `false`                           | Wrap the given fields in `{{...}}` so BibTeX preserves their case. |
| `removeBraces`          | `boolean \| string[]` | `false`                           | Strip braces inside values, unless they are part of a LaTeX command. |
| `wrap`                  | `boolean`             | `false`                           | Wrap long values at `lineWidth` (`80` if unset) when `true`. No effect when `false`, the default. |
| `omit`                  | `string[]`            | `[]`                              | Delete these fields entirely from every entry. |

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
