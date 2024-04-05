//!
//! OpenThread IPv6 API
//! 

use crate::OTInstance;

// Size of an IPv6 prefix (bytes)
pub const OT_IP6_PREFIX_SIZE: usize = 8;
// Size of an IPv6 prefix (bits)
pub const OT_IP6_PREFIX_BITSIZE: usize = OT_IP6_PREFIX_SIZE * 8;
// Size of an IPv6 Interface Identifier (bytes)
pub const OT_IP6_IID_SIZE: usize = 8;
// Size of an IPv6 Address (bytes)
pub const OT_IP6_ADDRESS_SIZE: usize = 16;
// Size of an IPv6 address (bits)
pub const OT_IP6_ADDRESS_BITSIZE: usize = OT_IP6_ADDRESS_SIZE * 8;
// Size of an IPv6 header (bytes)
pub const OT_IP6_HEADER_SIZE: usize = 40;
// Offset of the proto field in the IPv6 Header (bytes)
pub const OT_IP6_HEADER_PROTO_OFFSET: usize = 6;

/// Interface Identifier of an IPv6 address.
pub enum OTIp6InterfaceIdentifier {
    M8([u8; OT_IP6_IID_SIZE]),
    M16([u16; OT_IP6_IID_SIZE / 2]),
    M32([u8; OT_IP6_IID_SIZE / 4]),
}

/// Network Prefix of an IPv5 address (most significant 64 bits of the address)
type OTIp6NetworkPrefix = [u8; OT_IP6_PREFIX_SIZE];

/// IPv6 Address Components
pub struct OTIp6AddressComponents {
    network_prefix: OTIp6NetworkPrefix,
    iid: OTIp6InterfaceIdentifier,
}

/// Ipv6 Address
pub enum OTIp6Address {
    M8([u8; OT_IP6_ADDRESS_SIZE]),
    M16([u16; OT_IP6_ADDRESS_SIZE / 2]),
    M32([u32; OT_IP6_ADDRESS_SIZE / 4]),
    Components(OTIp6AddressComponents),
}

/// IPv6 Prefix
pub struct OTIp6Prefix {
    prefix: OTIp6Address,
    length: u8,
}

/// Ipv6 Address Origins
pub enum OTAddressOrigin {
    Thread = 0,
    Slaac = 1,
    DHCPv6 = 2,
    Manual = 3,
}

/// IPv6 network interface unicast address.
pub struct OTNetworkInterfaceAddress<'a> {
    address: OTIp6Address,
    prefix_length: u8,
    address_origin: u8,
    preferred: bool,
    valie: bool,
    scope_override_valid: bool,
    scope_override: u8,
    rloc: bool,
    mesh_local: bool,
    src_registered: bool,
    next: Option<&'a OTNetworkInterfaceAddress<'a>>,
}

/// IPv6 network interface multicast address
pub struct OTNetworkInterfaceMulticastAddress<'a> {
    address: OTIp6Address,
    next: Option<&'a OTNetworkInterfaceMulticastAddress<'a>>,
}

/// Ipv6 socket address
pub struct OTSocketAddress {
    address: OTIp6Address,
    port: u16,
}

/// ECN statusess, represented as in the IP header
pub enum OTECNStatus {
    NotCapable = 0x0,
    Capable0 = 0x2,
    Capable1 = 0x1,
    Marked = 0x3,
}

/// Local and Peer Ipv6 socket addresses
pub struct OTMessageInfo {
    socket_address: OTIp6Address,
    peer_address: OTIp6Address,
    socket_port: u16,
    peer_port: u16,
    hop_limi: u8,
    ecn: u8,
    is_host_interface: bool,
    allow_zero_hop_limit: bool,
    multicast_loop: bool,
}

/// Internet Protocol Numbers
pub enum OTIp6Protocol {
    // Ipv6 Hop-by-Hop Option
    HopOpts = 0,
    TCP = 6,
    UDP = 17,
    IP6 = 41,
    Routing = 43,
    Fragment = 44,
    ICMP6 = 58,
    NONE = 59,
    DestinationOptions = 60,
}

/// Ipv6 Module Interface
pub trait OTIPv6 {
    type Error;

    /// Brings the IPv6 interface up or down.
    /// 
    /// Call this to enable or disable IPv6 communication
    fn set_ipv6_enabled(
        &mut self,
        enabled: bool,
    ) -> Result<(), Self::Error>;

    /// Indicates whether or not the IPv6 interface is up.
    fn is_ipv6_enabled(&mut self) -> Result<bool, Self::Error>;

    /// Adds a Network Interface Address to the Thread interface.
    ///
    /// The passed-in instance @p aAddress is copied by the Thread interface. The Thread interface only
    /// supports a fixed number of externally added unicast addresses. See `OPENTHREAD_CONFIG_IP6_MAX_EXT_UCAST_ADDRS`.
    fn add_unicast_address(&mut self, address: &OTNetworkInterfaceAddress) -> Result<(), Self::Error>;

    fn remove_unicast_address(&mut self, address: OTIp6Address) -> Result<(), Self::Error>;

    fn get_unicast_addresses(&mut self) -> Result<&[OTIp6Address], Self::Error>;

    fn has_unicast_address(&mut self, address: OTIp6Address) -> Result<bool, Self::Error>;

    fn subscribe_multicast_address(&mut self, address: OTIp6Address) -> Result<(),Self::Error>;

    fn unsubscribe_multicast_address(&mut self, address: OTIp6Address) -> Result<(), Self::Error>;
}