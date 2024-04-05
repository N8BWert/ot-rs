//!
//! Platform Abstraction for border routing manager.
//! 

/// Platform Abstraction for border routing manager.
pub trait OTBorderRouting {
    type Error;

    /// Handles ICMP6 RA messages received on the Thread interface on the platform.
    ///
    /// The `aMessage` should point to a buffer of a valid ICMPv6 message (without IP headers) with router advertisement as
    /// the value of type field of the message.
    ///
    /// When DHCPv6 PD is disabled, the message will be dropped silently.
    ///
    /// Note: RA messages will not be forwarded into Thread networks, while for many platforms, RA messages is the way of
    /// distributing a prefix and other infomations to the downstream network. The typical usecase of this function is to
    /// handle the router advertisement messages sent by the platform as a result of DHCPv6 Prefix Delegation.
    fn border_routing_process_icmp6_ra(&mut self, message: &[u8]) -> Result<(), Self::Error>;
}