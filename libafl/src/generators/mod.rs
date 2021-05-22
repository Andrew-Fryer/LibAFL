//! Generators may generate bytes or, in general, data, for inputs.

use alloc::vec::Vec;
use core::{cmp::min, marker::PhantomData};

use crate::{
    bolts::rands::Rand,
    inputs::{bytes::BytesInput, Input},
    Error,
};

/// The maximum size of dummy bytes generated by _dummy generator methods
const DUMMY_BYTES_MAX: usize = 64;

/// Generators can generate ranges of bytes.
pub trait Generator<I, R>
where
    I: Input,
    R: Rand,
{
    /// Generate a new input
    fn generate(&mut self, rand: &mut R) -> Result<I, Error>;

    /// Generate a new dummy input
    fn generate_dummy(&self) -> I;
}

#[derive(Clone, Debug)]
/// Generates random bytes
pub struct RandBytesGenerator<R>
where
    R: Rand,
{
    max_size: usize,
    phantom: PhantomData<R>,
}

impl<R> Generator<BytesInput, R> for RandBytesGenerator<R>
where
    R: Rand,
{
    fn generate(&mut self, rand: &mut R) -> Result<BytesInput, Error> {
        let mut size = rand.below(self.max_size as u64);
        if size == 0 {
            size = 1;
        }
        let random_bytes: Vec<u8> = (0..size).map(|_| rand.below(256) as u8).collect();
        Ok(BytesInput::new(random_bytes))
    }

    /// Generates up to `DUMMY_BYTES_MAX` non-random dummy bytes (0)
    fn generate_dummy(&self) -> BytesInput {
        let size = min(self.max_size, DUMMY_BYTES_MAX);
        BytesInput::new(vec![0; size])
    }
}

impl<R> RandBytesGenerator<R>
where
    R: Rand,
{
    /// Returns a new [`RandBytesGenerator`], generating up to `max_size` random bytes.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            phantom: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
/// Generates random printable characters
pub struct RandPrintablesGenerator<R> {
    max_size: usize,
    phantom: PhantomData<R>,
}

impl<R> Generator<BytesInput, R> for RandPrintablesGenerator<R>
where
    R: Rand,
{
    fn generate(&mut self, rand: &mut R) -> Result<BytesInput, Error> {
        let mut size = rand.below(self.max_size as u64);
        if size == 0 {
            size = 1;
        }
        let printables = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz \t\n!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes();
        let random_bytes: Vec<u8> = (0..size).map(|_| *rand.choose(printables)).collect();
        Ok(BytesInput::new(random_bytes))
    }

    /// Generates up to `DUMMY_BYTES_MAX` non-random dummy bytes (0)
    fn generate_dummy(&self) -> BytesInput {
        let size = min(self.max_size, DUMMY_BYTES_MAX);
        BytesInput::new(vec![0_u8; size])
    }
}

impl<R> RandPrintablesGenerator<R>
where
    R: Rand,
{
    /// Creates a new [`RandPrintablesGenerator`], generating up to `max_size` random printable characters.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            phantom: PhantomData,
        }
    }
}
