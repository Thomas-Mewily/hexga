use super::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization




impl<C,Idx> Serialize for ImageBaseOf<C,Idx>
    where
    Idx : Integer + Serialize,
    C : Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>> + Serialize,
    u8: CastRangeFrom<C::Component>,
    u16: CastRangeFrom<C::Component>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_with_encoding(self)
    }
}


impl<'de, C, Idx> Deserialize<'de> for ImageBaseOf<C, Idx>
    where
        Idx: Integer + Deserialize<'de>,
        C: Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>> + Deserialize<'de>,
        u8: CastRangeFrom<C::Component>,
        u16: CastRangeFrom<C::Component>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_with_encoding::<Self>()
    }
}