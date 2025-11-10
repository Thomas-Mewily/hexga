use super::*;

impl<T> Serialize for Asset<T>
where
    T: Async
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.path().serialize(serializer)
    }
}


impl<'de, T> Deserialize<'de> for Asset<T>
where
    T: Async + Load,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = Path::deserialize(deserializer)?;
        Ok(Self::load(path))
    }
}