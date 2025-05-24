use hexga_engine::render::*;
use crate::*;

/// MiniQuad <=> Hexga Engine conversion
pub(crate) trait Convert
{
    type Output;
    fn convert(self) -> Self::Output;
}

/// MiniQuad <=> Hexga Engine conversion
pub(crate) trait ConvertRef
{
    type Output<'a> where Self: 'a;
    fn convert<'a>(&'a self) -> Self::Output<'a>;
}

impl Convert for miniquad::TouchPhase
{
    type Output = TouchPhase;

    fn convert(self) -> Self::Output {
        match self
        {
            miniquad::TouchPhase::Started => TouchPhase::Begin,
            miniquad::TouchPhase::Moved => TouchPhase::Move,
            miniquad::TouchPhase::Ended => TouchPhase::End,
            miniquad::TouchPhase::Cancelled => TouchPhase::Cancel,
        }
    }
}

impl Convert for miniquad::MouseButton
{
    type Output = MouseButton;

    fn convert(self) -> Self::Output {
        match self
        {
            miniquad::MouseButton::Left => MouseButton::Left,
            miniquad::MouseButton::Middle => MouseButton::Middle,
            miniquad::MouseButton::Right => MouseButton::Right,
            miniquad::MouseButton::Unknown => MouseButton::Unknown,
        }
    }
}

impl Convert for miniquad::KeyMods
{
    type Output = KeyMods;

    fn convert(self) -> Self::Output {
        let miniquad::KeyMods{ shift, ctrl, alt, logo } = self;
        KeyMods { shift, ctrl, alt, logo }
    }
}

impl Convert for miniquad::KeyCode
{
    type Output = KeyCode;

