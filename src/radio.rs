//!
//! OpenThread Radio Traits
//! 

use core::ops::BitOr;

use alloc::string::String;

use crate::{crypto::OTCryptoKeyRef, error::OTError};

// aMaxPHYPacketSize (IEEE 802.15.4-2006)
pub const OT_RADIO_FRAME_MAX_SIZE: usize = 127;
// Minimal size of frame FCS + CONTROL
pub const OT_RADIO_FRAME_MIN_SIZE: usize = 3;

// 2.4 GHz IEEE 802.15.4-2006
pub const OT_RADIO_SYMBOLS_PER_OCTET: usize = 2;
// 2.4 GHz IEEE 802.15.4 (bits per second)
pub const OT_RADIO_BIT_RATE: usize = 250_000;
// Number of bits per octet
pub const OT_RADIO_BITS_PER_OCTET: usize = 8;

// Per IEEE 802.15.4-2015, 12.3.3 Symbol rate:
// The O-QPSK PHY symbol rate shall be 25 ksymbol/s when operating in the 868 MHz band and 62.5 ksymbol/s when
// operating in the 780 MHz, 915 MHz, 2380 MHz, or 2450 MHz band
// The O-QPSK PHY symbol rate when operating in the 780MHz, 915MHz, 2380MHz, 2450Mhz
pub const OT_RADIO_SYMBOL_RATE: usize = 62500;
// Symbol duration time in unit of microseconds
pub const OT_RADIO_SYMBOL_TIME: usize = 1_000_000 * 1 / OT_RADIO_SYMBOL_RATE;
// Time for 10 symbols in unit of microseconds
pub const OT_RADIO_TEN_SYMBOLS_TIME: usize = 10 * OT_RADIO_SYMBOL_TIME;

// LQI measurement not supported
pub const OT_RADIO_LQI_NONE: usize = 0;
// Invalid or unknown RSSI value
pub const OT_RADIO_RSSI_INVALID: usize = 127;
// Invalid or unknown power value
pub const OT_RADIO_POWER_INVALID: usize = 127;

//< 2.4 GHz IEEE 802.15.4-2006
pub const OT_RADIO_CHANNEL_PAGE_0: u8 = 0;
//< 2.4 GHz IEEE 802.15.4-2006
pub const OT_RADIO_CHANNEL_PAGE_0_MASK: u8 = 1 << OT_RADIO_CHANNEL_PAGE_0;
//< 915 MHz IEEE 802.15.4-2006
pub const OT_RADIO_CHANNEL_PAGE_2: u8 = 2;
//< 915 MHz IEEE 802.15.4-2006
pub const OT_RADIO_CHANNEL_PAGE_2_MASK: u8 = 1 << OT_RADIO_CHANNEL_PAGE_2;

// 915 MHz IEEE 802.15.4-2006
pub const OT_RADIO_915MHZ_OQPSK_CHANNEL_MIN: u16 = 1;
// 915 MHz IEEE 802.15.4-2006
pub const OT_RADIO_915MHZ_OQPSK_CHANNEL_MAX: u16 = 10;
// 915 MHz IEEE 802.15.4-2006
pub const OT_RADIO_915MHZ_OQPSK_CHANNEL_MASK: u16 = 0x3ff << OT_RADIO_915MHZ_OQPSK_CHANNEL_MIN;
// 2.4 GHz IEEE 802.15.4-2006
pub const OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MIN: u16 = 11;
// 2.4 GHz IEEE 802.15.4-2006
pub const OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MAX: u16 = 26;
// 2.4 GHz IEEE 802.15.4-2006
pub const OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MASK: u16 = 0xffff << OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MIN;

pub enum Capabilities {
    None = 0,
    AckTimeout = 1 << 0,
    EnergyScan = 1 << 1,
    TransmitRetries = 1 << 2,
    CSMABackoff = 1 << 3,
    SleepToTx = 1 << 4,
    TransmitSec = 1 << 5,
    TransmitTiming = 1 << 6,
    ReceiveTiming = 1 << 7,
    RxOnWhenIdle = 1 << 8,
}

