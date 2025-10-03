#[cfg(feature = "hexga_io")]
use ::hexga_io::{IoResult, IoSaveResult, extension, path};

use super::*;


#[cfg(feature = "hexga_io")]
impl<C,Idx> IoLoad for ImageBaseOf<C,Idx>
    where
    Idx : Integer + for<'a> Deserialize<'a>,
    C : IColor + for<'a> Deserialize<'a>,
{
    fn load_own_extensions() -> impl Iterator<Item = &'static str> {
        [
            "png",
            "jpeg", "jpg",
            "bmp",
            "gif",
            "webp",
        ].iter().copied()
    }

    fn load_from_bytes_with_own_extension_pathless(data : &[u8], extension : &extension) -> IoResult<Self>
    {
        use ::image::{DynamicImage, GenericImageView, ImageFormat};

        let format = match extension.to_lowercase().as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "bmp" => ImageFormat::Bmp,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            /*
            "ico" => ImageFormat::Ico,
            "tiff" => ImageFormat::Tiff,
            */
            other => Err(IoErrorKind::UnsupportedExtension { name: "Image".to_owned(), got: extension.to_owned(), expected: Self::load_extensions().map(|s| s.to_owned()).collect() })?,
        };

        let img = ::image::load_from_memory_with_format(data, format);

        let img: DynamicImage = match img
        {
            Ok(v) => v,
            Err(e) => Err(IoErrorKind::Encoding(e.to_debug()))?,
        };

        let (width, height) : (u32, u32) = img.dimensions();
        let w = Idx::cast_from(width);
        let h = Idx::cast_from(height);
        let casted_width = w.to_u32();
        let casted_height = h.to_u32();
        if casted_width != width || height != casted_height
        {
            return Err(IoErrorKind::Encoding("Image is too big".to_owned()));
        }

        let error_invalid_size = || IoErrorKind::Encoding("Invalid bytes len".to_owned());

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

#[cfg(feature = "hexga_io")]
impl<C,Idx> IoSave for ImageBaseOf<C,Idx>
    where
    Idx : Integer + Serialize,
    C : Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>> + Serialize,
    u8: CastRangeFrom<C::Component>,
    u16: CastRangeFrom<C::Component>,
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
                    NumberType::IntegerSigned =>
                    {
                        if std::mem::size_of::<C::Component>() * 8 <= 8
                        {
                            self.clone().to_rgba_u8().save_to_with_own_extension_pathless(extension, w, fs)
                        }else
                        {
                            self.clone().to_rgba_u16().save_to_with_own_extension_pathless(extension, w, fs)
                        }
                    },
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
                            self.clone().to_rgba_u8().save_to_with_own_extension_pathless(extension, w, fs)
                        }
                    }
                    NumberType::Float => self.clone().to_rgba_u16().save_to_with_own_extension_pathless(extension, w, fs),
                    NumberType::Bool => self.clone().to_rgba_u8().save_to_with_own_extension_pathless(extension, w, fs),
                }
            },
            /*
            "jpeg" | "jpg" =>
            {
                // Todo: Jpeg don't support alpha
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