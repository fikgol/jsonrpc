// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::net::SocketAddr;
use jsonrpc::Metadata;

pub struct RequestContext {
    pub peer_addr: SocketAddr,
}

pub trait MetaExtractor<M: Metadata> : Send + Sync {
    fn extract(&self, context: &RequestContext) -> Option<M>;
}

pub struct NoopExtractor;

impl<M: Metadata> MetaExtractor<M> for NoopExtractor {
    fn extract(&self, _context: &RequestContext) -> Option<M> { None }
}

#[derive(Clone)]
pub struct SocketMetadata {
    addr: SocketAddr,
}

impl Default for SocketMetadata {
    fn default() -> Self {
        SocketMetadata { addr: "0.0.0.0:0".parse().unwrap() }
    }
}

impl SocketMetadata {
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}

impl Metadata for SocketMetadata { }

impl From<SocketAddr> for SocketMetadata {
    fn from(addr: SocketAddr) -> SocketMetadata {
        SocketMetadata { addr: addr }
    }
}

pub struct PeerMetaExtractor;

impl MetaExtractor<SocketMetadata> for PeerMetaExtractor {
    fn extract(&self, context: &RequestContext) -> Option<SocketMetadata> {
        Some(context.peer_addr.into())
    }
}