impl BitOr for Capabilities {
    type Output = OTRadioCapabilities;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u16) | (rhs as u16)
    }
}

pub type OTRadioCapabilities = u16;

// IEEE 802.16.5 Boradcast PAN ID
pub const OT_PANID_BROADCAST: u16 = 0xFFFF;

/// IEEE 802.16.5 PAN ID.
pub type OTPanId = u16;

/// IEEE 802.15.4 Short Address.
pub type OTShortAddress = u16;

/// Size of an IEEE 802.15.4 Extended Address (bytes)
pub const OT_EXT_ADDRESS_SIZE: usize = 8;

// Size of IE header in bytes
pub const OT_IE_HEADER_SIZE: usize = 2;
// Size of CSL IE content in bytes
pub const OT_CSL_IE_SIZE: usize = 4;
// Max length for header IE in ACK.
pub const OT_ACK_IE_MAX_SIZE: usize = 16;
// Max length of Link Metrics data in Vendor-Specific IE.
pub const OT_ENH_PROBING_IE_DATA_MAX_SIZE: usize = 2;
// Fixed CSL IE header first byte
pub const CSL_IE_HEADER_BYTES_LO: u8 = 0x04;
// Fixed CSL IE header second byte
pub const CSL_IE_HEADER_BYTES_HI: u8 = 0x0D;

/// IEEE 802.15.4 Extended Address.
pub type OTExtAddress = [u8; OT_EXT_ADDRESS_SIZE];

// Size of MAC KEY in bytes.
pub const OT_MAC_KEY_SIZE: usize = 16;

/// MAC Key
pub type OTMacKey = [u8; OT_MAC_KEY_SIZE];

/// MAK Key Ref used by PSA
pub type OTMacKeyRef = OTCryptoKeyRef;

/// Mak Key Representation
pub enum OTMacKeyMaterial {
    Key(OTMacKey),
    Ref(OTMacKeyRef),
}

/// Key Types for OpenThread
pub enum OTKeyType {
    LiteralKey = 0,
    ReferenceKey = 1,
}

/// IEEE 802.15.4 Header IE (Information Element) related information of a radio frame.
pub struct RadioIEInfo {
    // The time offset to the Thread network time.
    pub network_time_offset: i64,
    // The Time IE offset from the start of PSDU.
    pub time_ie_offset: u8,
    // The Time sync sequence.
    pub time_sync_sequency: u8
}

pub struct TransmitFrame {

}

pub struct ReceiveFrame {

}