    fn convert(self) -> Self::Output {
        match self
        {
            miniquad::KeyCode::Space => KeyCode::Space,
            miniquad::KeyCode::Apostrophe => KeyCode::Apostrophe,
            miniquad::KeyCode::Comma => KeyCode::Comma,
            miniquad::KeyCode::Minus => KeyCode::Minus,
            miniquad::KeyCode::Period => KeyCode::Period,
            miniquad::KeyCode::Slash => KeyCode::Slash,
            miniquad::KeyCode::Key0 => KeyCode::Key0,
            miniquad::KeyCode::Key1 => KeyCode::Key1,
            miniquad::KeyCode::Key2 => KeyCode::Key2,
            miniquad::KeyCode::Key3 => KeyCode::Key3,
            miniquad::KeyCode::Key4 => KeyCode::Key4,
            miniquad::KeyCode::Key5 => KeyCode::Key5,
            miniquad::KeyCode::Key6 => KeyCode::Key6,
            miniquad::KeyCode::Key7 => KeyCode::Key7,
            miniquad::KeyCode::Key8 => KeyCode::Key8,
            miniquad::KeyCode::Key9 => KeyCode::Key9,
            miniquad::KeyCode::Semicolon => KeyCode::Semicolon,
            miniquad::KeyCode::Equal => KeyCode::Equal,
            miniquad::KeyCode::A => KeyCode::A,
            miniquad::KeyCode::B => KeyCode::B,
            miniquad::KeyCode::C => KeyCode::C,
            miniquad::KeyCode::D => KeyCode::D,
            miniquad::KeyCode::E => KeyCode::E,
            miniquad::KeyCode::F => KeyCode::F,
            miniquad::KeyCode::G => KeyCode::G,
            miniquad::KeyCode::H => KeyCode::H,
            miniquad::KeyCode::I => KeyCode::I,
            miniquad::KeyCode::J => KeyCode::J,
            miniquad::KeyCode::K => KeyCode::K,
            miniquad::KeyCode::L => KeyCode::L,
            miniquad::KeyCode::M => KeyCode::M,
            miniquad::KeyCode::N => KeyCode::N,
            miniquad::KeyCode::O => KeyCode::O,
            miniquad::KeyCode::P => KeyCode::P,
            miniquad::KeyCode::Q => KeyCode::Q,
            miniquad::KeyCode::R => KeyCode::R,
            miniquad::KeyCode::S => KeyCode::S,
            miniquad::KeyCode::T => KeyCode::T,
            miniquad::KeyCode::U => KeyCode::U,
            miniquad::KeyCode::V => KeyCode::V,
            miniquad::KeyCode::W => KeyCode::W,
            miniquad::KeyCode::X => KeyCode::X,
            miniquad::KeyCode::Y => KeyCode::Y,
            miniquad::KeyCode::Z => KeyCode::Z,
            miniquad::KeyCode::LeftBracket => KeyCode::LeftBracket,
            miniquad::KeyCode::Backslash => KeyCode::Backslash,
            miniquad::KeyCode::RightBracket => KeyCode::RightBracket,
            miniquad::KeyCode::GraveAccent => KeyCode::GraveAccent,
            miniquad::KeyCode::World1 => KeyCode::World1,
            miniquad::KeyCode::World2 => KeyCode::World2,
            miniquad::KeyCode::Escape => KeyCode::Escape,
            miniquad::KeyCode::Enter => KeyCode::Enter,
            miniquad::KeyCode::Tab => KeyCode::Tab,
            miniquad::KeyCode::Backspace => KeyCode::Backspace,
            miniquad::KeyCode::Insert => KeyCode::Insert,
            miniquad::KeyCode::Delete => KeyCode::Delete,
            miniquad::KeyCode::Right => KeyCode::Right,
            miniquad::KeyCode::Left => KeyCode::Left,
            miniquad::KeyCode::Down => KeyCode::Down,
            miniquad::KeyCode::Up => KeyCode::Up,
            miniquad::KeyCode::PageUp => KeyCode::PageUp,
            miniquad::KeyCode::PageDown => KeyCode::PageDown,
            miniquad::KeyCode::Home => KeyCode::Home,
            miniquad::KeyCode::End => KeyCode::End,
            miniquad::KeyCode::CapsLock => KeyCode::CapsLock,
            miniquad::KeyCode::ScrollLock => KeyCode::ScrollLock,
            miniquad::KeyCode::NumLock => KeyCode::NumLock,
            miniquad::KeyCode::PrintScreen => KeyCode::PrintScreen,
            miniquad::KeyCode::Pause => KeyCode::Pause,
            miniquad::KeyCode::F1 => KeyCode::F1,
            miniquad::KeyCode::F2 => KeyCode::F2,
            miniquad::KeyCode::F3 => KeyCode::F3,
            miniquad::KeyCode::F4 => KeyCode::F4,
            miniquad::KeyCode::F5 => KeyCode::F5,
            miniquad::KeyCode::F6 => KeyCode::F6,
            miniquad::KeyCode::F7 => KeyCode::F7,
            miniquad::KeyCode::F8 => KeyCode::F8,
            miniquad::KeyCode::F9 => KeyCode::F9,
            miniquad::KeyCode::F10 => KeyCode::F10,
            miniquad::KeyCode::F11 => KeyCode::F11,
            miniquad::KeyCode::F12 => KeyCode::F12,
            miniquad::KeyCode::F13 => KeyCode::F13,
            miniquad::KeyCode::F14 => KeyCode::F14,
            miniquad::KeyCode::F15 => KeyCode::F15,
            miniquad::KeyCode::F16 => KeyCode::F16,
            miniquad::KeyCode::F17 => KeyCode::F17,
            miniquad::KeyCode::F18 => KeyCode::F18,
            miniquad::KeyCode::F19 => KeyCode::F19,
            miniquad::KeyCode::F20 => KeyCode::F20,
            miniquad::KeyCode::F21 => KeyCode::F21,
            miniquad::KeyCode::F22 => KeyCode::F22,
            miniquad::KeyCode::F23 => KeyCode::F23,
            miniquad::KeyCode::F24 => KeyCode::F24,
            miniquad::KeyCode::F25 => KeyCode::F25,
            miniquad::KeyCode::Kp0 => KeyCode::Kp0,
            miniquad::KeyCode::Kp1 => KeyCode::Kp1,
            miniquad::KeyCode::Kp2 => KeyCode::Kp2,
            miniquad::KeyCode::Kp3 => KeyCode::Kp3,
            miniquad::KeyCode::Kp4 => KeyCode::Kp4,
            miniquad::KeyCode::Kp5 => KeyCode::Kp5,
            miniquad::KeyCode::Kp6 => KeyCode::Kp6,
            miniquad::KeyCode::Kp7 => KeyCode::Kp7,
            miniquad::KeyCode::Kp8 => KeyCode::Kp8,
            miniquad::KeyCode::Kp9 => KeyCode::Kp9,
            miniquad::KeyCode::KpDecimal => KeyCode::KpDecimal,
            miniquad::KeyCode::KpDivide => KeyCode::KpDivide,
            miniquad::KeyCode::KpMultiply => KeyCode::KpMultiply,
            miniquad::KeyCode::KpSubtract => KeyCode::KpSubtract,
            miniquad::KeyCode::KpAdd => KeyCode::KpAdd,
            miniquad::KeyCode::KpEnter => KeyCode::KpEnter,
            miniquad::KeyCode::KpEqual => KeyCode::KpEqual,
            miniquad::KeyCode::LeftShift => KeyCode::LeftShift,
            miniquad::KeyCode::LeftControl => KeyCode::LeftControl,
            miniquad::KeyCode::LeftAlt => KeyCode::LeftAlt,
            miniquad::KeyCode::LeftSuper => KeyCode::LeftSuper,
            miniquad::KeyCode::RightShift => KeyCode::RightShift,
            miniquad::KeyCode::RightControl => KeyCode::RightControl,
            miniquad::KeyCode::RightAlt => KeyCode::RightAlt,
            miniquad::KeyCode::RightSuper => KeyCode::RightSuper,
            miniquad::KeyCode::Menu => KeyCode::Menu,
            miniquad::KeyCode::Back => KeyCode::Back,
            miniquad::KeyCode::Unknown => KeyCode::Unknown,
        }
    }
}

