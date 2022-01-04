use opentelemetry::propagation::Extractor;
use std::collections::HashMap;

pub struct EnvVarExtractor(pub HashMap<String, String>);

impl<'a> Extractor for EnvVarExtractor {
  /// Get a value for a key from the Env.  If the value is not valid ASCII, returns None.
  fn get(&self, key: &str) -> Option<&str> {
    if let Some(val) = self.0.get(&key.to_string()) {
      Some(val.as_str())
    } else {
      Some("")
    }
  }

  /// Collect all the keys from the HeaderMap.
  fn keys(&self) -> Vec<&str> {
    self.0.keys()
          .map(|key| key.as_str())
          .collect::<Vec<_>>()
  }
}