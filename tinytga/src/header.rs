use crate::parse_error::ParseError;
use nom::{
    bytes::complete::take,
    combinator::map_res,
    number::complete::{le_u16, le_u8},
    IResult,
};

/// TGA footer length in bytes
pub const HEADER_LEN: usize = 18;

/// Image type
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ImageType {
    /// Image contains no pixel data
    Empty = 0,

    /// Color mapped image
    ColorMapped = 1,

    /// Truecolor image
    Truecolor = 2,

    /// Monochrome (greyscale) image
    Monochrome = 3,

    /// Run length encoded color mapped image
    RleColorMapped = 9,

    /// Run length encoded RGB image
    RleTruecolor = 10,

    /// Run length encoded monochrome (greyscale) image
    RleMonochrome = 11,
}

/// TGA header structure, referenced from <https://www.fileformat.info/format/tga/egff.htm>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TgaHeader {
    /// Image ID field length
    pub id_len: u8,

    /// Whether a color map is included in the image data
    pub has_color_map: bool,

    /// Image type
    pub image_type: ImageType,

    /// Color map origin
    pub color_map_start: u16,

    /// Length of color map
    pub color_map_len: u16,

    /// Number of bits in each color palette entry, typically 15, 16, 24, or 32 bits
    pub color_map_depth: u8,

    /// Image origin (X)
    pub x_origin: u16,

    /// Image origin (Y)
    pub y_origin: u16,

    /// Image width in pixels
    pub width: u16,

    /// Image height in pixels
    pub height: u16,

    /// Pixel bit depth (8, 16, 24, 32 bits)
    pub pixel_depth: u8,

    /// Image descriptor (unused)
    ///
    /// Bits 0:3: Number of bits per pixel designated to alpha channel
    /// Bits 4:5: Image origin:
    ///
    /// * `00` = bottom left
    /// * `01` = bottom right
    /// * `10` = top left
    /// * `11` = top right
    pub image_descriptor: u8,
}

fn has_color_map(input: &[u8]) -> IResult<&[u8], bool> {
    map_res(le_u8, |b| match b {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(ParseError::UnknownColorMap(other)),
    })(input)
}

fn image_type(input: &[u8]) -> IResult<&[u8], ImageType> {
    map_res(le_u8, |b| match b {
        0 => Ok(ImageType::Empty),
        1 => Ok(ImageType::ColorMapped),
        2 => Ok(ImageType::Truecolor),
        3 => Ok(ImageType::Monochrome),
        9 => Ok(ImageType::RleColorMapped),
        10 => Ok(ImageType::RleTruecolor),
        11 => Ok(ImageType::RleMonochrome),
        other => Err(ParseError::UnknownImageType(other)),
    })(input)
}

pub fn header(input: &[u8]) -> IResult<&[u8], TgaHeader> {
    let (input, id_len) = le_u8(input)?;
    let (input, has_color_map) = has_color_map(input)?;
    let (input, image_type) = image_type(input)?;
    let (input, color_map_start) = le_u16(input)?;
    let (input, color_map_len) = le_u16(input)?;
    let (input, color_map_depth) = le_u8(input)?;
    let (input, x_origin) = le_u16(input)?;
    let (input, y_origin) = le_u16(input)?;
    let (input, width) = le_u16(input)?;
    let (input, height) = le_u16(input)?;
    let (input, pixel_depth) = le_u8(input)?;
    let (input, image_descriptor) = le_u8(input)?;
    let (input, _image_ident) = take(id_len)(input)?;

    Ok((
        input,
        TgaHeader {
            id_len,
            has_color_map,
            image_type,
            color_map_start,
            color_map_len,
            color_map_depth,
            x_origin,
            y_origin,
            width,
            height,
            pixel_depth,
            image_descriptor,
        },
    ))
}
