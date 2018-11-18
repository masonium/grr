use __gl;

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum Format {
    R8G8B8A8_SRGB = __gl::SRGB8_ALPHA8,
    // TODO
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum BaseFormat {
    R = __gl::RED,
    RG = __gl::RG,
    RGB = __gl::RGB,
    RGBA = __gl::RGBA,
    // TODO
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum FormatLayout {
    U8 = __gl::UNSIGNED_BYTE,
    I8 = __gl::BYTE,
    // TODO
}
