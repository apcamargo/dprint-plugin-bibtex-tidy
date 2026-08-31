use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::ConfigKeyValue;
use dprint_core::configuration::ConfigurationDiagnostic;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::configuration::get_nullable_value;
use dprint_core::configuration::get_unknown_property_diagnostics;

use crate::configuration::types::*;

pub fn resolve_config(
  mut config: ConfigKeyMap,
  global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();

  // Line width
  let mut line_width =
    get_nullable_value::<u32>(&mut config, "lineWidth", &mut diagnostics).or(global_config.line_width);
  if line_width == Some(0) {
    diagnostics.push(ConfigurationDiagnostic {
      property_name: "lineWidth".to_string(),
      message: "lineWidth must be greater than 0.".to_string(),
    });
    line_width = None;
  }

  // Indent width
  let indent_width =
    get_nullable_value::<u8>(&mut config, "indentWidth", &mut diagnostics).or(global_config.indent_width);

  // New line kind - plugin newLineKind overrides global, matching lineWidth/indentWidth/useTabs. Supports auto/lf/crlf/system.
  let raw_new_line_kind =
    get_nullable_value::<dprint_core::configuration::RawNewLineKind>(&mut config, "newLineKind", &mut diagnostics);
  let local_new_line_kind = raw_new_line_kind.map(|kind| match kind {
    dprint_core::configuration::RawNewLineKind::Auto => dprint_core::configuration::NewLineKind::Auto,
    dprint_core::configuration::RawNewLineKind::LineFeed => dprint_core::configuration::NewLineKind::LineFeed,
    dprint_core::configuration::RawNewLineKind::CarriageReturnLineFeed => {
      dprint_core::configuration::NewLineKind::CarriageReturnLineFeed
    }
    dprint_core::configuration::RawNewLineKind::System => {
      if cfg!(windows) {
        dprint_core::configuration::NewLineKind::CarriageReturnLineFeed
      } else {
        dprint_core::configuration::NewLineKind::LineFeed
      }
    }
  });
  let new_line_kind = local_new_line_kind.or(global_config.new_line_kind);

  // Use tabs
  let use_tabs = get_nullable_value::<bool>(&mut config, "useTabs", &mut diagnostics).or(global_config.use_tabs);

  // Align
  let align = resolve_align(&mut config, &mut diagnostics);

  // Boolean flags
  let blank_lines = get_nullable_value::<bool>(&mut config, "blankLines", &mut diagnostics);
  let curly = get_nullable_value::<bool>(&mut config, "curly", &mut diagnostics);
  let numeric = get_nullable_value::<bool>(&mut config, "numeric", &mut diagnostics);
  let months = get_nullable_value::<bool>(&mut config, "months", &mut diagnostics);

  // Sort entries
  let sort = resolve_field_list_or_bool(&mut config, "sort", &mut diagnostics);

  // Duplicates
  let duplicates = resolve_duplicates(&mut config, &mut diagnostics);

  // Merge
  let merge = resolve_merge(&mut config, &mut diagnostics);

  // More boolean flags
  let strip_enclosing_braces = get_nullable_value::<bool>(&mut config, "stripEnclosingBraces", &mut diagnostics);
  let drop_all_caps = get_nullable_value::<bool>(&mut config, "dropAllCaps", &mut diagnostics);

  // Escape
  let escape = resolve_escape(&mut config, &mut diagnostics);

  let unescape = get_nullable_value::<bool>(&mut config, "unescape", &mut diagnostics);

  // Sort fields
  let sort_fields = resolve_field_list_or_bool(&mut config, "sortFields", &mut diagnostics);

  let strip_comments = get_nullable_value::<bool>(&mut config, "stripComments", &mut diagnostics);
  let trailing_commas = get_nullable_value::<bool>(&mut config, "trailingCommas", &mut diagnostics);
  let encode_urls = get_nullable_value::<bool>(&mut config, "encodeUrls", &mut diagnostics);
  let tidy_comments = get_nullable_value::<bool>(&mut config, "tidyComments", &mut diagnostics);
  let remove_empty_fields = get_nullable_value::<bool>(&mut config, "removeEmptyFields", &mut diagnostics);
  let remove_duplicate_fields = get_nullable_value::<bool>(&mut config, "removeDuplicateFields", &mut diagnostics);

  // Generate keys
  let generate_keys = resolve_generate_keys(&mut config, &mut diagnostics);

  // Max authors
  let mut max_authors = get_nullable_value::<u32>(&mut config, "maxAuthors", &mut diagnostics);
  if max_authors == Some(0) {
    diagnostics.push(ConfigurationDiagnostic {
      property_name: "maxAuthors".to_string(),
      message: "maxAuthors must be greater than 0.".to_string(),
    });
    max_authors = None;
  }

  let lowercase = get_nullable_value::<bool>(&mut config, "lowercase", &mut diagnostics);
  let enclosing_braces = resolve_field_list_or_bool(&mut config, "enclosingBraces", &mut diagnostics);
  let remove_braces = resolve_field_list_or_bool(&mut config, "removeBraces", &mut diagnostics);

  // Wrap
  let wrap = resolve_wrap(&mut config, &mut diagnostics);

  // Omit
  let omit = resolve_string_array(&mut config, "omit", &mut diagnostics);

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: Configuration {
      line_width,
      indent_width,
      new_line_kind,
      use_tabs,
      align,
      blank_lines,
      curly,
      numeric,
      months,
      sort,
      duplicates,
      merge,
      strip_enclosing_braces,
      drop_all_caps,
      escape,
      unescape,
      sort_fields,
      strip_comments,
      trailing_commas,
      encode_urls,
      tidy_comments,
      remove_empty_fields,
      remove_duplicate_fields,
      generate_keys,
      max_authors,
      lowercase,
      enclosing_braces,
      remove_braces,
      wrap,
      omit,
    },
    diagnostics,
  }
}

