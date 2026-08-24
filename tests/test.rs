use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_bibtex_tidy::configuration::Configuration;
use dprint_plugin_bibtex_tidy::configuration::resolve_config;
use dprint_plugin_bibtex_tidy::*;
use serde_json::json;

const UNFORMATTED_BIBTEX: &str = "@article{key,title={Hello}}\n";
const FORMATTED_LF: &str = "@article{key,\n  title         = {Hello}\n}\n";
const FORMATTED_CRLF: &str = "@article{key,\r\n  title         = {Hello}\r\n}\r\n";
const LONG_VALUE: &str = "@article{key,\n  title         = {This is a very long title that should certainly be wrapped when formatted with a short line width limit}\n}\n";
const WRAPPED_AT_40: &str = "@article{key,\n  title         = {\n    This is a very long title that\n    should certainly be wrapped when\n    formatted with a short line width\n    limit\n  }\n}\n";

#[test]
fn formats_documented_bibtex_tidy_options() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.bib",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    Arc::new(move |file_path, file_text, spec_config| {
      let config = resolve_plugin_config(spec_config.clone().into(), global_config.clone());
      format_text(file_path, file_text, &config)
    }),
    Arc::new(|_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  );
}

#[test]
fn global_line_width_controls_wrapping() {
  let formatted = format_with_resolved_config(
    LONG_VALUE,
    json!({}),
    GlobalConfiguration {
      line_width: Some(40),
      ..GlobalConfiguration::default()
    },
  )
  .expect("formatting succeeds")
  .expect("input needs formatting");

  assert_eq!(formatted, WRAPPED_AT_40);
}

#[test]
fn global_auto_preserves_crlf_and_is_idempotent() {
  let global_config = GlobalConfiguration {
    new_line_kind: Some(NewLineKind::Auto),
    ..GlobalConfiguration::default()
  };
  let formatted = format_with_resolved_config("@article{key,title={Hello}}\r\n", json!({}), global_config.clone())
    .expect("formatting succeeds")
    .expect("input needs formatting");

  assert_eq!(formatted, FORMATTED_CRLF);
  assert_eq!(
    format_with_resolved_config(&formatted, json!({}), global_config).expect("formatting succeeds"),
    None,
  );
}

#[test]
fn global_auto_preserves_lf() {
  let formatted = format_with_resolved_config(
    UNFORMATTED_BIBTEX,
    json!({}),
    GlobalConfiguration {
      new_line_kind: Some(NewLineKind::Auto),
      ..GlobalConfiguration::default()
    },
  )
  .expect("formatting succeeds")
  .expect("input needs formatting");

  assert_eq!(formatted, FORMATTED_LF);
}

#[test]
fn global_auto_uses_lf_without_input_newline() {
  let formatted = format_with_resolved_config(
    "@article{key,title={Hello}}",
    json!({}),
    GlobalConfiguration {
      new_line_kind: Some(NewLineKind::Auto),
      ..GlobalConfiguration::default()
    },
  )
  .expect("formatting succeeds")
  .expect("input needs formatting");

  assert_eq!(formatted, FORMATTED_LF);
}

#[test]
fn plugin_lf_line_ending_overrides_global_auto() {
  let formatted = format_with_resolved_config(
    "@article{key,title={Hello}}\r\n",
    json!({ "lineEnding": "lf" }),
    GlobalConfiguration {
      new_line_kind: Some(NewLineKind::Auto),
      ..GlobalConfiguration::default()
    },
  )
  .expect("formatting succeeds")
  .expect("input needs formatting");

  assert_eq!(formatted, FORMATTED_LF);
}

#[test]
fn plugin_crlf_line_ending_overrides_global_auto() {
  let formatted = format_with_resolved_config(
    UNFORMATTED_BIBTEX,
    json!({ "lineEnding": "crlf" }),
    GlobalConfiguration {
      new_line_kind: Some(NewLineKind::Auto),
      ..GlobalConfiguration::default()
    },
  )
  .expect("formatting succeeds")
  .expect("input needs formatting");

  assert_eq!(formatted, FORMATTED_CRLF);
}

#[test]
fn malformed_bibtex_returns_an_error() {
  let invalid_bibtex = "@article{foobar,\n  title {My first paper},\n  author = {Leg, Table}\n}\n";

  assert!(
    format_with_resolved_config(invalid_bibtex, json!({}), GlobalConfiguration::default()).is_err(),
    "malformed BibTeX input must return an error"
  );
}

