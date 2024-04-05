//!
//! OpenThread APIs related to message buffer and queues
//! 

use alloc::vec::Vec;
use core::cell::RefCell;
use alloc::rc::Rc;
use alloc::boxed::Box;

/// OpenThread message priority levels
pub enum OTMessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// OpenThread message origins
pub enum OTMessageOrigin {
    ThreadNetif = 0,
    HostTrusted = 1,
    HostUntrusted = 2,
}

/// Message Settings
pub struct OTMessageSettings {
    link_security_enabled: bool,
    priority: u8,
}

/// Link-specific information for messages received from the Thread radio.
pub struct OTThreadLinkInfo {
    pan_id: u16,
    channel: u8,
    rssi: i8,
    lqi: u8,
    link_security: bool,
    is_dst_pan_id_broadcast: bool,
    time_sync_sequence: u8,
    network_time_offset: i64,
    // TODO: Change this to enum
    radio_type: u8,
}

/// Open Thread Message
pub trait OTMessage {
    type Error;

    fn get_message_length(&self) -> u16;

    fn set_message_length(&mut self, length: u16);

    fn get_message_offset(&self) -> u16;

    fn set_message_offset(&mut self, offset: u16);

    fn is_link_security_enabled(&self) -> bool;

    fn is_loopback_to_host_allowed(&self) -> bool;

    fn set_loopback_to_host_allowed(&mut self, allowed: bool);

    fn is_multicast_loop_enabled(&self) -> bool;

    fn set_multicast_loop_enabled(&mut self, enabled: bool);

    fn get_message_origin(&self) -> OTMessageOrigin;

    fn set_message_origin(&mut self, origin: OTMessageOrigin);

    fn set_direct_transmission(&mut self, enabled: bool);

    fn get_rssi(&self) -> i8;

    fn get_thread_link_info(&self) -> Result<OTThreadLinkInfo, Self::Error>;

    fn append_to_message(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Read bytes from message, returning the number of bytes read
    fn read_from_message(&self, offset: u16, buffer: &mut [u8]) -> u16;

    /// Write bytes to the message, returning the number of bytes read
    fn write_to_message(&mut self, offset: u16, buffer: &[u8]) -> usize;
}

/// Linked List Containing OTMessages
// pub struct OTMessageQueue<'a, E> {
//     data: Vec<&'a mut dyn OTMessage<Error=E>>,
// }

/// Information about a message queue
pub struct OTMessageQueueInfo {
    num_messages: u16,
    num_buffers: u16,
    total_bytes: u32,
}

/// Message buffer information for different queues used by OpenThread stack
pub struct OTBufferInfo {
    total_buffers: u16,
    free_buffers: u16,

    // maximum number of used buffers at the same time since OT stack initialization or last call to
    //`reset_buffer_info()`
    max_used_buffers: u16,

    lowpan_send_queue: OTMessageQueueInfo,
    lowpan_reassembly_queue: OTMessageQueueInfo,
    ip6_queue: OTMessageQueueInfo,
    mpl_queue: OTMessageQueueInfo,
    coap_queue: OTMessageQueueInfo,
    coap_secure_queue: OTMessageQueueInfo,
    application_coap_queue: OTMessageQueueInfo,
}

/// OTMessage Queue Trait
pub trait OTMessageQueue: IntoIterator {
    type Error;

    fn enqueue_message(&mut self, message: Box<dyn OTMessage<Error=Self::Error>>);

    fn enqueue_message_at_head(&mut self, message: Box<dyn OTMessage<Error=Self::Error>>);

    fn dequeue_message(&mut self, message: &Box<dyn OTMessage<Error=Self::Error>>);

    fn get_head(&mut self) -> &Box<dyn OTMessage<Error=Self::Error>>;
}

/// Trait Implemented by OpenThread Instance For Interfacing with the Message Queue
pub trait OTInstanceMessageQueue {
    fn get_message_buffer_info(&mut self) -> OTBufferInfo;

    fn reset_buffer_info(&mut self);
}

impl<E> OTMessageQueue for Vec<Box<dyn OTMessage<Error=E>>> {
    type Error = E;

    fn enqueue_message(&mut self, message: Box<dyn OTMessage<Error=Self::Error>>) {
        self.push(message);
    }

    fn enqueue_message_at_head(&mut self, message: Box<dyn OTMessage<Error=Self::Error>>) {
        self.insert(0, message);
    }

    fn dequeue_message(&mut self, message: &Box<dyn OTMessage<Error=Self::Error>>) {
        for i in 0..self.len() {
            if Box::into_raw(self[i]) == Box::into_raw(*message) {
                self.remove(i);
                return;
            }
        }
    }

    fn get_head(&mut self) -> &Box<dyn OTMessage<Error=Self::Error>> {
        &self[0]
    }
}