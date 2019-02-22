use arrayvec::ArrayVec;

#[cfg(test)]
mod mod_test_includes;

pub(crate) mod transformations;

#[derive(Debug, PartialEq, Clone)]
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

pub enum OpArg {
    Empty,
    FloatingPoint(f32),
    Integer(i32),
    UnsignedIntegerTuple2(u32, u32),
    UnsignedIntegerTuple4(u32, u32, u32, u32),
    FloatingPointArrayVec9(ArrayVec<[f32; 9]>),
    FloatingPointIntegerTuple2(f32, i32),
}

pub fn operation_by_name(name: &str, value: OpArg) -> Result<Operation, String> {
    match (name, value) {
        ("blur", OpArg::FloatingPoint(v)) => Ok(Operation::Blur(v)),
        ("brighten", OpArg::Integer(v)) => Ok(Operation::Brighten(v)),
        ("contrast", OpArg::FloatingPoint(v)) => Ok(Operation::Contrast(v)),
        ("crop", OpArg::UnsignedIntegerTuple4(u0, u1, u2, u3)) => {
            Ok(Operation::Crop(u0, u1, u2, u3))
        }
        ("filter3x3", OpArg::FloatingPointArrayVec9(v)) => Ok(Operation::Filter3x3(v)),
        ("fliph", OpArg::Empty) => Ok(Operation::FlipHorizontal),
        ("flipv", OpArg::Empty) => Ok(Operation::FlipVertical),
        ("grayscale", OpArg::Empty) => Ok(Operation::GrayScale),
        ("huerotate", OpArg::Integer(v)) => Ok(Operation::HueRotate(v)),
        ("invert", OpArg::Empty) => Ok(Operation::GrayScale),
        ("resize", OpArg::UnsignedIntegerTuple2(u0, u1)) => Ok(Operation::Resize(u0, u1)),
        ("rotate90", OpArg::Empty) => Ok(Operation::Rotate90),
        ("rotate180", OpArg::Empty) => Ok(Operation::Rotate180),
        ("rotate270", OpArg::Empty) => Ok(Operation::Rotate270),
        ("unsharpen", OpArg::FloatingPointIntegerTuple2(f, i)) => Ok(Operation::Unsharpen(f, i)),
        _ => Err("No suitable operation was found.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Blur
    // ----------

    #[test]
    fn blur_ok() {
        let actual = operation_by_name("blur", OpArg::FloatingPoint(1.5));

        assert_eq!(actual, Ok(Operation::Blur(1.5)));
    }

    #[test]
    fn blur_name_err() {
        let actual = operation_by_name("blur'", OpArg::FloatingPoint(1.5));

        assert_ne!(actual, Ok(Operation::Blur(1.5)));
    }

    #[test]
    fn blur_arg_err() {
        let actual = operation_by_name("blur", OpArg::Empty);

        assert_ne!(actual, Ok(Operation::Blur(1.5)));
    }

    // Brighten
    // ----------
}