impl Convert for MultiMediaParam
{
    type Output=miniquad::conf::Conf;

    fn convert(self) -> Self::Output {
        let Self { window_param: window_config } = self;
        let WindowParam 
        { 
            title: window_title, 
            size,
            high_dpi,
            fullscreen, 
            sample_count, 
            resizable, 
            icon, 
            platform 
        } = window_config;

        miniquad::conf::Conf
        {
            window_title,
            window_width: size.x as _,
            window_height: size.y as _,
            high_dpi,
            fullscreen,
            sample_count: sample_count as _,
            window_resizable : resizable,
            icon: icon.map(|v| v.convert()),
            platform: platform.convert(),
        }
    }
}

impl Convert for window::Icon
{
    type Output = miniquad::conf::Icon;

    fn convert(self) -> Self::Output 
    {
        let Self { rgba_16x16, rgba_32x32, rgba_64x64 } = self;
        Self::Output { small: rgba_16x16, medium: rgba_32x32, big: rgba_64x64 }
    }
}

impl Convert for window::Platform
{
    type Output = miniquad::conf::Platform;

    fn convert(self) -> Self::Output {
        let Self { swap_interval, blocking_event_loop, framebuffer_alpha } = self;
        let mut p = miniquad::conf::Platform::default();
        p.swap_interval = swap_interval;
        p.blocking_event_loop = blocking_event_loop;
        p.framebuffer_alpha = framebuffer_alpha;
        p
    }
}

impl Convert for window::CursorIcon
{
    type Output =  miniquad::CursorIcon;

    fn convert(self) -> Self::Output 
    {
        match self
        {
            window::CursorIcon::Default => miniquad::CursorIcon::Default,
            window::CursorIcon::Help => miniquad::CursorIcon::Help,
            window::CursorIcon::Pointer => miniquad::CursorIcon::Pointer,
            window::CursorIcon::Wait => miniquad::CursorIcon::Wait,
            window::CursorIcon::Crosshair => miniquad::CursorIcon::Crosshair,
            window::CursorIcon::Text => miniquad::CursorIcon::Text,
            window::CursorIcon::Move => miniquad::CursorIcon::Move,
            window::CursorIcon::NotAllowed => miniquad::CursorIcon::NotAllowed,
            window::CursorIcon::EWResize => miniquad::CursorIcon::EWResize,
            window::CursorIcon::NSResize => miniquad::CursorIcon::NSResize,
            window::CursorIcon::NESWResize => miniquad::CursorIcon::NESWResize,
            window::CursorIcon::NWSEResize => miniquad::CursorIcon::NWSEResize,
        }
    }
}