fn resolve_align(config: &mut ConfigKeyMap, diagnostics: &mut Vec<ConfigurationDiagnostic>) -> Option<AlignOption> {
  match config.shift_remove("align") {
    Some(ConfigKeyValue::Bool(b)) => Some(AlignOption::Bool(b)),
    Some(ConfigKeyValue::Number(n)) => {
      if n < 0 {
        diagnostics.push(ConfigurationDiagnostic {
          property_name: "align".to_string(),
          message: "align must be a non-negative integer or boolean.".to_string(),
        });
        None
      } else {
        Some(AlignOption::Column(n as u32))
      }
    }
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "align".to_string(),
        message: "Expected a boolean or integer for align.".to_string(),
      });
      None
    }
    None => None,
  }
}

fn resolve_wrap(config: &mut ConfigKeyMap, diagnostics: &mut Vec<ConfigurationDiagnostic>) -> Option<bool> {
  match config.shift_remove("wrap") {
    Some(ConfigKeyValue::Bool(b)) => Some(b),
    Some(ConfigKeyValue::Number(_)) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "wrap".to_string(),
        message: "wrap must be a boolean.".to_string(),
      });
      None
    }
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "wrap".to_string(),
        message: "Expected a boolean for wrap.".to_string(),
      });
      None
    }
    None => None,
  }
}

fn resolve_field_list_or_bool(
  config: &mut ConfigKeyMap,
  name: &str,
  diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> Option<FieldListOrBool> {
  match config.shift_remove(name) {
    Some(ConfigKeyValue::Bool(b)) => Some(FieldListOrBool::Bool(b)),
    Some(ConfigKeyValue::Array(arr)) => {
      let mut fields = Vec::with_capacity(arr.len());
      for item in arr {
        if let ConfigKeyValue::String(s) = item {
          fields.push(s);
        } else {
          diagnostics.push(ConfigurationDiagnostic {
            property_name: name.to_string(),
            message: format!("Expected array of strings for {name}."),
          });
          return None;
        }
      }
      Some(FieldListOrBool::List(fields))
    }
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: name.to_string(),
        message: format!("Expected a boolean or array of strings for {name}."),
      });
      None
    }
    None => None,
  }
}

