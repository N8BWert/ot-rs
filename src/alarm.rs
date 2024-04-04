pub trait OTAlarm {
    /// Signal the alarm has fired
    fn alarm_fired(&mut self) -> bool;

    // Signal the diagnostics module that the alarm has fired.
    fn alarm_fired_diagnostics(&mut self) -> bool;
    
    /// Set the alarm to fire dt milliseconds after t0
    /// 
    /// Params:
    ///     t0 - reference time
    ///     dt - alarm delay
    fn start_alarm_at(&mut self, t0: u32, dt: u32);

    /// Stop the alarm
    fn stop_alarm(&mut self);

    /// Get the current time (in ms)
    /// 
    /// Returns:
    ///     (u32): The current time
    fn get_now(&mut self) -> u32;
}