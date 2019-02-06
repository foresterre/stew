use arrayvec::ArrayVec;

#[cfg(test)]
mod mod_test_includes;

pub(crate) mod transformations;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Blur(f32),
    Brighten(i32),
    Contrast(f32),
    Crop(u32, u32, u32, u32),
    Filter3x3(ArrayVec<[f32; 9]>),
    FlipHorizontal,
    FlipVertical,
    GrayScale,
    HueRotate(i32),
    Invert,
    Resize(u32, u32),
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen(f32, i32),
}
