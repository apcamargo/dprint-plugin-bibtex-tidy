use boa_engine::{Context, JsValue, Source, js_string};

const BIBTEX_TIDY_BUNDLE: &str = include_str!("bibtex_tidy_bundle.js");

#[derive(Debug)]
struct EngineError(String);

impl std::fmt::Display for EngineError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for EngineError {}

/// A loaded instance of the bundled bibtex-tidy JavaScript engine.
///
/// Boa contexts are not `Send`, so the synchronous WASM plugin handler owns one
/// and reuses it only on its formatting thread.
pub(crate) struct BibtexTidyEngine {
  context: Context,
}

impl BibtexTidyEngine {
  pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    let mut context = Context::default();
    context
      .eval(Source::from_bytes(BIBTEX_TIDY_BUNDLE))
      .map_err(|e| EngineError(format!("Failed to evaluate bundle: {e}")))?;
    Ok(Self { context })
  }

  pub(crate) fn format(
    &mut self,
    input: &str,
    options_json: &str,
  ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let formatter = self
      .context
      .global_object()
      .get(js_string!("__bibtexTidyFormat"), &mut self.context)
      .map_err(|e| EngineError(format!("Failed to access bibtex-tidy formatter: {e}")))?
      .as_object()
      .ok_or_else(|| EngineError("bibtex-tidy formatter is not callable".to_string()))?;

    let result = formatter
      .call(
        &JsValue::undefined(),
        &[
          JsValue::from(js_string!(input)),
          JsValue::from(js_string!(options_json)),
        ],
        &mut self.context,
      )
      .map_err(|e| EngineError(format!("JavaScript execution error: {e}")))?;

    let json_output = result
      .to_string(&mut self.context)
      .map_err(|e| EngineError(format!("Failed to convert JS result to string: {e}")))?
      .to_std_string_escaped();

    #[derive(serde::Deserialize)]
    struct TidyOutput {
      bibtex: String,
      #[allow(dead_code)]
      warnings: Vec<serde_json::Value>,
    }

    let parsed: TidyOutput = serde_json::from_str(&json_output).map_err(|e| {
      EngineError(format!(
        "Failed to parse bibtex-tidy output: {e} (output: {json_output})"
      ))
    })?;
    Ok(parsed.bibtex)
  }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[unsafe(no_mangle)]
/// Fills `dest` for the custom `getrandom` backend used by the Wasm build.
///
/// The dprint Wasm host does not expose an entropy source, so this backend
/// intentionally writes a deterministic byte sequence. It must not be used on
/// a path that requires cryptographic randomness.
///
/// # Safety
///
/// `dest` must reference a writable buffer of `len` bytes. This function is
/// called only by `getrandom`, which upholds that requirement.
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), getrandom::Error> {
  // SAFETY: getrandom calls this symbol with a valid, writable buffer of `len`
  // bytes. `u8` permits every bit pattern and the loop writes only that range.
  let slice = unsafe { std::slice::from_raw_parts_mut(dest, len) };
  for (i, b) in slice.iter_mut().enumerate() {
    *b = (i & 0xFF) as u8;
  }
  Ok(())
}
