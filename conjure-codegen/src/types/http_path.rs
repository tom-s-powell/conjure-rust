use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct HttpPath(pub String);
impl std::fmt::Display for HttpPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for HttpPath {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for HttpPath {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl ser::Serialize for HttpPath {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for HttpPath {
    fn deserialize<D>(d: D) -> Result<HttpPath, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(HttpPath)
    }
}
