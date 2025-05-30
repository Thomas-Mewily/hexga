use crate::*;

// Todo : check https://github.com/RReverser/serde-ndim/tree/main
// Support nested array during deserialization


#[cfg(feature = "serde")]
impl<'de, T, Idx> Deserialize<'de> for ImageBase<T, Idx>
    where
        Idx: Integer + Deserialize<'de>,
        T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ImageVisitor<T, Idx,> {
            marker: std::marker::PhantomData<(T, Idx)>,
        }

        impl<'de, T, Idx,> Visitor<'de> for ImageVisitor<T, Idx>
        where
            Idx: Integer + Deserialize<'de>,
            T:  Deserialize<'de>,
        {
            type Value = ImageBase<T,Idx>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("An Image with an `size` and `pixels`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut size: Option<Vector2<Idx>> = None;
                let mut pixels: Option<Vec<T>> = None;

                while let Some(key) = map.next_key()?
                {
                    match key
                    {
                        "size" => {
                            if size.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size = Some(map.next_value()?);
                        }
                        "pixels" => {
                            if pixels.is_some() {
                                return Err(serde::de::Error::duplicate_field("pixels"));
                            }
                            pixels = Some(map.next_value()?);
                        }
                        _ => { let _ = map.next_value::<serde::de::IgnoredAny>()?; }
                    }
                }

                let size = size.ok_or_else(|| serde::de::Error::missing_field("size"))?;
                let pixels = pixels.ok_or_else(|| serde::de::Error::missing_field("pixels"))?;

                match ImageBase::try_from_vec(size, pixels)
                {
                    Ok(g) => Ok(g),
                    Err(e) => Err(serde::de::Error::custom(
                        match e
                        {
                            ImageBaseError::NegativeSize(vector) => format!("Area component of the image can't be negative : {:?}", vector),
                            ImageBaseError::WrongDimension(vector, got) => format!("The area of the image ({:?} => {} pixels) does not match the number of pixels ({})", vector, vector.area_usize(), got),
                        }
                    ))
                }
            }
        }

        deserializer.deserialize_struct("Image", &["size", "pixels"], ImageVisitor {
            marker: std::marker::PhantomData,
        })
    }
}



/*
#[cfg(feature = "hexga_io")]
impl<C,Idx> hexga_io::IoLoad for ImageBase<C,Idx>
    where
    Idx : Integer + for<'de> Deserialize<'de>,
    C : ToColor + for<'de> Deserialize<'de>
{
    fn load_own_extensions() -> impl Iterator<Item = &'static str> {
        [
            "png",
            "jpeg", "jpg",
            "gif",
        ].iter().copied()
    }

    fn load_from_bytes_with_own_extension_pathless(data : &[u8], extension : &hexga_io::extension) -> hexga_io::IoResult<Self>
    {
        match extension
        {
            "png" =>
            {
                match C::COLOR_INSIDE
                {
                    ColorKind::RGBAByte =>
                    {
                        ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(buffered_write),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba8,
                        ).expect("Failed to write PNG Rgba8 image");
                    },
                    ColorKind::RGBAU16 =>
                    {
                        ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(buffered_write),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba16,
                        ).expect("Failed to write PNG Rgba16 mage");
                    },
                    _ => todo!(),
                }
            }
            _ => Err(___())
        }
        todo!()
    }
}
    */

#[cfg(feature = "hexga_io")]
impl<C,Idx> hexga_io::IoSave for ImageBase<C,Idx>
    where
    Idx : Integer + Serialize,
    C : ToColor + Serialize
{
    fn save_own_extensions() -> impl Iterator<Item = &'static str> {
        [
            "png",
            "jpeg", "jpg",
            "gif",
        ].iter().copied()
    }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, extension : &hexga_io::extension, w : W, fs : &mut Fs) -> hexga_io::IoResult
            where W : Write, Fs : hexga_io::prelude::IoFsWrite
    {
        match extension
        {
            "png" =>
            {
                match C::COLOR_INSIDE
                {
                    ColorKind::RGBABool =>
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
                            ::image::ExtendedColorType::Rgba1,
                        ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .png rgba1 image : {}", e.to_string())))
                    }
                    ColorKind::RGBAByte =>
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
                    },
                    ColorKind::RGBAU16 =>
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
                    },
                    ColorKind::RGBAF32 =>
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
                            ::image::ExtendedColorType::Rgba32F,
                        ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .png rgba32F image : {}", e.to_string())))
                    },
                    ColorKind::RGBAF64 => self.transform(|p| p.to_color_rgba_f32()).save_to_with_own_extension_pathless(extension, w, fs),
                    _ => self.transform(|p| p.to_color_rgba_byte()).save_to_with_own_extension_pathless(extension, w, fs),
                }
            },
            "jpeg" | "jpg" =>
            {
                match C::COLOR_INSIDE
                {
                    ColorKind::RGBABool =>
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
                            ::image::ExtendedColorType::Rgba1,
                        ).map_err(|e| IoErrorKind::Encoding(format!("Failed to save .jpeg rgba1 image : {}", e.to_string())))
                    }
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
            "gif" =>
            {
                match C::COLOR_INSIDE
                {
                    ColorKind::RGBAByte =>
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
                    ColorKind::RGBAU16 =>
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
                            ::image::ColorType::Rgba16.into(),
                        );
                        result.map_err(|e| IoErrorKind::Encoding(format!("Failed to save .gif rgba16 image : {}", e.to_string())))
                    },
                    ColorKind::RGBAF32 =>
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
                            ::image::ExtendedColorType::Rgba32F.into(),
                        );
                        result.map_err(|e| IoErrorKind::Encoding(format!("Failed to save .gif rgba32F image : {}", e.to_string())))
                    },
                    ColorKind::RGBAF64 => self.transform(|p| p.to_color_rgba_f32()).save_to_with_own_extension_pathless(extension, w, fs),
                    _ => self.transform(|p| p.to_color_rgba_byte()).save_to_with_own_extension_pathless(extension, w, fs),
                }
            }
            _ => Err(___())
        }
    }
}