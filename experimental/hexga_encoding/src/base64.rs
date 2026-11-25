use super::*;
use ::base64::Engine;

pub trait ToBase64
{
    fn to_base64(&self) -> String { self.to_base64_in(String::with_capacity(1024)) }
    fn to_base64_in(&self, base64: String) -> String;
}
impl ToBase64 for [u8]
{
    fn to_base64_in(&self, mut base64: String) -> String
    {
        ::base64::engine::general_purpose::STANDARD.encode_string(self, &mut base64);
        base64
    }
}

pub type Base64Error = ::base64::DecodeError;
pub trait FromBase64: Sized
{
    fn from_base64<D>(data: D) -> EncodeResult<Self>
    where
        D: AsRef<[u8]>;
}
impl<A: Allocator + Default> FromBase64 for Vec<u8, A>
{
    fn from_base64<D>(data: D) -> EncodeResult<Self>
    where
        D: AsRef<[u8]>,
    {
        let data = data.as_ref();
        let capacity = ::base64::decoded_len_estimate(data.len());
        let mut decoded = Vec::with_capacity_in(capacity, A::default());
        decoded.resize(capacity, 0);
        let len = match ::base64::engine::general_purpose::STANDARD.decode_slice(data, &mut decoded)
        {
            Ok(len) => len,
            Err(::base64::DecodeSliceError::DecodeError(error)) => return Err(error.into()),
            Err(::base64::DecodeSliceError::OutputSliceTooSmall) => unreachable!("decoded length exceeds the upper bound"),
        };
        decoded.truncate(len);
        Ok(decoded)
    }
}