fn resolve_duplicates(
  config: &mut ConfigKeyMap,
  diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> Option<DuplicatesOption> {
  match config.shift_remove("duplicates") {
    Some(ConfigKeyValue::Bool(b)) => Some(DuplicatesOption::Bool(b)),
    Some(ConfigKeyValue::Array(arr)) => {
      let mut rules = Vec::with_capacity(arr.len());
      for item in arr {
        if let ConfigKeyValue::String(s) = item {
          match s.as_str() {
            "doi" => rules.push(DuplicateRule::Doi),
            "key" => rules.push(DuplicateRule::Key),
            "abstract" => rules.push(DuplicateRule::Abstract),
            "citation" => rules.push(DuplicateRule::Citation),
            _ => {
              diagnostics.push(ConfigurationDiagnostic {
                property_name: "duplicates".to_string(),
                message: format!("Invalid duplicate rule '{s}'. Expected 'doi', 'key', 'abstract', or 'citation'."),
              });
              return None;
            }
          }
        } else {
          diagnostics.push(ConfigurationDiagnostic {
            property_name: "duplicates".to_string(),
            message: "Expected array of strings for duplicates.".to_string(),
          });
          return None;
        }
      }
      Some(DuplicatesOption::Rules(rules))
    }
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "duplicates".to_string(),
        message: "Expected a boolean or array of strings for duplicates.".to_string(),
      });
      None
    }
    None => None,
  }
}

fn resolve_merge(config: &mut ConfigKeyMap, diagnostics: &mut Vec<ConfigurationDiagnostic>) -> Option<MergeOption> {
  match config.shift_remove("merge") {
    Some(ConfigKeyValue::Bool(b)) => Some(MergeOption::Bool(b)),
    Some(ConfigKeyValue::String(s)) => match s.as_str() {
      "first" => Some(MergeOption::Strategy(MergeStrategy::First)),
      "last" => Some(MergeOption::Strategy(MergeStrategy::Last)),
      "combine" => Some(MergeOption::Strategy(MergeStrategy::Combine)),
      "overwrite" => Some(MergeOption::Strategy(MergeStrategy::Overwrite)),
      _ => {
        diagnostics.push(ConfigurationDiagnostic {
          property_name: "merge".to_string(),
          message: format!("Invalid merge strategy '{s}'. Expected 'first', 'last', 'combine', or 'overwrite'."),
        });
        None
      }
    },
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "merge".to_string(),
        message: "Expected a boolean or string for merge.".to_string(),
      });
      None
    }
    None => None,
  }
}

fn resolve_escape(config: &mut ConfigKeyMap, diagnostics: &mut Vec<ConfigurationDiagnostic>) -> Option<EscapeOption> {
  match config.shift_remove("escape") {
    Some(ConfigKeyValue::Bool(b)) => Some(EscapeOption::Bool(b)),
    Some(ConfigKeyValue::String(s)) => {
      if s == "new" {
        Some(EscapeOption::Mode(s))
      } else {
        diagnostics.push(ConfigurationDiagnostic {
          property_name: "escape".to_string(),
          message: format!("Invalid escape option '{s}'. Expected true, false, or 'new'."),
        });
        None
      }
    }
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "escape".to_string(),
        message: "Expected a boolean or 'new' for escape.".to_string(),
      });
      None
    }
    None => None,
  }
}

fn resolve_generate_keys(
  config: &mut ConfigKeyMap,
  diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> Option<GenerateKeysOption> {
  match config.shift_remove("generateKeys") {
    Some(ConfigKeyValue::Bool(b)) => Some(GenerateKeysOption::Bool(b)),
    Some(ConfigKeyValue::String(s)) => Some(GenerateKeysOption::Template(s)),
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: "generateKeys".to_string(),
        message: "Expected a boolean or string template for generateKeys.".to_string(),
      });
      None
    }
    None => None,
  }
}

fn resolve_string_array(
  config: &mut ConfigKeyMap,
  name: &str,
  diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> Option<Vec<String>> {
  match config.shift_remove(name) {
    Some(ConfigKeyValue::Array(arr)) => {
      let mut items = Vec::with_capacity(arr.len());
      for item in arr {
        if let ConfigKeyValue::String(s) = item {
          items.push(s);
        } else {
          diagnostics.push(ConfigurationDiagnostic {
            property_name: name.to_string(),
            message: format!("Expected array of strings for {name}."),
          });
          return None;
        }
      }
      Some(items)
    }
    Some(_) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: name.to_string(),
        message: format!("Expected array of strings for {name}."),
      });
      None
    }
    None => None,
  }
}