pub enum OTFrameInformation<'a> {
    TxInfo {
        // The key material used for AES-CCM frame security
        aes_key: OTMacKeyMaterial,
        // The pointer to the Header IE(s) related information
        io_info: &'a RadioIEInfo,

        // The base time in microseconds for scheduled transmissions
        // relative to the local radio clock, see `otPlatRadioGetNow` and
        // `mTxDelay`.
        tx_delay_base_time: u32,

        // The delay time in microseconds for this transmission referenced
        // to `tx_delay_base_time`.
        //
        // Note: `tx_delay_base_time` + `tx_delay` SHALL piont to the point in
        // time when the end of the SFD will be present at the local
        // antenna, relative to the local radio clock.
        tx_delay: u32,

        // Maximum number of backoffs attempted before declaring CCA failure
        max_csma_backoffs: u8,
        // Maximum number of retries allowed after a transmission failure.
        max_frame_retries: u8,

        // The RX channel after frame TX is done (after all frame retries - ack received, or timeout, or abort).
        //
        // Radio platforms can choose to fully ignore this. OT stack will make sure to call `otPlatRadioReceive()`
        // with the desired RX channel after a frame TX is done and signaled in `otPlatRadioTxDone()` callback.
        // Radio platforms that don't provide `OT_RADIO_CAPS_TRANSMIT_RETRIES` must always ignore this.
        //
        // This is intended for situations where there may be delay in interactions between OT stack and radio, as
        // an example this is used in RCP/host architecture to make sure RCP switches to PAN channel more quickly.
        // In particular, this can help with CSL tx to a sleepy child, where the child may use a different channel
        // for CSL than the PAN channel. After frame tx, we want the radio/RCP to go back to the PAN channel
        // quickly to ensure that parent does not miss tx from child afterwards, e.g., child responding to the
        // earlier CSL transmitted frame from parent using PAN channel while radio still staying on CSL channel.
        //
        // The switch to the RX channel MUST happen after the frame TX is fully done, i.e., after all retries and
        // when ack is received (when "Ack Request" flag is set on the TX frame) or ack timeout. Note that ack is
        // expected on the same channel that frame is sent on.
        rx_channel_after_tx_done: u8,

        // Indicates whether frame counter and CSL IEs are properly updated in the header.
        //
        // If the platform layer does not provide `OT_RADIO_CAPS_TRANSMIT_SEC` capability, it can ignore this flag.
        //
        // If the platform provides `OT_RADIO_CAPS_TRANSMIT_SEC` capability, then platform is expected to handle tx
        // security processing and assignment of frame counter. In this case the following behavior is expected:
        //
        // When `mIsHeaderUpdated` is set, it indicates that OpenThread core has already set the frame counter and
        // CSL IEs (if security is enabled) in the prepared frame. The counter is ensured to match the counter value
        // from the previous attempts of the same frame. The platform should not assign or change the frame counter
        // (but may still need to perform security processing depending on `mIsSecurityProcessed` flag).
        //
        // If `mIsHeaderUpdated` is not set, then the frame counter and key CSL IE not set in the frame by
        // OpenThread core and it is the responsibility of the radio platform to assign them. The platform
        // must update the frame header (assign counter and CSL IE values) before sending the frame over the air,
        // however if the the transmission gets aborted and the frame is never sent over the air (e.g., channel
        // access error) the platform may choose to not update the header. If the platform updates the header,
        // it must also set this flag before passing the frame back from the `otPlatRadioTxDone()` callback.
        is_header_updated: bool,
        // Indicates whether the frame is a retransmission or not.
        is_a_retx: bool,
        // Set to true to enable CSMA-CA for this packet, false otherwise.
        csma_ca_enabled: bool,
        // Set to true if CSL header IE is present.
        csl_present: bool,
        // True if SubMac should skip the AES processing of this frame.
        is_security_processed: bool,
    },
    RxInfo {
        // The time of the local radio clock in microseconds when the end of
        // the SFD was present at the local antenna.
        timestamp: u64,
        // ACK security frame counter (applicable when `acked_with_sec_enh_ack` is set)
        ack_frame_counter: u32,
        // ACK security key index (applicable when `acked_with_sec_enh_ack` is set)
        ack_key_id: u8,
        // Received signal strength indicator in dBm for received frames.
        rssi: i8,
        // Link Quality Indicator for received frames.
        lqi: u8,
        
        // Flags
        // This indicates if this frame was acknowledged with frame pending set.
        acked_with_frame_pending: bool,
        acked_with_sec_enh_ack: bool,
    }
}

/// IEEE 802.15.4 radio frame
pub struct OTRadioFrame<'a> {
    // The PSDU
    pub psdu: &'a [u8],
    // Channel used to transmit/receive the frame
    pub channel: u8,
    // Radio link type (ignored by radio driver)
    pub radio_type: u8,
    // transmit and receive information for a radio frame
    pub frame_information: OTFrameInformation<'a>,
}

/// Representation of the state of a radio.
/// Initially, a radio is in the Disabled state.
pub enum OTRadioState {
    Disabled = 0,
    Sleep = 1,
    Receive = 2,
    Transmit = 3,
    Invalid = 255,
}

// The following are valid radio state transitions:
//
//                                    (Radio ON)
//  +----------+  Enable()  +-------+  Receive() +---------+   Transmit()  +----------+
//  |          |----------->|       |----------->|         |-------------->|          |
//  | Disabled |            | Sleep |            | Receive |               | Transmit |
//  |          |<-----------|       |<-----------|         |<--------------|          |
//  +----------+  Disable() +-------+   Sleep()  +---------+   Receive()   +----------+
//                                    (Radio OFF)                 or
//                                                        signal TransmitDone
//
// During the IEEE 802.15.4 data request command the transition Sleep->Receive->Transmit
// can be shortened to direct transition from Sleep to Transmit if the platform supports
// the OT_RADIO_CAPS_SLEEP_TO_TX capability.

