use super::*;
use ::base64::Engine;


pub trait ToBase64
{
    fn to_base64(&self) -> String
    {
        self.to_base64_in(String::with_capacity(1024))
    }
    fn to_base64_in(&self, base64: String) -> String;
}
impl ToBase64 for [u8]
{
    fn to_base64_in(&self, mut base64: String) -> String {
        ::base64::engine::general_purpose::STANDARD.encode_string(self, &mut base64);
        base64
    }
}

pub type Base64Error = ::base64::DecodeError;
pub trait FromBase64 : Sized
{
    fn from_base64<D>(data: D) -> EncodeResult<Self>
        where D: AsRef<[u8]>;
}
impl FromBase64 for Vec<u8>
{
    fn from_base64<D>(data: D) -> EncodeResult<Self>
        where D: AsRef<[u8]>
    {
        ::base64::engine::general_purpose::STANDARD.decode(data).map_err(|e| e.into())
    }
}
