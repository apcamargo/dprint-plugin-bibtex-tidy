use crate::configuration::Configuration;
use crate::configuration::resolve_config;
use crate::engine::BibtexTidyEngine;

use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::CheckConfigUpdatesMessage;
use dprint_core::plugins::ConfigChange;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatError;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncFormatRequest;
use dprint_core::plugins::SyncHostFormatRequest;
use dprint_core::plugins::SyncPluginHandler;

struct BibTeXPluginHandler {
  engine: Option<BibtexTidyEngine>,
}

impl SyncPluginHandler<Configuration> for BibTeXPluginHandler {
  fn resolve_config(
    &mut self,
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
  ) -> PluginResolveConfigurationResult<Configuration> {
    let result = resolve_config(config, global_config);
    PluginResolveConfigurationResult {
      config: result.config,
      diagnostics: result.diagnostics,
      file_matching: FileMatchingInfo {
        file_extensions: vec!["bib".to_string(), "bibtex".to_string()],
        file_names: vec![],
      },
    }
  }

  fn check_config_updates(&self, _message: CheckConfigUpdatesMessage) -> Result<Vec<ConfigChange>, FormatError> {
    Ok(Vec::new())
  }

  fn plugin_info(&mut self) -> PluginInfo {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    PluginInfo {
      name: env!("CARGO_PKG_NAME").to_string(),
      version: VERSION.to_string(),
      config_key: "bibtex-tidy".to_string(),
      help_url: "https://github.com/apcamargo/dprint-plugin-bibtex-tidy#readme".to_string(),
      config_schema_url: format!(
        "https://github.com/apcamargo/dprint-plugin-bibtex-tidy/releases/download/{VERSION}/schema.json"
      ),
      update_url: None,
    }
  }

  fn license_text(&mut self) -> String {
    format!(
      "{}\n\n{}",
      include_str!("../LICENSE"),
      include_str!("../THIRD_PARTY_NOTICES.md"),
    )
  }

  fn format(
    &mut self,
    request: SyncFormatRequest<Configuration>,
    _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
  ) -> FormatResult {
    if request.range.is_some() {
      return Ok(None); // range formatting is not supported
    }

    let text =
      std::str::from_utf8(&request.file_bytes).map_err(|err| format!("Failed to decode file as UTF-8: {err}"))?;
    if self.engine.is_none() {
      self.engine = Some(BibtexTidyEngine::new()?);
    }
    let engine = self.engine.as_mut().expect("engine was initialized above");
    let maybe_text = crate::format_text::format_text_with_engine(engine, request.file_path, text, request.config)?;
    Ok(maybe_text.map(|t| t.into_bytes()))
  }
}

generate_plugin_code!(BibTeXPluginHandler, BibTeXPluginHandler { engine: None });