/// Radio Coexistence Metrics.
pub struct OTRadioCoexMetrics {
    // Number of grant glitches.
    pub num_grant_glitch: u32,
    // Number of tx requests.
    pub num_tx_request: u32,
    // Number of tx requests while grant was active.
    pub num_tx_grant_immediate: u32,
    // Number of tx requests while grant was inactive.
    pub num_tx_grant_wait: u32,
    // Number of tx requests while grant was inactive that were ultimately granted.
    pub num_tx_grant_wait_activated: u32,
    // Number of tx requests while grant was inactive that timed out.
    pub num_tx_grant_wait_timeout: u32,
    // Number of tx that were in progress when grant was deactivated.
    pub num_tx_grant_deactivated_during_request: u32,
    // Number of tx requests that were not granted within 50us.
    pub num_tx_delayed_grant: u32,
    // Average time in usec from tx request to grant.
    pub avg_tx_request_to_grant_time: u32,
    // Number of rx requests.
    pub num_rx_request: u32,
    // Number of rx requests while grant was active.
    pub num_rx_grant_immediate: u32,
    // Number of rx requests while grant was inactive.
    pub num_rx_grant_wait: u32,
    // Number of rx requests while grant was inactive that were ultimately granted.
    pub num_rx_grant_wait_activated: u32,
    // Number of rx requests while grant was inactive that timed out.
    pub num_rx_grant_wait_timeout: u32,
    // Number of rx that were in progress when grant was deactivated.
    pub num_rx_grant_deactivated_during_request: u32,
    // Number of rx requests that were not granted within 50us.
    pub num_rx_delayed_grant: u32,
    // Average time in usec from rx request to grant.
    pub avg_rx_request_to_grant_time: u32,
    // Number of rx requests that completed without receiving grant.
    pub num_rx_grant_none: u32,
    // Stats collection stopped due to saturation.
    pub stopped: bool,
}

/// Metrics specified to query
pub struct OTLinkMetrics {
    pub pdu_count: bool,
    pub lqi: bool,
    pub link_margin: bool,
    pub rssi: bool,
    pub reserved: bool,
}

/// Platform abstraction for radio configuration
pub trait OTRadioConfiguration {
    type Error;

    /// Get the Radio's Capabilities
    fn radio_capabilities(&mut self) -> Result<OTRadioCapabilities, Self::Error>;

    /// Get the radio's receive sensitivity value.
    fn radio_receive_sensitivity(&mut self) -> Result<u8, Self::Error>;

    /// Get the factory-assigned IEEE EUI-64 for the radio
    fn radio_ieee_eui_64(&mut self) -> Result<[u8; 8], Self::Error>;

    /// Set the PAN ID for address filtering
    fn set_pan_id(&mut self, pan_id: OTPanId) -> Result<(), Self::Error>;

    /// Set the extended address for address filtering
    fn set_extended_address(&mut self, address: OTExtAddress) -> Result<(), Self::Error>;

    /// Set the short address for address filtering
    fn set_short_address(&mut self, address: OTShortAddress) -> Result<(), Self::Error>;

    /// Get the radio's transmit power in dBm.
    /// 
    /// Note: the transmit power returned will be no larger than the power specified in the max power table for
    /// the current channel.
    fn get_transmit_power(&mut self) -> Result<i8, OTError<Self::Error>>;

    /// Set the radio's transmit power in dBm.
    /// 
    /// Note: the real transmit power will be no larger than the power specified in the max power table for
    /// the current channel
    fn set_transmit_power(&mut self, power: i8) -> Result<(), OTError<Self::Error>>;

    /// Get the radio's CCA ED threshold in dBm measured at antenna connector per IEEE 802.15.4 - 2015 section 10.1.4.
    fn get_cca_energy_detect_threshold(&mut self) -> Result<i8, OTError<Self::Error>>;

    /// Set the radio's CCA ED threshold in dBm measured at antenna connector per IEEE 802.15.4 -2015 section 10.1.4.
    fn set_cca_energy_detect_threshold(&mut self, threshold: i8) -> Result<(), OTError<Self::Error>>;