impl Convert for buffer::RawBufferID
{
    type Output = miniquad::BufferId;
    fn convert(self) -> Self::Output {
        unsafe { std::mem::transmute(self) }
    }
}
impl Convert for miniquad::BufferId
{
    type Output = buffer::RawBufferID;
    fn convert(self) -> Self::Output {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'b> ConvertRef for buffer::BufferSource<'b>
{
    type Output<'a> = miniquad::BufferSource<'a> where Self: 'a;

    fn convert<'a>(&'a self) -> Self::Output<'a>
    {
        // I'm not sure about the alignement
        match &self
        {
            buffer::BufferSource::UntypedSlice(untyped_slice) 
                => miniquad::BufferSource::slice(&*untyped_slice),
            buffer::BufferSource::Empty(buffer_layout) 
                => miniquad::BufferSource::empty::<u8>(buffer_layout.size()),
        }
    }
}

impl Convert for texture::TextureAccess
{
    type Output = miniquad::TextureAccess;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            texture::TextureAccess::Static => miniquad::TextureAccess::Static,
            texture::TextureAccess::RenderTarget => miniquad::TextureAccess::RenderTarget,
        }
    }
}

impl Convert for texture::TextureFormat
{
    type Output = miniquad::TextureFormat;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            texture::TextureFormat::RGBA8 => miniquad::TextureFormat::RGBA8,
        }
    }
}

impl Convert for texture::TextureWrap
{
    type Output = miniquad::TextureWrap;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            texture::TextureWrap::Repeat => miniquad::TextureWrap::Repeat,
            texture::TextureWrap::Mirror => miniquad::TextureWrap::Mirror,
            texture::TextureWrap::Clamp  => miniquad::TextureWrap::Clamp,
        }
    }
}

impl Convert for texture::FilterMode
{
    type Output = miniquad::FilterMode;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            texture::FilterMode::Linear => miniquad::FilterMode::Linear,
            texture::FilterMode::Nearest => miniquad::FilterMode::Nearest,
        }
    }
}

impl Convert for texture::MipmapFilterMode
{
    type Output = miniquad::MipmapFilterMode;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            texture::MipmapFilterMode::None => miniquad::MipmapFilterMode::None,
            texture::MipmapFilterMode::Linear => miniquad::MipmapFilterMode::Linear,
            texture::MipmapFilterMode::Nearest => miniquad::MipmapFilterMode::Nearest,
        }
    }
}

impl Convert for vertex::VertexStep
{
    type Output = miniquad::VertexStep;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            vertex::VertexStep::PerVertex   => miniquad::VertexStep::PerVertex,
            vertex::VertexStep::PerInstance => miniquad::VertexStep::PerInstance,
        }
    }
}

impl Convert for vertex::VertexBufferLayout
{
    type Output = miniquad::BufferLayout;
    fn convert(self) -> Self::Output 
    {
        let Self { stride, step_func, step_rate } = self;
        miniquad::BufferLayout { stride, step_func: step_func.convert(), step_rate }
    }
}

impl Convert for texture::TextureParam
{
    type Output = miniquad::TextureParams;
    fn convert(self) -> Self::Output
    {
        let Self 
        { 
            format, 
            wrap, 
            min_filter, 
            mag_filter, 
            mipmap_filter, 
            allocate_mipmaps, 
            size,
            sample_count, 
            access : _ 
        } = self;

        let [width, height] = size;

        miniquad::TextureParams 
        { 
            kind: miniquad::TextureKind::Texture2D, 
            format: format.convert(), 
            wrap: wrap[0].convert(), 
            min_filter: min_filter.convert(), 
            mag_filter: mag_filter.convert(), 
            mipmap_filter: mipmap_filter.convert(), 
            width: width as _,
            height: height as _,
            allocate_mipmaps,
            sample_count : sample_count as _ 
        }
    }
}


impl Convert for pipeline::CullFace
{
    type Output=miniquad::CullFace;

    fn convert(self) -> Self::Output {
        match self
        {
            pipeline::CullFace::Nothing => miniquad::CullFace::Nothing,
            pipeline::CullFace::Front => miniquad::CullFace::Front,
            pipeline::CullFace::Back => miniquad::CullFace::Back,
        }
    }
}

impl Convert for pipeline::FrontFaceOrder
{
    type Output=miniquad::FrontFaceOrder;

    fn convert(self) -> Self::Output {
        match self
        {
            pipeline::FrontFaceOrder::Clockwise => miniquad::FrontFaceOrder::Clockwise,
            pipeline::FrontFaceOrder::CounterClockwise => miniquad::FrontFaceOrder::CounterClockwise,
        }
    }
}

