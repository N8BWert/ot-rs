//!
//! Platform abstraction for entropy generation
//! 

pub trait OTEntropy {
    type Error;

    fn get_entropy(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;
}