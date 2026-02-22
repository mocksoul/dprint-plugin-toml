use super::configuration::CommentSpacesBefore;
use super::Configuration;
use dprint_core::configuration::*;

/// Resolves configuration from a collection of key value strings.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use dprint_core::configuration::ConfigKeyMap;
/// use dprint_core::configuration::resolve_global_config;
/// use dprint_plugin_toml::configuration::resolve_config;
///
/// let mut config_map = ConfigKeyMap::new(); // get a collection of key value pairs from somewhere
/// let global_config_result = resolve_global_config(&mut config_map);
///
/// // check global_config_result.diagnostics here...
///
/// let config_map = ConfigKeyMap::new(); // get a collection of k/v pairs from somewhere
/// let config_result = resolve_config(
///     config_map,
///     &global_config_result.config
/// );
///
/// // check config_result.diagnostics here and use config_result.config
/// ```
pub fn resolve_config(config: ConfigKeyMap, global_config: &GlobalConfiguration) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;

  // Manually extract comment.spacesBefore since it accepts multiple types
  let comment_spaces_before = resolve_comment_spaces_before(&mut config, &mut diagnostics);

  let resolved_config = Configuration {
    line_width: get_value(
      &mut config,
      "lineWidth",
      global_config.line_width.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.line_width),
      &mut diagnostics,
    ),
    use_tabs: get_value(
      &mut config,
      "useTabs",
      global_config.use_tabs.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.use_tabs),
      &mut diagnostics,
    ),
    indent_width: get_value(&mut config, "indentWidth", global_config.indent_width.unwrap_or(2), &mut diagnostics),
    new_line_kind: get_value(
      &mut config,
      "newLineKind",
      global_config.new_line_kind.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.new_line_kind),
      &mut diagnostics,
    ),
    comment_force_leading_space: get_value(&mut config, "comment.forceLeadingSpace", true, &mut diagnostics),
    comment_spaces_before,
    cargo_apply_conventions: get_value(&mut config, "cargo.applyConventions", true, &mut diagnostics),
  };

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  }
}

fn resolve_comment_spaces_before(config: &mut ConfigKeyMap, diagnostics: &mut Vec<ConfigurationDiagnostic>) -> CommentSpacesBefore {
  let key = "comment.spacesBefore";
  match config.shift_remove(key) {
    None => CommentSpacesBefore::Disabled,
    Some(ConfigKeyValue::Bool(false)) | Some(ConfigKeyValue::Null) => CommentSpacesBefore::Disabled,
    Some(ConfigKeyValue::Bool(true)) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: key.to_string(),
        message: "Use a number or \"smart\" instead of true".to_string(),
      });
      CommentSpacesBefore::Disabled
    }
    Some(ConfigKeyValue::Number(n)) if n >= 1 => CommentSpacesBefore::Fixed(n as u32),
    Some(ConfigKeyValue::Number(n)) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: key.to_string(),
        message: format!("Expected a positive number, got {}", n),
      });
      CommentSpacesBefore::Disabled
    }
    Some(ConfigKeyValue::String(ref s)) if s == "smart" => CommentSpacesBefore::Smart { min: 1 },
    Some(ConfigKeyValue::String(s)) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: key.to_string(),
        message: format!("Expected false, a number, or \"smart\", got \"{}\"", s),
      });
      CommentSpacesBefore::Disabled
    }
    Some(ConfigKeyValue::Object(obj)) => {
      let min = obj
        .get("min")
        .and_then(|v| match v {
          ConfigKeyValue::Number(n) => Some(*n as u32),
          _ => None,
        })
        .unwrap_or(1);
      CommentSpacesBefore::Smart { min }
    }
    Some(ConfigKeyValue::Array(_)) => {
      diagnostics.push(ConfigurationDiagnostic {
        property_name: key.to_string(),
        message: "Expected false, a number, or \"smart\"".to_string(),
      });
      CommentSpacesBefore::Disabled
    }
  }
}
