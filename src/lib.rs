//! [![github]](https://github.com/rusty-bolt/conntrack-rs)
//!
//! [github]: https://img.shields.io/badge/github-rusty--bolt/conntrack--rs-blueviolet?style=for-the-badge&labelColor=555555&logo=github
//!
//! ---
//!
//! This library provides access to the [`conntrack`](https://conntrack-tools.netfilter.org/conntrack.html)
//! subsystem in the linux kernel leveraging netlink support via the [`neli`](https://docs.rs/neli/latest/neli/index.html)
//! library.
//!
//! The current version only supplies `Dump()` functionality for the `Conntrack` table. Leveraging the
//! [`conntrack-tools`](https://conntrack-tools.netfilter.org/) utility in linux, the `Dump()` behavior
//! is equivalent to: `conntrack -L`. Most of the model and attribute parsing supported in this library
//! extends beyond the `dump()` command, which allows this library to eventually cover the full feature set
//! of the conntrack subsystem.
//!
//! You can enable byte and packet counters using `sysctl -w net.netfilter.nf_conntrack_acct=1`
//!
//! # Privileges
//!
//! You need the `CAP_NET_ADMIN` capability in order to allow your application to receive events from and to send commands to kernel-space,
//! excepting the conntrack table dumping operation.
//!
//! ### WSL2 Conntrack
//!
//! Note that in order to enable connection tracking via `conntrack` on WSL2, you'll need to add the following iptable entry:
//! ```bash
//! iptables -A INPUT -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT
//! ```
//!
//! # Example
//!
//! ```rust
//! use conntrack::*;
//!
//! fn main() -> Result<()> {
//!     // Create the Conntrack table via netfilter socket syscall
//!     let mut ct = Conntrack::connect()?;
//!
//!     // Dump conntrack table as a Vec<Flow>
//!     let flows = ct.dump()?;
//!
//!     for flow in flows {
//!         log::info!("{flow:?}");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! <br>

pub use crate::connection::*;
pub use crate::error::*;
pub use crate::result::*;

pub mod attributes;
pub mod decoders;
pub mod message;
pub mod model;

mod connection;
mod error;
mod result;
