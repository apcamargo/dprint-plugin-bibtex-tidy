use dprint_core::configuration::NewLineKind;
use dprint_core::configuration::ParseConfigurationError;
use dprint_core::generate_str_to_from;
use serde::{Deserialize, Serialize};

/// Duplicate detection rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DuplicateRule {
  Doi,
  Key,
  Abstract,
  Citation,
}

generate_str_to_from![
  DuplicateRule,
  [Doi, "doi"],
  [Key, "key"],
  [Abstract, "abstract"],
  [Citation, "citation"]
];

/// Merge strategy for duplicate entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeStrategy {
  First,
  Last,
  Combine,
  Overwrite,
}

generate_str_to_from![
  MergeStrategy,
  [First, "first"],
  [Last, "last"],
  [Combine, "combine"],
  [Overwrite, "overwrite"]
];

/// Options for escaping special characters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EscapeOption {
  Bool(bool),
  Mode(String),
}

/// Options for fields that can be boolean or a list of field names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldListOrBool {
  Bool(bool),
  List(Vec<String>),
}

/// Options for aligning field values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlignOption {
  Bool(bool),
  Column(u32),
}

/// Options for duplicates check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DuplicatesOption {
  Bool(bool),
  Rules(Vec<DuplicateRule>),
}

/// Options for duplicate merging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MergeOption {
  Bool(bool),
  Strategy(MergeStrategy),
}

/// Options for generating citation keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateKeysOption {
  Bool(bool),
  Template(String),
}

/// Configuration for the BibTeX formatter.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub line_width: Option<u32>,
  pub indent_width: Option<u8>,
  /// Effective newline kind (plugin `newLineKind` overrides global).
  pub new_line_kind: Option<NewLineKind>,
  pub use_tabs: Option<bool>,
  pub align: Option<AlignOption>,
  pub blank_lines: Option<bool>,
  pub curly: Option<bool>,
  pub numeric: Option<bool>,
  pub months: Option<bool>,
  pub sort: Option<FieldListOrBool>,
  pub duplicates: Option<DuplicatesOption>,
  pub merge: Option<MergeOption>,
  pub strip_enclosing_braces: Option<bool>,
  pub drop_all_caps: Option<bool>,
  pub escape: Option<EscapeOption>,
  pub unescape: Option<bool>,
  pub sort_fields: Option<FieldListOrBool>,
  pub strip_comments: Option<bool>,
  pub trailing_commas: Option<bool>,
  pub encode_urls: Option<bool>,
  pub tidy_comments: Option<bool>,
  pub remove_empty_fields: Option<bool>,
  pub remove_duplicate_fields: Option<bool>,
  pub generate_keys: Option<GenerateKeysOption>,
  pub max_authors: Option<u32>,
  pub lowercase: Option<bool>,
  pub enclosing_braces: Option<FieldListOrBool>,
  pub remove_braces: Option<FieldListOrBool>,
  pub wrap: Option<bool>,
  pub omit: Option<Vec<String>>,
}
