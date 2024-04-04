/// OpenThread Error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTError<E> {
    // Operation failed
    Failed,

    // Message dropped
    Dropped,

    // Insufficient buffers.
    NoBuffers,

    // No route available
    NoRoute,

    // Service is busy and could not service the operation
    Busy,

    // Failed to parse message
    Parse,

    // Input arguments were invalid
    InvalidArgs,

    // Security checks failed
    Security,

    // Address resolution requires an address query operation
    AddressQuery,

    // Address is not in the source match table
    NoAddress ,
    
    // Operation was aborted
    Abort ,

    // Function or method is not implemented
    NotImplemented ,

    // Cannot complete due to invalid state
    InvalidState ,

    // No acknowledgment was received after macMaxFrameRetries (IEEE 802.15.4-2006)
    NoAck,

    // A transmission could not take place due to activity on the channel, i.e., the CSMA-CA mechnism has failed
    // (IEEE 802.15.4-2006).
    ChannelAccessFailure,

    // Not currently attached to a Thread Partition.
    Detached,

    // FCS check failure while receiving
    Fcs,

    // No frame received
    NoFrameReceived,
    
    // Received a frame from an unknown neighbor
    UnknownNeighbor,

    // Received a frame from an invalid source address
    InvalidSourceAddress,

    // Received a frame filtered by the address filter (allowlisted or denylisted)
    AddressFiltered,

    // Received a frame filtered by the destination address check
    DestinationAddressFiltered,

    // The requested item could not be found.
    NotFound,

    // The operation is already in progress
    Already,

    // The creation of IPv6 address failed.
    IP6AddressCreationFailure,

    // Operation prevented by mode flags
    NoCapable,

    // Coap response or acknowledgment or DNS, SNTP response not received
    ResponseTimeout,

    // Received a duplicated frame.
    Duplicated,

    // Message is being dropped from reassembly list due to timeout
    ReassemblyTimeout,

    // Message is not a TMF Message
    NotTMF,

    // Received a non-lowpan data frame
    NotLowpanDataFrame,

    // The link margin was too low
    LinkMarginLow,

    // Input (CLI) command is invalid
    InvalidCommand,

    // Special error code used to indicate success/error status is pending and not key known
    Pending,

    // Request rejected
    Rejected,

    // Generic error (should not use)
    Generic,

    Platform(E),
}