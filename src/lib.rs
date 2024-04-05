//!
//! Complete OpenThread Rust Port
//!

#![no_std]

extern crate alloc;

pub mod platform;

pub mod core;
pub use core::instance::instance::OTInstance;

// TODO: Move these around
pub mod backbone_router_ftd;
pub mod backbone_router;
pub mod ble_secure;
pub mod border_agent;
pub mod border_router;
pub mod border_routing;
pub mod channel_manager;
pub mod channel_monitor;
pub mod child_supervision;
pub mod cli;
pub mod coap_secure;
pub mod coap;
pub mod commissioner;
pub mod config;
pub mod crypto;
pub mod dataset_ftd;
pub mod dataset_updater;
pub mod dataset;
pub mod diag;
pub mod dns_client;
pub mod dns;
pub mod dnssd_server;
pub mod error;
pub mod heap;
pub mod history_tracker;
pub mod icmp6;
pub mod instance;
pub mod ip6;
pub mod jam_detection;
pub mod joiner;
pub mod link_metrics;
pub mod link_raw;
pub mod link;
pub mod logging;
pub mod mesh_diag;
pub mod message;
pub mod multi_radio;
pub mod nat64;
pub mod netdata_publisher;
pub mod netdata;
pub mod netdiag;
pub mod network_time;
pub mod ping_sender;
pub mod radio_status;
pub mod random_crypto;
pub mod random_noncrypto;
pub mod server;
pub mod sntp;
pub mod srp_client_buffers;
pub mod srp_client;
pub mod tasklet;
pub mod tcat;
pub mod tcp_ext;
pub mod tcp;
pub mod thread_ftd;
pub mod thread;
pub mod trel;
pub mod udp;