    /// Get the external REM's Rx LNA gain in dBm.
    fn get_fem_lna_gain(&mut self) -> Result<i8, OTError<Self::Error>>;

    /// Set the external FEM's Rx LNA gain in dBm.
    fn set_fem_lna_gain(&mut self, gain: i8) -> Result<(), OTError<Self::Error>>;

    /// Get the status of promiscuous mode
    fn get_promiscuous(&mut self) -> Result<bool, Self::Error>;

    /// Enable or disable promiscuous mode
    fn set_promiscuous(&mut self, enabled: bool) -> Result<(), Self::Error>;

    /// Sets the rx-on-when-idle state to the radio platform.
    ///
    /// There are a few situations that the radio can enter sleep state if the device is in rx-off-when-idle state but
    /// it's hard and costly for the SubMac to identify these situations and instruct the radio to enter sleep:
    ///
    /// - Finalization of a regular frame reception task, provided that:
    ///   - The frame is received without errors and passes the filtering and it's not an spurious ACK.
    ///   - ACK is not requested or transmission of ACK is not possible due to internal conditions.
    /// - Finalization of a frame transmission or transmission of an ACK frame, when ACK is not requested in the transmitted
    ///   frame.
    /// - Finalization of the reception operation of a requested ACK due to:
    ///   - ACK timeout expiration.
    ///   - Reception of an invalid ACK or not an ACK frame.
    ///   - Reception of the proper ACK, unless the transmitted frame was a Data Request Command and the frame pending bit
    ///     on the received ACK is set to true. In this case the radio platform implementation SHOULD keep the receiver on
    ///     until a determined timeout which triggers an idle period start.`OPENTHREAD_CONFIG_MAC_DATA_POLL_TIMEOUT` can be
    ///     taken as a reference for this.
    /// - Finalization of a stand alone CCA task.
    /// - Finalization of a CCA operation with busy result during CSMA/CA procedure.
    /// - Finalization of an Energy Detection task.
    /// - Finalization of a radio reception window scheduled with `otPlatRadioReceiveAt`.
    ///
    /// If a platform supports `OT_RADIO_CAPS_RX_ON_WHEN_IDLE` it must also support `OT_RADIO_CAPS_CSMA_BACKOFF` and handle
    /// idle periods after CCA as described above.
    fn set_rx_on_when_idle(&mut self, enabled: bool) -> Result<(), Self::Error>;

    /// Get the current time in microseconds referenced to a continuous monotonic
    /// local radio clock (64 bits width).
    ///
    /// The radio clock SHALL NOT wrap during the device's uptime. Implementations
    /// SHALL therefore identify and compensate for internal counter overflows. The
    /// clock does not have a defined epoch and it SHALL NOT introduce any continuous
    /// or discontinuous adjustments (e.g. leap seconds). Implementations SHALL
    /// compensate for any sleep times of the device.
    ///
    /// Implementations MAY choose to discipline the radio clock and compensate for
    /// sleep times by any means (e.g. by combining a high precision/low power RTC
    /// with a high resolution counter) as long as the exposed combined clock
    /// provides continuous monotonic microsecond resolution ticks within the
    /// accuracy limits announced by @ref otPlatRadioGetCslAccuracy.
    fn get_now(&mut self) -> u64;

    /// Get the bus speed in bits/second between the host and the radio chip
    /// 
    /// Returns:
    ///     bus_speed in bits/second
    fn get_bus_speed(&mut self) -> u32;
}

pub trait OTRadioConfigurationCapTransmit {
    type Error;

    /// Update MAC keys and key index
    /// 
    /// Is used when radio provides OT_RADIO_CAPS_TRANSMIT_SEC capability
    /// 
    /// Params:
    ///     key_id_mode (u8): The key ID mode
    ///     key_id (u8): Current MAC key index
    ///     previous_key (OTMacKeyMaterial): The previous MAC key
    ///     current_key (OTMacKeyMaterial): The current MAC key
    ///     next_key (OTMacKeyMaterial): The next MAC key.
    ///     key_type (OTKeyType): Key Type used.
    fn set_mac_key(
        &mut self,
        key_id_mode: u8,
        key_id: u8,
        previous_key: OTMacKeyMaterial,
        current_key: OTMacKeyMaterial,
        next_key: OTMacKeyMaterial,
        key_type: OTKeyType,
    ) -> Result<(), Self::Error>;

