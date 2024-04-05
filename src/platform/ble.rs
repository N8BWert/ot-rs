//!
//! Platform Abstraction for BLE Host communication. 
//! 
//! Note: The platform needs to implement Bluetooth LE 4.2 or higher
//! 

/// Time slot duration on PHY layer in microseconds (0.625ms)
pub const OT_BLE_TIMESLOT_UNIT: u32 = 625;
/// Minimum allowed interval for advertising packet in OT_BLE_ADV_INTERVAL_UNIT units (20ms)
pub const OT_BLE_ADV_INTERVAL_MIN: u32 = 0x0020;
/// Maximum allowed interval for advertising packet in OT_BLE_ADV_INTERVAL_UNIT units (10.24s)
pub const OT_BLE_ADV_INTERVAL_MAX: u32 = 0x4000;
/// Default interval for advertising packet (ms)
pub const OT_BLE_ADV_INTERVAL_DEFAULT: u32 = 100;
/// Maximum allowed ATT MTU size (must be >= 23)
pub const OT_BLE_ATT_MTU_MAX: u32 = 67;
/// Default power value for BLE
pub const OT_BLUE_DEFAULT_POWER: u8 = 0;

/// BLE packet
pub struct OTBLERadioPacket<'a> {
    pub value: &'a [u8],
    pub power: u8,
}

/// Platform Implementation for Bluetooth Low Energy Radio
pub trait OTBLERadio {
    type Error;

    /// Enable the Bluetooth Low Energy Radio
    /// 
    /// NOTE: BLE Device should use the highest ATT_MTU supported that does not
    /// exceed OT_BLE_ATT_MTU_MAX octets.
    fn ble_enable(&mut self) -> Result<(), Self::Error>;

    /// Disable the Bluetooth Low Energy Radio
    /// 
    /// When disabled, the BLE stack will flush event queues and not generate new
    /// events. The BLE peripheral is turned off or put into a low power sleep
    /// state. Any dynamic memory used by the stack should be released,
    /// but static memory may remain reserved.
    fn ble_disable(&mut self) -> Result<(), Self::Error>;

    /// Starts BLE Advertising procedure.
    ///
    /// The BLE device shall use undirected advertising with no filter applied.
    /// A single BLE Advertising packet must be sent on all advertising
    /// channels (37, 38 and 39).
    fn ble_start_advertising(&mut self, interval: u16) -> Result<(), Self::Error>;

    /// Stops BLE Advertising procedure.
    ///
    /// Note: This function shall be used only for BLE Peripheral role.
    fn ble_stop_advertising(&mut self) -> Result<(), Self::Error>;

    /// Disconnects BLE connection.
    ///
    /// The BLE device shall use the Remote User Terminated Connection (0x13) reason
    /// code when disconnecting from the peer BLE device..
    fn ble_disconnect(&mut self) -> Result<(), Self::Error>;

    /// Reads currently used value of ATT_MTU
    fn ble_get_att(&mut self) -> Result<u16, Self::Error>;

    /// Sends ATT Handle Indication
    /// 
    /// Note: This function shall be used only for GATT server
    fn ble_server_indicate(&mut self, handle: u16, packet: &OTBLERadioPacket) -> Result<(), Self::Error>;
}

/// Trait for a BLE Radio Handler
pub trait OTBLERadioHandler {
    type Error;

    /// The BLE driver calls this method to notify OpenThread that a BLE Central Device has
    /// been connected.
    fn ble_on_connected(&mut self, connection_id: u16) -> Result<(), Self::Error>;

    /// The BLE driver calls this method to notify OpenThread that the BLE Central Device
    /// has been disconnected.
    fn ble_on_disconnected(&mut self, connection_id: u16) -> Result<(), Self::Error>;

    /// The BLE driver calls this method to notify OpenThread that ATT_MTU has been updated
    fn ble_mtu_update(&mut self, mtu: u16) -> Result<(), Self::Error>;

    /// The BLE driver calls this method to notify OpenThread that an ATT Write Request
    /// packet has been received.
    fn ble_server_on_write_request(&mut self, handle: u16, packet: &OTBLERadioPacket) -> Result<(), Self::Error>;
}