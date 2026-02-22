use dprint_core::configuration::NewLineKind;
use serde::Deserialize;
use serde::Serialize;

/// Controls spacing before trailing comments.
#[derive(Clone, Debug, PartialEq)]
pub enum CommentSpacesBefore {
  /// Don't modify spacing — upstream behavior (1 space).
  Disabled,
  /// Fixed number of spaces before trailing comments.
  Fixed(u32),
  /// Smart alignment: align trailing comments within consecutive entry groups.
  Smart { min: u32 },
}

impl Default for CommentSpacesBefore {
  fn default() -> Self {
    CommentSpacesBefore::Disabled
  }
}

impl Serialize for CommentSpacesBefore {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      CommentSpacesBefore::Disabled => serializer.serialize_bool(false),
      CommentSpacesBefore::Fixed(n) => serializer.serialize_u32(*n),
      CommentSpacesBefore::Smart { min } => {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("smart", &true)?;
        map.serialize_entry("min", min)?;
        map.end()
      }
    }
  }
}

impl<'de> Deserialize<'de> for CommentSpacesBefore {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    use serde::de;

    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = CommentSpacesBefore;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("false, a positive integer, \"smart\", or { \"smart\": true, \"min\": N }")
      }

      fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        if v {
          Err(E::custom("use a number or \"smart\" instead of true"))
        } else {
          Ok(CommentSpacesBefore::Disabled)
        }
      }

      fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        if v >= 1 {
          Ok(CommentSpacesBefore::Fixed(v as u32))
        } else {
          Err(E::custom(format!("expected a positive number, got {}", v)))
        }
      }

      fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        if v >= 1 {
          Ok(CommentSpacesBefore::Fixed(v as u32))
        } else {
          Err(E::custom("expected a positive number, got 0"))
        }
      }

      fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if v == "smart" {
          Ok(CommentSpacesBefore::Smart { min: 1 })
        } else {
          Err(E::custom(format!("expected \"smart\", got \"{}\"", v)))
        }
      }

      fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut min: Option<u32> = None;
        while let Some(key) = map.next_key::<String>()? {
          match key.as_str() {
            "smart" => {
              let _: bool = map.next_value()?;
            }
            "min" => {
              min = Some(map.next_value()?);
            }
            other => return Err(de::Error::unknown_field(other, &["smart", "min"])),
          }
        }
        Ok(CommentSpacesBefore::Smart { min: min.unwrap_or(1) })
      }
    }

    deserializer.deserialize_any(Visitor)
  }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub line_width: u32,
  pub use_tabs: bool,
  pub indent_width: u8,
  pub new_line_kind: NewLineKind,
  pub comment_force_leading_space: bool,
  pub comment_spaces_before: CommentSpacesBefore,
  pub cargo_apply_conventions: bool,
}
