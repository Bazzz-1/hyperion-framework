// -------------------------------------------------------------------------------------------------
// Hyperion Framework
// https://github.com/Bazzz-1/hyperion-framework
//
// A lightweight component-based TCP framework for building service-oriented Rust applications with
// CLI control, async messaging, and lifecycle management.
//
// Copyright 2025 Robert Hannah
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// -------------------------------------------------------------------------------------------------

// Package
use serde::{Deserialize, Serialize};

// Local
use crate::containerisation::container_state::ContainerState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerDirective {
    // IMPLEMENTED
    Shutdown,       // Shutdown local container
    SystemShutdown, // Shutdown command that is propagated through the container network
    // TO BE IMPLEMENTED
    RetryAllConnections,           // Retries any broken connections
    Heartbeat,                     // Container heartbeat
    FriendStateRequest,            // Request the state of a friend container
    StateResponse(ContainerState), // Response to a container state request
}
