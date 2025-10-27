use super::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization

use ::image::{DynamicImage, GenericImageView, ImageFormat};
use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::marker::PhantomData;
use std::fmt::{self, Formatter};


pub(crate) fn image_load_extensions() -> &'static[&'static str]
{
    &[
        "png",
        "jpg", "jpeg",
        "bmp",
        "gif",
        "webp"
    ]
}

pub(crate) fn image_save_extensions() -> &'static[&'static str]
{
    &[
        "png",
    ]
}

impl <C,Idx> ImageBaseOf<C,Idx>
    where
    Idx : Integer,
    C : Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>>,
    u8: CastRangeFrom<C::Component>,
    u16: CastRangeFrom<C::Component>,
{
    pub fn encode(&self, extension: &extension)  -> IoResult<Vec<u8>>
    {
        let mut buf = Vec::with_capacity(4096);
        self.encode_in(extension, &mut buf)?;
        Ok(buf)
    }

    pub fn encode_in(&self, extension: &extension, bytes : &mut Vec<u8>) -> IoResult<()>
    {
        match extension
        {
            "png" => {
                match C::Component::PRIMITIVE_TYPE
                {
                    NumberType::IntegerSigned =>
                    {
                        if std::mem::size_of::<C::Component>() * 8 <= 8
                        {
                            self.clone().to_rgba_u8().encode_in(extension, bytes)
                        }else
                        {
                            self.clone().to_rgba_u16().encode_in(extension, bytes)
                        }
                    },
                    NumberType::IntegerUnsigned => match std::mem::size_of::<C::Component>() * 8
                    {
                        8 =>
                        {
                            ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(bytes),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba8,
                            ).map_err(|e| IoError::encoding(format!("Failed to encode .png rgba8 image : {}", e.to_string())))
                        }
                        16 =>
                        {
                            ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(bytes),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba16,
                            ).map_err(|e| IoError::encoding(format!("Failed to encode .png rgba16 image : {}", e.to_string())))
                        }
                        _ =>
                        {
                            self.clone().to_rgba_u8().encode_in(extension, bytes)
                        }
                    }
                    NumberType::Float => self.clone().to_rgba_u16().encode_in(extension, bytes),
                    NumberType::Bool => self.clone().to_rgba_u8().encode_in(extension, bytes),
                }
            }
            _ => Err(IoError::UnsupportedExtension { mode: IoMode::Write, typename: "Image".to_owned(), got: extension.to_owned(), expected: image_save_extensions().into_iter().map(|ext| (*ext).to_owned()).to_vec() }),
        }
    }
}