impl Convert for pipeline::DepthComparison
{
    type Output=miniquad::Comparison;

    fn convert(self) -> Self::Output 
    {
        match self
        {
            pipeline::DepthComparison::Never => miniquad::Comparison::Never,
            pipeline::DepthComparison::Less => miniquad::Comparison::Less,
            pipeline::DepthComparison::LessOrEqual => miniquad::Comparison::LessOrEqual,
            pipeline::DepthComparison::Greater => miniquad::Comparison::Greater,
            pipeline::DepthComparison::GreaterOrEqual => miniquad::Comparison::GreaterOrEqual,
            pipeline::DepthComparison::Equal => miniquad::Comparison::Equal,
            pipeline::DepthComparison::NotEqual => miniquad::Comparison::NotEqual,
            pipeline::DepthComparison::Always => miniquad::Comparison::Always,
        }
    }
}


impl Convert for blend::BlendFactor
{
    type Output=miniquad::BlendFactor;

    fn convert(self) -> Self::Output 
    {
        match self
        {
            blend::BlendFactor::Zero => miniquad::BlendFactor::Zero,
            blend::BlendFactor::One => miniquad::BlendFactor::One,
            blend::BlendFactor::Value(blend_value) => miniquad::BlendFactor::Value(blend_value.convert()),
            blend::BlendFactor::OneMinusValue(blend_value) => miniquad::BlendFactor::OneMinusValue(blend_value.convert()),
            blend::BlendFactor::SourceAlphaSaturate => miniquad::BlendFactor::SourceAlphaSaturate,
        }
    }
}

impl Convert for blend::BlendState
{
    type Output=miniquad::BlendState;

    fn convert(self) -> Self::Output 
    {
        let Self { equation, sfactor, dfactor } = self;
        miniquad::BlendState::new(equation.convert(), sfactor.convert(), dfactor.convert())
    }
}

impl Convert for blend::BlendEquation
{
    type Output=miniquad::Equation;

    fn convert(self) -> Self::Output {
        match self
        {
            blend::BlendEquation::Add => miniquad::Equation::Add,
            blend::BlendEquation::Subtract => miniquad::Equation::Subtract,
            blend::BlendEquation::ReverseSubtract => miniquad::Equation::ReverseSubtract,
        }
    }
}

impl Convert for blend::BlendValue
{
    type Output=miniquad::BlendValue;

    fn convert(self) -> Self::Output {
        match self
        {
            blend::BlendValue::SourceColor => miniquad::BlendValue::SourceColor,
            blend::BlendValue::SourceAlpha => miniquad::BlendValue::SourceAlpha,
            blend::BlendValue::DestinationColor => miniquad::BlendValue::DestinationColor,
            blend::BlendValue::DestinationAlpha => miniquad::BlendValue::DestinationAlpha,
        }
    }
}

impl Convert for stencil::StencilCompareFunc
{
    type Output=miniquad::CompareFunc;

    fn convert(self) -> Self::Output {
        match self
        {
            stencil::StencilCompareFunc::Never => miniquad::CompareFunc::Never,
            stencil::StencilCompareFunc::Less => miniquad::CompareFunc::Less,
            stencil::StencilCompareFunc::LessOrEqual => miniquad::CompareFunc::LessOrEqual,
            stencil::StencilCompareFunc::Greater => miniquad::CompareFunc::Greater,
            stencil::StencilCompareFunc::GreaterOrEqual => miniquad::CompareFunc::GreaterOrEqual,
            stencil::StencilCompareFunc::Equal => miniquad::CompareFunc::Equal,
            stencil::StencilCompareFunc::NotEqual => miniquad::CompareFunc::NotEqual,
            stencil::StencilCompareFunc::Always => miniquad::CompareFunc::Always,
        }
    }
}

impl Convert for stencil::StencilFaceState
{
    type Output=miniquad::StencilFaceState;

    fn convert(self) -> Self::Output {
        let Self { fail_op, depth_fail_op, pass_op, test_func, test_ref, test_mask, write_mask } = self;
        miniquad::StencilFaceState 
        { 
            fail_op: fail_op.convert(), 
            depth_fail_op: depth_fail_op.convert(), 
            pass_op: pass_op.convert(), 
            test_func: test_func.convert(), 
            test_ref, 
            test_mask, 
            write_mask 
        }
    }
}

