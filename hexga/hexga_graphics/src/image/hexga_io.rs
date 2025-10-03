#[cfg(feature = "hexga_io")]
use ::hexga_io::{IoResult, IoSaveResult, extension, path};

use super::*;

#[cfg(feature = "hexga_io")]
impl<C,Idx> IoSave for ImageBase<C,Idx>
    where
    Idx : Integer + Serialize,
    C : IColor + Serialize
{
    fn save_own_extensions() -> impl Iterator<Item = &'static str> {
        [
            "png",
            //"jpeg", "jpg",
            //"gif",
        ].iter().copied()
    }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, extension: &extension, w: W, fs: &mut Fs) -> IoResult
            where W : Write, Fs : ::hexga_io::prelude::IoFsWrite
    {
        match extension
        {
            "png" =>
            {
                match C::Component::PRIMITIVE_TYPE
                {
                    NumberType::IntegerSigned => todo!(),
                    NumberType::IntegerUnsigned => match std::mem::size_of::<C::Component>() * 8
                    {
                        8 =>
                        {
                            ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(w),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba8,
                            ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .png rgba8 image : {}", e.to_string())))
                        }
                        16 =>
                        {
                            ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(w),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba16,
                            ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .png rgba16 image : {}", e.to_string())))
                        }
                        _ =>
                        {
                            self.to_rgba_u8().save_to_with_own_extension_pathless(extension, w, fs)
                        }
                    }
                    NumberType::Float => self.to_rgba_u16().save_to_with_own_extension_pathless(extension, w, fs),
                    NumberType::Bool => self.to_rgba_u8().save_to_with_own_extension_pathless(extension, w, fs),
                }
            },
            /*
            "jpeg" | "jpg" =>
            {
                match C::COLOR_INSIDE
                {
                    ColorKind::RGBAByte =>
                    {
                        ::image::ImageEncoder::write_image(
                            ::image::codecs::jpeg::JpegEncoder::new(w),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba8,
                        ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .jpeg rgba8 image : {}", e.to_string())))
                    },
                    ColorKind::RGBAU16 =>
                    {
                        ::image::ImageEncoder::write_image(
                            ::image::codecs::jpeg::JpegEncoder::new(w),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba16,
                        ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .jpeg rgba16 image : {}", e.to_string())))
                    },
                    ColorKind::RGBAF32 =>
                    {
                        ::image::ImageEncoder::write_image(
                            ::image::codecs::jpeg::JpegEncoder::new(w),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba32F,
                        ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .jpeg rgba32F image : {}", e.to_string())))
                    },
                    ColorKind::RGBAF64 => self.transform(|p| p.to_color_rgba_f32()).save_to_with_own_extension_pathless(extension, w, fs),
                    _ => self.transform(|p| p.to_color_rgba_byte()).save_to_with_own_extension_pathless(extension, w, fs),
                }
            },
            */
            /*
            "gif" =>
            {
                match C::COLOR_INSIDE
                {
                    ColorKind::RgbaU8 =>
                    {
                        let mut encoder = ::image::codecs::gif::GifEncoder::new(w);
                        let result = encoder.encode(
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as u32,
                            self.height().to_usize() as u32,
                            ::image::ColorType::Rgba8.into(),
                        );
                        result.map_err(|e| IoErrorKind::Encoding(format!("Failed to save .gif rgba8 image : {}", e.to_string())))
                    },
                    _ => self.to_rgba_u8().save_to_with_own_extension_pathless(extension, w, fs),
                }
            }
            */
            _ => Err(IoErrorKind::UnsupportedExtension { name: "Image".to_owned(), got: extension.to_owned(), expected: Self::save_extensions().map(|s| s.to_owned()).collect() })
        }
    }
}