impl<C,Idx> ImageBaseOf<C,Idx>
    where
    Idx : Integer,
    C : IColor,
{
    pub fn decode(extension: &extension, bytes: &[u8]) -> IoResult<Self>
    {
        let format = match extension
        {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "bmp" => ImageFormat::Bmp,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            /*
            "ico" => ImageFormat::Ico,
            "tiff" => ImageFormat::Tiff,
            */
            other =>  Err(IoError::UnsupportedExtension { mode: IoMode::Read, typename: "Image".to_owned(), got: extension.to_owned(), expected: image_load_extensions().into_iter().map(|ext| (*ext).to_owned()).to_vec() })?,
        };

        let img = ::image::load_from_memory_with_format(&bytes, format);
        let img = match img
        {
            Ok(dyn_img) => dyn_img,
            Err(e) => Err(IoError::encoding(e.to_debug()))?,
        };

        let (width, height) : (u32, u32) = img.dimensions();
        let w = Idx::cast_from(width);
        let h = Idx::cast_from(height);
        let casted_width = w.to_u32();
        let casted_height = h.to_u32();
        if casted_width != width || height != casted_height
        {
            return Err(IoError::encoding("Image is too big".to_owned()));
        }

        let error_invalid_size = || IoError::encoding("Invalid bytes len".to_owned());

        match C::Component::PRIMITIVE_TYPE
        {
            NumberType::IntegerSigned => {}, // Todo: handle >= 16 bits signed to use u16 precision ?
            NumberType::IntegerUnsigned =>
            {
                if std::mem::size_of::<C::Component>() * 8 >= 16
                {
                    let bytes = match img
                    {
                        DynamicImage::ImageRgba16(rgba) => rgba,
                        x => x.to_rgba16(),
                    }.into_raw();
                    let multiple = 4 * std::mem::size_of::<u16>(); // 4 components (rbga)
                    if bytes.len() % multiple != 0 || bytes.len() / 4 != vector2(w, h).area_usize()
                    {
                        return Err(error_invalid_size());
                    }

                    let rgba_vec: Vec<RgbaU16> = bytes
                        .chunks_exact(4)
                        .map(|chunk| RgbaU16 {
                            r: chunk[0],
                            g: chunk[1],
                            b: chunk[2],
                            a: chunk[3],
                        })
                        .collect();

                    let pixels = rgba_vec.into_iter().map(|v| C::from_rgba_u16(v)).collect();
                    let size = vector2(w, h);

                    return Self::from_vec(size, pixels).ok_or_else(error_invalid_size);
                }
            },
            NumberType::Float =>
            {
                let bytes = match img
                    {
                        DynamicImage::ImageRgba32F(rgba) => rgba,
                        x => x.to_rgba32f(),
                    }.into_raw();

                    let multiple = 4 * std::mem::size_of::<float>(); // 4 components (rbga)
                    if bytes.len() % multiple != 0 || bytes.len() / 4 != vector2(w, h).area_usize()
                    {
                        return Err(error_invalid_size());
                    }

                    let rgba_vec: Vec<RgbaF32> = bytes
                        .chunks_exact(4)
                        .map(|chunk| RgbaF32 {
                            r: chunk[0],
                            g: chunk[1],
                            b: chunk[2],
                            a: chunk[3],
                        })
                        .collect();

                    let pixels = rgba_vec.into_iter().map(|v| C::from_rgba_f32(v)).collect();
                    let size = vector2(w, h);

                    return Self::from_vec(size, pixels).ok_or_else(error_invalid_size);
            },
            NumberType::Bool => {},
        }

        // fallback on u8
        let bytes = match img
        {
            DynamicImage::ImageRgba8(rgba8) => rgba8,
            x => x.to_rgba8(),
        }.into_raw();
        let multiple = 4 * std::mem::size_of::<u8>(); // 4 components (rbga)
        if bytes.len() % multiple != 0 || bytes.len() / 4 != vector2(w, h).area_usize()
        {
            return Err(error_invalid_size());
        }

        let rgba_vec: Vec<RgbaU8> = bytes
            .chunks_exact(4)
            .map(|chunk| RgbaU8 {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            })
            .collect();

        let pixels = rgba_vec.into_iter().map(|v| C::from_rgba_u8(v)).collect();
        let size = vector2(w, h);

        Self::from_vec(size, pixels).ok_or_else(error_invalid_size)
    }
}

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
        let mut bytes = Vec::with_capacity(8192);

        if serializer.is_human_readable()
        {
            bytes.push_magic_file_extension("png");
        }
        self.encode_in("png", &mut bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }
}


impl<'de, C, Idx> Deserialize<'de> for ImageBaseOf<C, Idx>
    where
        Idx: Integer + Deserialize<'de>,
        C: IColor + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let is_human_readable = deserializer.is_human_readable();
        let bytes = <Vec<u8>>::deserialize(deserializer)?;

        if is_human_readable
        {
            let (extension, data) = bytes.extract_magic_file_extension_and_data().map_err(|_| serde::de::Error::custom("image binary data is not a multi file"))?;
            Self::decode(extension, data)
        }else
        {
            Self::decode("png", &bytes)
        }
        .map_err(serde::de::Error::custom)
    }
}