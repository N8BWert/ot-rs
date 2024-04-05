//!
//! OpenThread Backbone Router API (Thread 1.2)
//! 

use crate::OTInstance;

/// Backbone Router Configuration.
pub struct OTBackboneRouterConfig {
    // Only used when get Primary Backend Router information in the Thread Network
    pub server_16: u16,
    // Reregistration delay (in seconds)
    pub registration_delay: u16,
    // Multicast Listener Registration Timeout (in seconds)
    pub mlr_timeout: u32,
    // Sequence number
    pub sequence_number: u8,
}

/// Backbone Router API
pub trait OTBackboneRouter {
    type Error;

    /// Gets the primary backbone router information in the Thread Network
    fn get_primary_backbone_router_config(
        &mut self,
        openthread: &mut OTInstance,
    ) -> Result<OTBackboneRouterConfig, Self::Error>;
}