    /// Set the current MAC frame counter value.
    fn set_mac_frame_counter(&mut self, mac_frame_counter: u32) -> Result<(), Self::Error>;

    /// Set the current MAC frame counter value only if the new given value is larger than the current value.
    fn set_mac_frame_counter_if_larger(&mut self, mac_frame_counter: u32) -> Result<(), Self::Error>;
}

pub trait OTRadioConfigurationOptional {
    /// Get the radio version string.
    fn get_version(&self) -> String;
}

/// Traits Implementable for the Radio Module to make it OpenThread Compatible
pub trait OTRadioOperation {
    type Error;

    /// Enable the radio
    fn enable(&mut self) -> Result<(), OTError<Self::Error>>;

    /// Disable the radio
    fn disable(&mut self) -> Result<(), OTError<Self::Error>>;

    /// Check whether radio is enabled or not.
    fn is_enabled(&mut self) -> Result<bool, Self::Error>;

    /// Transition the radio from Receive to Sleep (turn off the radio).
    fn sleep(&mut self) -> Result<(), OTError<Self::Error>>;

    /// Transition the radio from Sleep to Receive (turn on the radio).
    fn receive(&mut self, channel: u8) -> Result<(), OTError<Self::Error>>;

    /// Schedule a radio reception window at a specific time and duration.
    fn receive_at(&mut self, channel: u8, start: u32, duration: u32) -> Result<(), OTError<Self::Error>>;

    /// Receive an OpenThread Frame from the radio
    fn receive_frame(&mut self) -> Result<OTRadioFrame, OTError<Self::Error>>;

    /// Begin the transmit sequence on the radio
    /// 
    /// The transmit sequence consists of:
    /// 1. Transitioning the radio to Transmit from one of the following states:
    ///     - Receive if RX is on when the device is idle or OT_RADIO_CAPS_SLEEP_TO_TX is not supported
    ///     - Sleep if RX is off when the device is idle and OT_RADIO_CAPS_SLEEP_TO_TX is supported
    /// 2. Transmits the psdu on the given channel and at the given transmit power.
    fn transmit(&mut self, frame: OTRadioFrame) -> Result<(), OTError<Self::Error>>;

    /// TODO: Find out where the following methods should go
    /// otPlatRadioTxStarted
    fn tx_started(&mut self);
    /// otPlatRadioTxDone
    fn tx_done(&mut self);
    // otPlatDiagRadioTransmitDone
    fn diag_tx_done(&mut self);

    /// Get the most recent RSSI measurement.
    fn get_rssi(&mut self) -> Result<i8, Self::Error>;

    /// Enable/Disable source address match feature.
    ///
    /// The source address match feature controls how the radio layer decides the "frame pending" bit for acks sent in
    /// response to data request commands from children.
    ///
    /// If disabled, the radio layer must set the "frame pending" on all acks to data request commands.
    ///
    /// If enabled, the radio layer uses the source address match table to determine whether to set or clear the "frame
    /// pending" bit in an ack to a data request command.
    ///
    /// The source address match table provides the list of children for which there is a pending frame. Either a short
    /// address or an extended/long address can be added to the source address match table.
    fn enable_src_match(&mut self, enabled: bool) -> Result<(), Self::Error>;

    /// Add a short address to the source address match table
    fn add_src_match_short_entry(&mut self, address: OTShortAddress) -> Result<(), OTError<Self::Error>>;

    /// Add an extended address to the source address match table
    fn add_src_match_ext_entry(&mut self, address: OTExtAddress) -> Result<(), OTError<Self::Error>>;

    /// Remove a short address from the source address match table
    fn clear_src_match_short_entry(&mut self, address: OTShortAddress) -> Result<(), OTError<Self::Error>>;

