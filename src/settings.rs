//!
//! Platform abstraction for non-volatile storage of settings
//!

use crate::error::OTError;

pub enum OTSettingsKeys {
    // Active Operational Dataset
    ActiveDataset = 0x0001,
    // Pending Operational Dataset
    PendingDataset = 0x0002,
    // Thread network information
    NetworkInfo = 0x0003,
    // Parent information
    ParentInfo = 0x0004,
    // Child information
    ChildInfo = 0x0005,
    // SLAAC key to generate semantically opaque IID
    SlaacIIDSecretKey = 0x0007,
    // Duplicate Address Detection (DAD) information
    DADInfo = 0x0008,
    // SRP client ECDSA public/private key pair
    SRPECDSAKey = 0x000B,
    // The SRP client info (selected SRP server address)
    SRPClientInfo = 0x000C,
    // The SRP server infor (UDP port)
    SRPServerInfo = 0x000D,
    // BR ULA prefix
    BRULAPrefix = 0x000F,
    // BR local on-link prefixes
    BROnLinkPrefixes = 0x0010,
    // Unique Border Agent/Router ID.
    BorderAgentId = 0x0011,

    // Deprecated and reserved key values:
    //
    //  0x0006 previously auto-start.
    //  0x0009 previously OMR prefix.
    //  0x000A previously on-link prefix.
    //  0x000E previously NAT64 prefix.

    // Keys in range 0x8000-0xFFFF are reserved for vendor-specific use.
    VendorReservedMin = 0x8000,
    VendorReservedMax = 0xFFFF,
}

pub trait OTSettings {
    type Error;

    /// Performs any initialization for the settings subsystem, if necessary.
    /// 
    /// Also sets the sensitive keys that should be stored in the secure area
    fn init(&mut self, sensitive_keys: &[u16]) -> Result<(), Self::Error>;

    /// Performs any de-initialization for the settings subsystem, if necessary.
    fn de_init(&mut self) -> Result<(), Self::Error>;

    /// Fetches the value of a setting
    /// 
    /// Fetches the value of the setting identified by key
    /// and writes it to buffer provided.
    /// 
    /// Note: the underlying storage implementation is not required
    /// to maintain the order of settings with multiple values.
    /// The order of such values MAY change after ANY
    /// write operation to the store.
    fn get(
        &mut self,
        key: u16,
        index: usize,
        buffer: &mut [u8],
    ) -> Result<(), OTError<Self::Error>>;

    /// Sets or replaces the value of a setting.
    /// 
    /// Sets or replaces the value of a setting identified by key.
    /// 
    /// Calling this function successfully may cause unrelated
    /// settings with multiple values to be reordered.
    /// 
    /// OpenThread stack guarantees to use `otPlatSettingsSet()`
    /// method for a key that was either previously set using
    /// `otPlatSettingsSet()` (i.e., contains a single value) or
    /// is empty and/or fully deleted (contains no value).
    /// 
    /// Platform layer can rely and use this fact for optimizing
    /// its implementation
    fn set(
        &mut self,
        key: u16,
        value: &[u8]
    ) -> Result<(), OTError<Self::Error>>;

    /// Adds a value to a setting.
    ///
    /// Adds the value to a setting
    /// identified by @p aKey, without replacing any existing
    /// values.
    ///
    /// Note that the underlying implementation is not required
    /// to maintain the order of the items associated with a
    /// specific key. The added value may be added to the end,
    /// the beginning, or even somewhere in the middle. The order
    /// of any pre-existing values may also change.
    ///
    /// Calling this function successfully may cause unrelated
    /// settings with multiple values to be reordered.
    ///
    /// OpenThread stack guarantees to use `otPlatSettingsAdd()`
    /// method for a @p aKey that was either previously managed by
    /// `otPlatSettingsAdd()` (i.e., contains one or more items) or
    /// is empty and/or fully deleted (contains no value).
    ///
    /// Platform layer can rely and use this fact for optimizing
    /// its implementation.
    fn add(
        &mut self,
        key: u16,
        value: &[u8]
    ) -> Result<(), OTError<Self::Error>>;

    /// Removes a setting from the setting store.
    /// 
    /// Deletes a specific value from the
    /// setting identified by key from the settings store.
    /// 
    /// Note: the underlying implementation is not required
    /// to maintain the order of the items associated with a specific key.
    fn delete(
        &mut self,
        key: u16,
        index: usize,
    ) -> Result<(), OTError<Self::Error>>;

    /// Removes all settings from the setting store.
    /// 
    /// Deletes all settings from the settings
    /// store, resetting it to its initial factory state.
    fn wipe(&mut self) -> Result<(), Self::Error>;
}