impl Convert for stencil::StencilOp
{
    type Output=miniquad::StencilOp;

    fn convert(self) -> Self::Output {
        match self
        {
            stencil::StencilOp::Keep => miniquad::StencilOp::Keep,
            stencil::StencilOp::Zero => miniquad::StencilOp::Zero,
            stencil::StencilOp::Replace => miniquad::StencilOp::Replace,
            stencil::StencilOp::IncrementClamp => miniquad::StencilOp::IncrementClamp,
            stencil::StencilOp::DecrementClamp => miniquad::StencilOp::DecrementClamp,
            stencil::StencilOp::Invert => miniquad::StencilOp::Invert,
            stencil::StencilOp::IncrementWrap => miniquad::StencilOp::IncrementWrap,
            stencil::StencilOp::DecrementWrap => miniquad::StencilOp::DecrementWrap,
        }
    }
}

impl Convert for stencil::StencilState
{
    type Output=miniquad::StencilState;

    fn convert(self) -> Self::Output {
        let Self { front, back } = self;
        miniquad::StencilState 
        { 
            front: front.convert(), 
            back: back.convert() 
        }
    }
}


impl Convert for pipeline::PrimitiveType
{
    type Output=miniquad::PrimitiveType;

    fn convert(self) -> Self::Output {
        match self
        {
            pipeline::PrimitiveType::Triangles => miniquad::PrimitiveType::Triangles,
            pipeline::PrimitiveType::Lines => miniquad::PrimitiveType::Lines,
            pipeline::PrimitiveType::Points => miniquad::PrimitiveType::Points,
        }
    }
}


impl Convert for pipeline::PipelineParam
{
    type Output = miniquad::PipelineParams;
    fn convert(self) -> Self::Output 
    {
        let Self 
        { 
            cull_face, 
            front_face_order, 
            color_mask, 
            primitive_type, 
            depth_test, 
            depth_write, 
            depth_write_offset, 
            color_blend, 
            alpha_blend, 
            stencil_test 
        } = self;

        let [cr, rg, rb, ra] = color_mask.into();
        
        miniquad::PipelineParams 
        { 
            cull_face: cull_face.convert(), 
            front_face_order: front_face_order.convert(), 
            depth_test: depth_test.convert(), 
            depth_write: depth_write, 
            depth_write_offset: depth_write_offset.map(|[x,y]| (x as _, y as _)), 
            color_blend: color_blend.map(|v| v.convert()), 
            alpha_blend: alpha_blend.map(|v| v.convert()), 
            stencil_test: stencil_test.map(|v| v.convert()), 
            color_write:  (cr, rg, rb, ra),
            primitive_type: primitive_type.convert(), 
        }
    }
}

impl Convert for vertex::VertexFormat
{
    type Output = miniquad::VertexFormat;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            vertex::VertexFormat::Float1 => miniquad::VertexFormat::Float1,
            vertex::VertexFormat::Float2 => miniquad::VertexFormat::Float2,
            vertex::VertexFormat::Float3 => miniquad::VertexFormat::Float3,
            vertex::VertexFormat::Float4 => miniquad::VertexFormat::Float4,
            vertex::VertexFormat::Byte1 => miniquad::VertexFormat::Byte1,
            vertex::VertexFormat::Byte2 => miniquad::VertexFormat::Byte2,
            vertex::VertexFormat::Byte3 => miniquad::VertexFormat::Byte3,
            vertex::VertexFormat::Byte4 => miniquad::VertexFormat::Byte4,
            vertex::VertexFormat::Short1 => miniquad::VertexFormat::Short1,
            vertex::VertexFormat::Short2 => miniquad::VertexFormat::Short2,
            vertex::VertexFormat::Short3 => miniquad::VertexFormat::Short3,
            vertex::VertexFormat::Short4 => miniquad::VertexFormat::Short4,
            vertex::VertexFormat::Int1 => miniquad::VertexFormat::Int1,
            vertex::VertexFormat::Int2 => miniquad::VertexFormat::Int2,
            vertex::VertexFormat::Int3 => miniquad::VertexFormat::Int3,
            vertex::VertexFormat::Int4 => miniquad::VertexFormat::Int4,
            vertex::VertexFormat::Mat4 => miniquad::VertexFormat::Mat4,
        }
    }
}


impl ConvertRef for shader::ShaderMeta
{
    type Output<'a> = miniquad::ShaderMeta;