    /// Remove and extended address from the source address match table
    fn clear_src_match_ext_entry(&mut self, address: OTExtAddress) -> Result<(), OTError<Self::Error>>;

    /// Clear all short addresses from the source address match table.
    fn clear_src_match_short_entries(&mut self) -> Result<(), Self::Error>;

    /// Clear all extended/long addresses from the source address match table
    fn clear_src_match_ext_entries(&mut self) -> Result<(), Self::Error>;

    /// Get the radio supported channel mask that the device is allowed to be on
    fn get_supported_channel_mask(&mut self) -> Result<u32, Self::Error>;

    /// Get the radio preferred channel mask that the device prefers to form on
    fn get_preferred_channel_mask(&mut self) -> Result<u32, Self::Error>;

    /// Set the max transmit power for a specific channel
    fn set_channel_max_transmit_power(&mut self, channel: u8, max_power: u8) -> Result<(), Self::Error>;

    /// Set the region code
    /// 
    /// The radio region format is the 2-bytes ascii representation of the
    /// ISO 3166 alpha-2 code.
    fn set_region(&mut self, region_code: u16) -> Result<(), Self::Error>;

    /// Get the region code
    /// 
    /// The radio region format is the 2-bytes ascii representation of the
    /// ISO 3166 alpha-2 code.
    fn get_region(&mut self) -> Result<u16, Self::Error>;

    /// Enable/disable or update Enhanced-ACK Based Probing in radio for a specific Initiator.
    ///
    /// After Enhanced-ACK Based Probing is configured by a specific Probing Initiator, the Enhanced-ACK sent to that
    /// node should include Vendor-Specific IE containing Link Metrics data. This method informs the radio to start/stop to
    /// collect Link Metrics data and include Vendor-Specific IE that containing the data in Enhanced-ACK sent to that
    /// Probing Initiator.
    fn configure_enh_ack_probing(
        &mut self,
        link_metrics: OTLinkMetrics,
        short_address: OTShortAddress,
        ext_address: OTExtAddress
    ) -> Result<(), OTError<Self::Error>>;
}

/// The Radio Operation Handles are Implemented by the Underlying Driver and
/// are called by the radio driver on Thread
pub trait OTRadioOperationHandles {
    type Error;

    /// Notify OpenThread that the transmission has started
    fn tx_started(&mut self, frame: OTRadioFrame) -> Result<(), Self::Error>;

    /// Notify OpenThread a transmission operation has completed, providing
    /// both the transmitted frame and, if applicable, the received ack frame.
    fn tx_done(
        &mut self,
        frame: OTRadioFrame,
        ack_frame: OTRadioFrame,
        error: OTError<Self::Error>,
    ) -> Result<(), Self::Error>;

    /// Notify the OpenThread diagnostics module that the transmission was completed.
    fn diag_tx_done(
        &mut self,
        frame: OTRadioFrame,
        error: OTError<Self::Error>,
    ) -> Result<(), Self::Error>;

    /// Get the raw power setting for the given channel.
    /// 
    /// Platform radio layer should parse the raw power setting based on the radio layer defined format and set the
    /// parameters of each radio hardware module
    fn get_raw_power_setting(
        &mut self,
        channel: u8,
        raw_power_setting_buffer: &mut [u8]
    ) -> Result<(), OTError<Self::Error>>;
}

/// Traits implemented by a radio that is capable of energy scans
pub trait OTRadioOperationEnergyScan {
    type Error;

    /// Begin th energy scan sequence on the radio
    fn energy_scan(&mut self, channel: u8, duration: i16) -> Result<(), OTError<Self::Error>>;

    /// Get the maximum energy scan result from a triggered energy scan
    fn energy_scan_done(&mut self) -> Result<i8, Self::Error>;
}

/// Handles called by a device that implements the OTRadioOperationsEnergyScan as callbacks
pub trait OTRadioOperationEnergyScanHandles {
    type Error;

    /// Notify OpenThread that the energy scan is complete.
    fn energy_scan_done(&mut self, max_rssi: i8) -> Result<(), Self::Error>;
}

/// Trait for OpenThread Radios that implement the Coex feature
pub trait OTRadioOperationCoex {
    type Error;

