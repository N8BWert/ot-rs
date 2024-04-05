//!
//! OpenThread Flash Driver Traits
//! 

pub trait OTFlash {
    type Error;

    /// Initializes the flash driver
    fn init(&mut self) -> Result<(), Self::Error>;

    /// Gets the size of the swap space (in bytes)
    fn get_swap_size(&mut self) -> Result<u32, Self::Error>;

    /// Erases the swap space indicated by swap_index
    fn flash_erase(&mut self, swap_index: u8) -> Result<(), Self::Error>;

    /// Reads bytes from the flash driver to a buffer
    fn flash_read(&mut self, swap_index: u8, offset: u32, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Write bytes from a buffer to the flash driver
    fn flash_write(&mut self, swap_index: u8, offset: u32, buffer: &[u8]) -> Result<(), Self::Error>;
}