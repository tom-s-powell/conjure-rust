use crate::serde::ser::SerializeMap as SerializeMap_;
use crate::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OptionalExample {
    item: Option<String>,
}
impl OptionalExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn item(&self) -> Option<&str> {
        self.item.as_ref().map(|o| &**o)
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    item: Option<String>,
}
impl Builder {
    pub fn item<T>(&mut self, item: T) -> &mut Self
    where
        T: Into<Option<String>>,
    {
        self.item = item.into();
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> OptionalExample {
        OptionalExample {
            item: self.item.clone(),
        }
    }
}
impl From<OptionalExample> for Builder {
    #[inline]
    fn from(_v: OptionalExample) -> Builder {
        Builder { item: _v.item }
    }
}
impl ser::Serialize for OptionalExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_item = self.item.is_none();
        if !skip_item {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        if !skip_item {
            map.serialize_entry(&"item", &self.item)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for OptionalExample {
    fn deserialize<D_>(d: D_) -> Result<OptionalExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct("OptionalExample", &["item"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = OptionalExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<OptionalExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut item = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Item => item = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let item = match item {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(OptionalExample { item })
    }
}
enum Field_ {
    Item,
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D_>(d: D_) -> Result<Field_, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E_>(self, value: &str) -> Result<Field_, E_>
    where
        E_: de::Error,
    {
        let v = match value {
            "item" => Field_::Item,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}