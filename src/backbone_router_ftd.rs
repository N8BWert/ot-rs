//!
//! OpenThread Backbone Router API (for Thread 1.2 FTD with
//! backbone router enabled)
//! 

use crate::{backbone_router::OTBackboneRouterConfig, OTInstance};

pub enum OTBackboneRouterState {
    Disabled = 0,
    Secondary = 1,
    Primary = 2,
}

/// OpenThread Backbone Border Router FTE API
pub trait OTBackboneRouterFTE {
    type Error;

    /// Enables or disables Backbone functionality.
    ///
    /// If enabled, a Server Data Request message `SRV_DATA.ntf` is triggered for the attached
    /// device if there is no Backbone Router Service in the Thread Network Data.
    ///
    /// If disabled, `SRV_DATA.ntf` is triggered if the Backbone Router is in the Primary state.
    fn enable_backbone_router(
        &mut self,
        openthread: &mut OTInstance,
        enable: bool,
    ) -> Result<(), Self::Error>;

    /// Gets the backbone router state
    fn get_backbone_router_state(
        &mut self,
        openthread: &mut OTInstance,
    ) -> Result<OTBackboneRouterState, Self::Error>;

    /// Get the local backbone router configuration
    fn get_backbone_router_config(
        &mut self,
        openthread: &mut OTInstance,
    ) -> Result<OTBackboneRouterConfig, Self::Error>;

    /// Sets the local Backbone Router configuration #otBackboneRouterConfig.
    ///
    /// A Server Data Request message `SRV_DATA.ntf` is initiated automatically if BBR Dataset changes for Primary
    /// Backbone Router.
    fn set_backbone_router_config(
        &mut self,
        openthread: &mut OTInstance,
        config: OTBackboneRouterConfig,
    ) -> Result<(), Self::Error>;

    /// Explicitly registers local Backbone Router configuration.
    ///
    /// A Server Data Request message `SRV_DATA.ntf` is triggered for the attached device.
    fn register_backbone_router(
        &mut self,
        openthread: &mut OTInstance,
    ) -> Result<(), Self::Error>;

    /// Returns the Backbone Router registration jitter value.
    fn get_backbone_router_jitter(
        &mut self,
        openthread: &mut OTInstance
    ) -> Result<u8, Self::Error>;

    /// Sets the Backbone Router registration jitter value.
    fn set_backbone_router_jitter(
        &mut self,
        openthread: &mut OTInstance,
        jitter: u8
    ) -> Result<(), Self::Error>;

    /// Get the local Domain Prefix configuration
    fn get_backbone_router_domain_prefix(
        &mut self,
        openthread: &mut OTInstance,
        config: &mut OTBorderRouterConfig,
    ) -> Result<(), Self::Error>;
}