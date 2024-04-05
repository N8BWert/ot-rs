//!
//! OpenThread Border Router API
//! 

use crate::OTInstance;

/// API to manage local network data with OpenThread Border Router.
pub trait OTBorderRouter {
    type Error;

    /// Provides a full or stable copy of the local Thread Network Data, returning
    /// the length of data in the new buffer
    fn get_network_data(
        &mut self,
        openthread: &mut OTInstance,
        stable: bool,
        buffer: &mut [u8],
    ) -> Result<usize, Self::Error>;

    /// Add a border router configuration to the local network data
    fn add_on_mesh_prefix(
        &mut self,
        openthread: &mut OTInstance,
        config: &OTBorderRouterConfig,
    ) -> Result<(), Self::Error>;
}