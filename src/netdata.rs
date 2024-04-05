//!
//! OpenThread Network Data API
//! 

pub const OT_NETWORK_DATA_ITERATOR_INIT: u32 = 0;

/// Used to iterate through Network Data Information
type OTNetworkDataIterator = u32;

/// Border Router Configuration
pub struct OTBorderRouterConfig {
    prefix: OTIp6Prefix,
}