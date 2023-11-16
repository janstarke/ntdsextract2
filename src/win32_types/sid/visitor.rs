use lazy_regex::regex_captures;
use serde::de::Visitor;

use super::Sid;

#[derive(Default)]
pub struct SIDVisitor {}

impl<'de> Visitor<'de> for SIDVisitor {
    type Value = Sid;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string looking like a Windows Security ID")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match regex_captures!(
            r#"^S-(?P<revision>\d+)-(?P<authority>\d+)-(?P<numbers>(?:-|\d+){1,1})$"#,
            v
        ) {
            None => Err(E::custom(format!("invalid SID: {v}"))),
            Some((_, revision, authority, numbers)) => {
                let revision = revision.parse::<u8>()
                    .map_err(|_| E::custom("unable to parse revision in '{revision}'"))?;
                let authority = authority.parse::<u64>()
                    .map_err(|_| E::custom("unable to parse authority in '{authority}'"))?;
                let mut vec_numbers = Vec::new();
                for r in numbers.split('_').map(|n| n.parse::<u32>()) {
                    match r {
                        Err(_) => return Err(E::custom("unable to parse number in '{numbers}'")),
                        Ok(n) => vec_numbers.push(n),
                    }
                }
                Ok(Sid::new(revision, authority, vec_numbers))
            }
        }
    }
}