    fn convert<'a>(&'a self) -> Self::Output<'a> {
        let Self { uniforms, images } = self;
        miniquad::ShaderMeta 
        { 
            uniforms: uniforms.convert(), 
            images: images.iter().map(|v| v.clone()).collect() 
        }
    }
}


impl ConvertRef for shader::UniformBlockLayout
{
    type Output<'a> = miniquad::UniformBlockLayout;

    fn convert<'a>(&'a self) -> Self::Output<'a> {
        let Self { uniforms } = self;
        miniquad::UniformBlockLayout 
        { 
            uniforms: uniforms.iter().map(|v| v.convert()).collect() 
        }
    }
}

impl ConvertRef for shader::UniformDesc
{
    type Output<'a> = miniquad::UniformDesc;

    fn convert<'a>(&'a self) -> Self::Output<'a> 
    {
        let Self { name, uniform_type, nb } = self;
        miniquad::UniformDesc { name : name.clone(), uniform_type: uniform_type.convert(), array_count: *nb }
    }
}


impl Convert for shader::UniformType
{
    type Output = miniquad::UniformType;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            shader::UniformType::Float1 => miniquad::UniformType::Float1,
            shader::UniformType::Float2 => miniquad::UniformType::Float2,
            shader::UniformType::Float3 => miniquad::UniformType::Float3,
            shader::UniformType::Float4 => miniquad::UniformType::Float4,
            shader::UniformType::Int1 => miniquad::UniformType::Int1,
            shader::UniformType::Int2 => miniquad::UniformType::Int2,
            shader::UniformType::Int3 => miniquad::UniformType::Int3,
            shader::UniformType::Int4 => miniquad::UniformType::Int4,
            shader::UniformType::Mat4 => miniquad::UniformType::Mat4,
        }
    }
}

impl Convert for shader::ShaderType
{
    type Output = miniquad::ShaderType;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            shader::ShaderType::Vertex => miniquad::ShaderType::Vertex,
            shader::ShaderType::Fragment => miniquad::ShaderType::Fragment,
        }
    }
}

impl Convert for miniquad::ShaderType
{
    type Output = shader::ShaderType;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            miniquad::ShaderType::Vertex => shader::ShaderType::Vertex,
            miniquad::ShaderType::Fragment => shader::ShaderType::Fragment,
        }
    }
}

impl Convert for miniquad::ShaderError
{
    type Output = shader::ShaderError;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            miniquad::ShaderError::CompilationError { shader_type, error_message } => 
                shader::ShaderError::CompilationError { shader_type: shader_type.convert(), error_message },
            miniquad::ShaderError::LinkError(v) => 
                shader::ShaderError::LinkError(v),
            miniquad::ShaderError::FFINulError(v) => 
                shader::ShaderError::FFINulError(v),
        }
    }
}

impl Convert for render_pass::PassAction
{
    type Output = miniquad::PassAction;
    fn convert(self) -> Self::Output 
    {
        match self
        {
            render_pass::PassAction::Nothing => miniquad::PassAction::Nothing,
            render_pass::PassAction::Clear(clear_data) => 
            miniquad::PassAction::Clear
            {
                color: clear_data.color.map(|c| c.convert()),
                depth: clear_data.depth,
                stencil: clear_data.stencil,
            },
        }
    }
}

impl Convert for Color
{
    type Output=(f32,f32,f32,f32);

    fn convert(self) -> Self::Output 
    {
        let [r,g,b,a] = self.into();
        (r as _, g as _, b as _, a as _)
    }
}

impl Convert for buffer::BufferType
{
    type Output=miniquad::BufferType;

    fn convert(self) -> Self::Output 
    {
        match self
        {
            buffer::BufferType::VertexBuffer => miniquad::BufferType::VertexBuffer,
            buffer::BufferType::IndexBuffer => miniquad::BufferType::IndexBuffer,
        }
    }
}

impl Convert for buffer::BufferUsage
{
    type Output=miniquad::BufferUsage;

    fn convert(self) -> Self::Output 
    {
        match self
        {
            buffer::BufferUsage::Immutable => miniquad::BufferUsage::Immutable,
            buffer::BufferUsage::Dynamic => miniquad::BufferUsage::Dynamic,
            buffer::BufferUsage::Stream => miniquad::BufferUsage::Stream,
        }
    }
}
