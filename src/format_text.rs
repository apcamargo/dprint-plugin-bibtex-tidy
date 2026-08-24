use std::path::Path;

use crate::configuration::*;
use crate::engine::BibtexTidyEngine;

type FormattingError = Box<dyn std::error::Error + Send + Sync>;

/// Formats BibTeX source text according to the provided configuration.
///
/// Returns `Ok(None)` when the text is already formatted.
pub fn format_text(
  file_path: &Path,
  input_text: &str,
  config: &Configuration,
) -> Result<Option<String>, FormattingError> {
  let mut engine = BibtexTidyEngine::new()?;
  format_text_with_engine(&mut engine, file_path, input_text, config)
}

/// Formats BibTeX source with an already initialized JavaScript engine.
pub(crate) fn format_text_with_engine(
  engine: &mut BibtexTidyEngine,
  _file_path: &Path,
  input_text: &str,
  config: &Configuration,
) -> Result<Option<String>, FormattingError> {
  let options_json = to_bibtex_tidy_options_json(config)?;
  let formatted = engine.format(input_text, &options_json)?;
  let formatted = apply_line_ending(formatted, input_text, config);

  if formatted == input_text {
    Ok(None)
  } else {
    Ok(Some(formatted))
  }
}

fn to_bibtex_tidy_options_json(config: &Configuration) -> Result<String, serde_json::Error> {
  let mut map = serde_json::Map::new();

  // Tab & Space / Indentation
  if let Some(tab) = config.tab.or(config.use_tabs) {
    map.insert("tab".to_string(), serde_json::Value::Bool(tab));
  }
  if let Some(space) = config.space.or(config.indent_width) {
    map.insert("space".to_string(), serde_json::json!(space));
  }

  // Align
  if let Some(align) = &config.align {
    match align {
      AlignOption::Bool(b) => {
        map.insert("align".to_string(), serde_json::Value::Bool(*b));
      }
      AlignOption::Column(c) => {
        map.insert("align".to_string(), serde_json::json!(c));
      }
    }
  }

  // Boolean flags
  if let Some(v) = config.blank_lines {
    map.insert("blankLines".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.curly {
    map.insert("curly".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.numeric {
    map.insert("numeric".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.months {
    map.insert("months".to_string(), serde_json::Value::Bool(v));
  }

  // Sort
  if let Some(sort) = &config.sort {
    match sort {
      FieldListOrBool::Bool(b) => {
        map.insert("sort".to_string(), serde_json::Value::Bool(*b));
      }
      FieldListOrBool::List(l) => {
        map.insert("sort".to_string(), serde_json::json!(l));
      }
    }
  }

  // Duplicates
  if let Some(dupes) = &config.duplicates {
    match dupes {
      DuplicatesOption::Bool(b) => {
        map.insert("duplicates".to_string(), serde_json::Value::Bool(*b));
      }
      DuplicatesOption::Rules(rules) => {
        map.insert("duplicates".to_string(), serde_json::json!(rules));
      }
    }
  }

  // Merge
  if let Some(merge) = &config.merge {
    match merge {
      MergeOption::Bool(b) => {
        map.insert("merge".to_string(), serde_json::Value::Bool(*b));
      }
      MergeOption::Strategy(s) => {
        map.insert("merge".to_string(), serde_json::json!(s));
      }
    }
  }

  if let Some(v) = config.strip_enclosing_braces {
    map.insert("stripEnclosingBraces".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.drop_all_caps {
    map.insert("dropAllCaps".to_string(), serde_json::Value::Bool(v));
  }

  // Escape
  if let Some(escape) = &config.escape {
    match escape {
      EscapeOption::Bool(b) => {
        map.insert("escape".to_string(), serde_json::Value::Bool(*b));
      }
      EscapeOption::Mode(m) => {
        map.insert("escape".to_string(), serde_json::Value::String(m.clone()));
      }
    }
  }

  if let Some(v) = config.unescape {
    map.insert("unescape".to_string(), serde_json::Value::Bool(v));
  }

  // Sort fields
  if let Some(sf) = &config.sort_fields {
    match sf {
      FieldListOrBool::Bool(b) => {
        map.insert("sortFields".to_string(), serde_json::Value::Bool(*b));
      }
      FieldListOrBool::List(l) => {
        map.insert("sortFields".to_string(), serde_json::json!(l));
      }
    }
  }

  if let Some(v) = config.strip_comments {
    map.insert("stripComments".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.trailing_commas {
    map.insert("trailingCommas".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.encode_urls {
    map.insert("encodeUrls".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.tidy_comments {
    map.insert("tidyComments".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.remove_empty_fields {
    map.insert("removeEmptyFields".to_string(), serde_json::Value::Bool(v));
  }
  if let Some(v) = config.remove_duplicate_fields {
    map.insert("removeDuplicateFields".to_string(), serde_json::Value::Bool(v));
  }

  // Generate keys
  if let Some(gk) = &config.generate_keys {
    match gk {
      GenerateKeysOption::Bool(b) => {
        map.insert("generateKeys".to_string(), serde_json::Value::Bool(*b));
      }
      GenerateKeysOption::Template(t) => {
        map.insert("generateKeys".to_string(), serde_json::Value::String(t.clone()));
      }
    }
  }

  if let Some(v) = config.max_authors {
    map.insert("maxAuthors".to_string(), serde_json::json!(v));
  }
  if let Some(v) = config.lowercase {
    map.insert("lowercase".to_string(), serde_json::Value::Bool(v));
  }

  if let Some(eb) = &config.enclosing_braces {
    match eb {
      FieldListOrBool::Bool(b) => {
        map.insert("enclosingBraces".to_string(), serde_json::Value::Bool(*b));
      }
      FieldListOrBool::List(l) => {
        map.insert("enclosingBraces".to_string(), serde_json::json!(l));
      }
    }
  }

  if let Some(rb) = &config.remove_braces {
    match rb {
      FieldListOrBool::Bool(b) => {
        map.insert("removeBraces".to_string(), serde_json::Value::Bool(*b));
      }
      FieldListOrBool::List(l) => {
        map.insert("removeBraces".to_string(), serde_json::json!(l));
      }
    }
  }

  // Wrap / LineWidth
  if let Some(wrap) = &config.wrap {
    match wrap {
      WrapOption::Bool(b) => {
        map.insert("wrap".to_string(), serde_json::Value::Bool(*b));
      }
      WrapOption::Column(c) => {
        map.insert("wrap".to_string(), serde_json::json!(c));
      }
    }
  } else if let Some(line_width) = config.line_width {
    map.insert("wrap".to_string(), serde_json::json!(line_width));
  }

  if let Some(omit) = &config.omit {
    map.insert("omit".to_string(), serde_json::json!(omit));
  }

  serde_json::to_string(&map)
}

fn apply_line_ending(text: String, input_text: &str, config: &Configuration) -> String {
  use dprint_core::configuration::{NewLineKind, resolve_new_line_kind};

  let new_line_kind = match config.line_ending {
    Some(LineEnding::Lf) => NewLineKind::LineFeed,
    Some(LineEnding::Crlf) => NewLineKind::CarriageReturnLineFeed,
    None => config.new_line_kind.unwrap_or(NewLineKind::LineFeed),
  };
  let line_ending = resolve_new_line_kind(input_text, new_line_kind);
  text.replace("\r\n", "\n").replace('\n', line_ending)
}
