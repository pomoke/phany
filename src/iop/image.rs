//! Image container and processor definition

use anyhow::Result;

pub trait Primitive: Copy {}
pub trait IntPrimitive: Primitive {}
pub trait FloatPrimitive: Primitive {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}

impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}

impl Primitive for f32 {}
impl Primitive for f64 {}

impl FloatPrimitive for f32 {}
impl FloatPrimitive for f64 {}

pub trait Pixel: Clone {}
impl<T: Primitive> Pixel for T {}
impl<T: Primitive> Pixel for (T, T) {}
impl<T: Primitive> Pixel for (T, T, T) {}
impl<T: Primitive> Pixel for (T, T, T, T) {}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ColorSpace {
    /// Linear value.
    Linear,
    RGB,
    HSL,
    LCH,
    LAB,
    JzAzBz,
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Encoding {
    Linear,
    Gamma(f32),
}

#[derive(Debug, Clone)]
pub struct Image<T: Pixel> {
    image: Vec<T>,
}

pub trait ImageOp<T: Pixel, U: Pixel> {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    fn input_format() -> &'static [()];
    fn output_format() -> &'static [()];

    fn pipe_inplace(&self, img: &mut Image<T>) -> Result<()>;
    fn pipe_inplace_fast(&self, img: &mut Image<T>) -> Result<()>;
    fn pipe(&self, img: &mut Image<T>, out: &mut Image<U>);
    fn pipe_fast(&self, img: &mut Image<T>, out: &mut Image<U>);
}
