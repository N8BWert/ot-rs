/// Platform Abstractions Necessary for OpenThread Alarms

/// Platform Abstraction for OpenThread Millisecond Alarm
pub trait OTAlarmMS {    
    /// Set the alarm to fire dt milliseconds after t0
    /// 
    /// Params:
    ///     t0 - reference time
    ///     dt - alarm delay
    fn start_alarm_at_ms(&mut self, t0: u32, dt: u32);

    /// Stop the alarm
    fn stop_alarm_ms(&mut self);

    /// Get the current time (in ms)
    /// 
    /// Returns:
    ///     (u32): The current time
    fn get_now_ms(&mut self) -> u32;
}

/// Platform Abstraction for Alarm Handlers
pub trait OTAlarmMSHandler {
    type Error;

    /// Handle a fired alarm
    fn alarm_fired_ms(&mut self) -> Result<(), Self::Error>;
}

/// Platform Abstraction for Alarm Diagnostics Handlers
pub trait OTAlarmMSDiagnosticsHandler {
    type Error;

    /// Diagnostics Module Handler for a fired alarm
    fn handle_fired_diagnostics_ms(&mut self) -> Result<(), Self::Error>;
}

/// Platform Abstraction for OpenThread Microsecond Alarm
pub trait OTAlarmUS {
    /// Set the alarm to fire ad dt microseconds after t0
    /// 
    /// Params:
    ///     t0 - reference time
    ///     dt - alarm delay
    fn start_alarm_at_us(&mut self, t0: u32, dt: u32);

    /// Stop the alarm
    fn stop_alarm_us(&mut self);

    /// Get the current time (in us)
    fn get_now_us(&mut self) -> u32;
}

/// Platform Abstraction for us Alarm Handlers
pub trait OTAlarmUSHandler {
    type Error;

    /// Handle a fired alarm
    fn alarm_fired_us(&mut self) -> Result<(), Self::Error>;
}