    /// Enable the radio coex
    fn set_coex_enabled(&mut self, enabled: bool) -> Result<(), Self::Error>;

    /// Check whether radio coex is enabled or not
    fn is_coex_enabled(&mut self) -> Result<bool, Self::Error>;

    /// Get the radio coexistence metrics
    fn get_coex_metrics(&mut self) -> Result<OTRadioCoexMetrics, Self::Error>;
}

/// Trait for OpenThread Radios that implement CSL retriever functionality
pub trait OTRadioOperationsCSL {
    type Error;

    /// Enable or disable CSL receiver
    fn enable_csl(
        &mut self,
        csl_period: u32,
        short_address: OTShortAddress,
        ext_address: OTExtAddress,
    ) -> Result<(), Self::Error>;

    /// Reset CSL receiver in the platform
    fn reset_csl(&mut self) -> Result<(), Self::Error>;

    /// Update CSL sample time in radio driver
    /// 
    /// Sample time is stored in radio driver as a copy to calculate phase when
    /// sending ACK with CSL IE. The CSL sample (window) of the CSL receiver extends
    /// before and after the sample time. The CSL sample time marks a timestamp in
    /// the CSL sample window when a frame should be received in "ideal conditions"
    /// if there would be no inaccuracy/clock-drift.
    fn update_csl_sample_time(&mut self, sample_time: u32) -> Result<(), Self::Error>;

    /// Get the current estimated worst case accuracy (maximum ± deviation from the
    /// nominal frequency) of the local radio clock in units of PPM. This is the
    /// clock used to schedule CSL operations.
    ///
    /// @note Implementations MAY estimate this value based on current operating
    /// conditions (e.g. temperature).
    ///
    /// In case the implementation does not estimate the current value but returns a
    /// fixed value, this value MUST be the worst-case accuracy over all possible
    /// foreseen operating conditions (temperature, pressure, etc) of the
    /// implementation.
    fn get_csl_accuracy(&mut self) -> Result<u8, Self::Error>;

    /// The fixed uncertainty (i.e. random jitter) of the arrival time of CSL
    /// transmissions received by this device in units of 10 microseconds.
    ///
    /// This designates the worst case constant positive or negative deviation of
    /// the actual arrival time of a transmission from the transmission time
    /// calculated relative to the local radio clock independent of elapsed time. In
    /// addition to uncertainty accumulated over elapsed time, the CSL channel sample
    /// ("RX window") must be extended by twice this deviation such that an actual
    /// transmission is guaranteed to be detected by the local receiver in the
    /// presence of random arrival time jitter.
    fn get_csl_uncertainty(&mut self) -> Result<u8, Self::Error>;
}

/// Optional Platform Implementations
pub trait OTRadioOperationOptional {
    type Error;

    /// Get current state of the radio
    fn get_state(&mut self) -> Result<OTRadioState, Self::Error>;

    /// Add a calibrated power of the specified channel to the power calibration table.
    ///
    /// @note This API is an optional radio platform API. It's up to the platform layer to implement it.
    ///
    /// The @p aActualPower is the actual measured output power when the parameters of the radio hardware modules
    /// are set to the @p aRawPowerSetting.
    ///
    /// The raw power setting is an opaque byte array. OpenThread doesn't define the format of the raw power setting.
    /// Its format is radio hardware related and it should be defined by the developers in the platform radio driver.
    /// For example, if the radio hardware contains both the radio chip and the FEM chip, the raw power setting can be
    /// a combination of the radio power register and the FEM gain value.
    fn add_calibrated_power(
        &mut self,
        channel: u8,
        actual_power: i16,
        raw_power_setting: &[u8],
    ) -> Result<(), OTError<Self::Error>>;

    /// Clear all calibrated powers from the power calibration table
    fn clear_calibrated_powers(&mut self) -> Result<(), OTError<Self::Error>>;

    /// Set the target power for the given channel
    /// 
    /// NOTE: This API is optional and should only be used in stead of set_transmit_power
    fn set_channel_target_power(&mut self, channel: u8, target_power: i16) -> Result<(), OTError<Self::Error>>;
}