#[test]
fn published_schema_and_resolver_validate_documented_configurations() {
  let schema = schema();
  let validator = jsonschema::validator_for(&schema).expect("schema.json must compile");
  let properties = schema_properties(&schema);
  let cases = vec![
    ("lineWidth", vec![json!(80)], vec![json!(0)]),
    ("indentWidth", vec![json!(4)], vec![json!(-1), json!(256)]),
    ("useTabs", vec![json!(true)], vec![json!(1)]),
    ("space", vec![json!(4)], vec![json!(-1), json!(256)]),
    ("tab", vec![json!(true)], vec![json!(1)]),
    ("align", vec![json!(16), json!(false)], vec![json!(-1)]),
    ("blankLines", vec![json!(true)], vec![json!(1)]),
    ("curly", vec![json!(true)], vec![json!(1)]),
    ("numeric", vec![json!(true)], vec![json!(1)]),
    ("months", vec![json!(true)], vec![json!(1)]),
    (
      "sort",
      vec![json!(true), json!(["-year", "title"])],
      vec![json!(123), json!([1])],
    ),
    (
      "duplicates",
      vec![json!(true), json!(["doi", "key", "abstract", "citation"])],
      vec![json!(["unknown"]), json!([1])],
    ),
    (
      "merge",
      vec![
        json!(true),
        json!("first"),
        json!("last"),
        json!("combine"),
        json!("overwrite"),
      ],
      vec![json!("unknown"), json!(123)],
    ),
    ("stripEnclosingBraces", vec![json!(true)], vec![json!(1)]),
    ("dropAllCaps", vec![json!(true)], vec![json!(1)]),
    (
      "escape",
      vec![json!(true), json!("new")],
      vec![json!("legacy"), json!(123)],
    ),
    ("unescape", vec![json!(true)], vec![json!(1)]),
    (
      "sortFields",
      vec![json!(true), json!(["title", "author"])],
      vec![json!(123), json!([1])],
    ),
    ("stripComments", vec![json!(true)], vec![json!(1)]),
    ("trailingCommas", vec![json!(true)], vec![json!(1)]),
    ("encodeUrls", vec![json!(true)], vec![json!(1)]),
    ("tidyComments", vec![json!(true)], vec![json!(1)]),
    ("removeEmptyFields", vec![json!(true)], vec![json!(1)]),
    ("removeDuplicateFields", vec![json!(true)], vec![json!(1)]),
    (
      "generateKeys",
      vec![json!(true), json!("[auth][year]")],
      vec![json!(123)],
    ),
    ("maxAuthors", vec![json!(3)], vec![json!(0)]),
    ("lowercase", vec![json!(true)], vec![json!(1)]),
    (
      "enclosingBraces",
      vec![json!(true), json!(["title"])],
      vec![json!(123), json!([1])],
    ),
    (
      "removeBraces",
      vec![json!(true), json!(["abstract"])],
      vec![json!(123), json!([1])],
    ),
    ("wrap", vec![json!(80), json!(false)], vec![json!(0)]),
    ("omit", vec![json!(["abstract", "file"])], vec![json!(123), json!([1])]),
    (
      "lineEnding",
      vec![json!("lf"), json!("crlf")],
      vec![json!("auto"), json!(true)],
    ),
  ];

  let expected_properties: BTreeSet<_> = cases.iter().map(|(name, _, _)| *name).collect();
  let actual_properties: BTreeSet<_> = properties.keys().map(String::as_str).collect();
  assert_eq!(actual_properties, expected_properties);
  assert!(properties["lineWidth"].get("default").is_none());
  assert!(properties["lineEnding"].get("default").is_none());

  // dprint-core intentionally coerces primitive strings. The published schema
  // remains strict so editor validation distinguishes JSON numbers from text.
  for (name, value) in [
    ("lineWidth", json!("80")),
    ("indentWidth", json!("4")),
    ("space", json!("4")),
    ("align", json!("16")),
    ("maxAuthors", json!("3")),
    ("wrap", json!("80")),
  ] {
    assert!(
      !validator.is_valid(&single_property(name, value)),
      "schema accepted a string where {name} requires a number"
    );
  }

  for (name, valid_values, invalid_values) in cases {
    for value in valid_values {
      let config = single_property(name, value.clone());
      assert!(
        validator.is_valid(&config),
        "schema rejected valid {name} configuration"
      );
      assert!(
        resolve_config(config_key_map(config), &GlobalConfiguration::default())
          .diagnostics
          .is_empty(),
        "resolver rejected valid {name} configuration"
      );
    }

    for value in invalid_values {
      let config = single_property(name, value.clone());
      assert!(
        !validator.is_valid(&config),
        "schema accepted invalid {name} configuration"
      );
      assert!(
        !resolve_config(config_key_map(config), &GlobalConfiguration::default())
          .diagnostics
          .is_empty(),
        "resolver accepted invalid {name} configuration: {value:?}"
      );
    }
  }
}

fn format_with_resolved_config(
  input: &str,
  plugin_config: serde_json::Value,
  global_config: GlobalConfiguration,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
  let config = resolve_plugin_config(plugin_config, global_config);
  format_text(Path::new("file.bib"), input, &config)
}

fn resolve_plugin_config(plugin_config: serde_json::Value, global_config: GlobalConfiguration) -> Configuration {
  let result = resolve_config(config_key_map(plugin_config), &global_config);
  ensure_no_diagnostics(&result.diagnostics);
  result.config
}

fn config_key_map(value: serde_json::Value) -> ConfigKeyMap {
  serde_json::from_value(value).expect("test configuration must be a dprint config map")
}

fn single_property(name: &str, value: serde_json::Value) -> serde_json::Value {
  let mut properties = serde_json::Map::new();
  properties.insert(name.to_string(), value);
  serde_json::Value::Object(properties)
}

fn schema() -> serde_json::Value {
  let schema_bytes = include_bytes!("../deployment/schema.json");
  serde_json::from_slice(schema_bytes).expect("schema.json must be valid JSON")
}

fn schema_properties(schema: &serde_json::Value) -> &serde_json::Map<String, serde_json::Value> {
  schema
    .get("properties")
    .expect("schema properties")
    .as_object()
    .expect("schema properties must be an object")
}
