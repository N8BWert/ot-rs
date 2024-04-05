//!
//! Optional Platform-Specific Functions needed by OpenThread's example applications
//! 

/// Platform-Specific Functions used by OpenThread's example applications
pub trait OTSystem {
    /// Performs all platform-specific initialization of OpenThread's drivers.
    /// 
    /// Note: This function is not called by the OpenThread library. Instead, the system/RTOS should call this function
    /// when initialization of OpenThread's drivers is most appropriate.
    fn ot_sys_init(&mut self, args: &[&str]);

    /// Performs all platform-specific deinitialization for OpenThread's drivers.
    /// 
    /// Note: This function is not called by the OpenThread library. Instead, the system/RTOs should call this function
    /// when deinitialization of OpenThread's drivers is mot appropriate.
    fn ot_sys_deinit(&mut self);

    /// Returns true if a pseudo-reset was requested.
    /// 
    /// In such case, the main loop should shut down and re-initialize the OpenThread instance.
    /// 
    /// Note: This function is not called by the OpenThread library. Instead, the system/RTOs should call this function
    /// when deinitialization of OpenThread's drivers is mot appropriate.
    fn reset_was_requested(&mut self) -> bool;

    /// Peforms all platform-specific processing for OpenThread's example applications.
    /// 
    /// Note: This function is not called by the OpenThread library. Instead, the system/RTOs should call this function
    /// when deinitialization of OpenThread's drivers is mot appropriate.
    fn process_drivers(&mut self);

    /// Is called whenever platform drivers need processing.
    /// 
    /// Note: This function is not called by the OpenThread library. Instead, the system/RTOs should call this function
    /// when deinitialization of OpenThread's drivers is mot appropriate.
    fn event_signal_pending(&mut self);
}