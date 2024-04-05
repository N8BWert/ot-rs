//!
//! This file includes platform abstractions for miscellaneous behaviors.
//! 

/// The possible reset codes for the device
pub enum OTResetReason {
    PowerOn = 0,
    External = 1,
    Software = 2,
    Fault = 3,
    Crash = 4,
    Assert = 5,
    Other = 6,
    Unknown = 7,
    Watchdog = 8,
}

/// Enumeration of micro-controller's power states.
///
/// These values are used for NCP configuration when `OPENTHREAD_CONFIG_NCP_ENABLE_MCU_POWER_STATE_CONTROL` is enabled.
///
/// The power state specifies the desired power state of NCP's micro-controller (MCU) when the underlying platform's
/// operating system enters idle mode (i.e., all active tasks/events are processed and the MCU can potentially enter a
/// energy-saving power state).
///
/// The power state primarily determines how the host should interact with the NCP and whether the host needs an
/// external trigger (a "poke") to NCP before it can communicate with the NCP or not.
///
/// After a reset, the MCU power state MUST be `OT_PLAT_POWER_STATE_ON`.
pub enum OTMcuPowerState {
    // NCP's MCU stays on and active all the time.
    //
    // When the NCP's desired power state is set to `ON`, host can send messages to NCP without requiring any "poke" or
    // external triggers.
    //
    // Note: The `ON` power state only determines the MCU's power mode and is not related to radio's state.
    On = 0,

    // NCP's MCU can enter low-power (energy-saving) state.
    //
    // When the NCP's desired power state is set to `LOW_POWER`, host is expected to "poke" the NCP (e.g., an external
    // trigger like an interrupt) before it can communicate with the NCP (send a message to the NCP). The "poke"
    // mechanism is determined by the platform code (based on NCP's interface to the host).
    //
    // While power state is set to `LOW_POWER`, NCP can still (at any time) send messages to host. Note that receiving
    // a message from the NCP does NOT indicate that the NCP's power state has changed, i.e., host is expected to
    // continue to "poke" when it wants to talk to the NCP until the power state is explicitly changed (by a successful
    // call to `otPlatSetMcuPowerState()` changing the state to `ON`).
    //
    // @note The `LOW_POWER` power state only determines the MCU's power mode and is not related to radio's state
    // (radio is managed by OpenThread core and device role, e.g., device being sleepy or not.
    LowPower = 1,

    // NCP is fully off.
    //
    // An NCP hardware reset (via a RESET pin) is required to bring the NCP back to `SPINEL_MCU_POWER_STATE_ON`.
    // RAM is not retained after reset.
    Off = 2,
}

pub trait OTMiscellaneous {
    type Error;

    /// Perform a software reset on the platform, if supported.
    fn reset(&mut self) -> Result<(), Self::Error>;

    /// Returns the reason for the last platform reset
    fn get_reset_reason(&mut self) -> Result<OTResetReason, Self::Error>;

    /// Provides a platform specific implementation for assert
    fn assert_fail(&mut self, filename: &'static str, line: isize);

    /// Performs a platform specific operation to wake the host MCU.
    /// This is used only for NCP configurations
    fn wake_host() -> Result<(), Self::Error>;
}

pub trait OTMiscellaneousBootloaderEnabled {
    type Error;

    /// Perform a hardware reset on the platform to launch bootloader mode, if supported
    fn reset_to_bootloader(&mut self) -> Result<(), Self::Error>;
}

pub trait OTMiscellaneousPowerStateControl {
    type Error;

    /// Sets the desired MCU power state
    fn set_mcu_power_state(&mut self, state: OTMcuPowerState) -> Result<(), Self::Error>;

    /// Gets the current desired MCU power state
    fn get_mcu_power_state(&mut self) -> Result<OTMcuPowerState, Self::Error>;
}