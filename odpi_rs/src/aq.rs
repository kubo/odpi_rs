// odpi_rs - a thin wrapper over Oracle Database Programming Interface for C
//
// URL: https://github.com/kubo/odpi_rs
//
//-----------------------------------------------------------------------------
// Copyright (c) 2025 Kubo Takehiro <kubo@jiubao.org>. All rights reserved.
// This program is free software: you can modify it and/or redistribute it
// under the terms of:
//
// (i)  the Universal Permissive License v 1.0 or at your option, any
//      later version (http://oss.oracle.com/licenses/upl); and/or
//
// (ii) the Apache License v 2.0. (http://www.apache.org/licenses/LICENSE-2.0)
//-----------------------------------------------------------------------------
//! Advanced Queueing

mod deq_options;
mod enq_options;
mod enums;
mod msg_props;
mod msg_recipient;
mod queue;

pub use deq_options::DeqOptions;
pub use enq_options::EnqOptions;
pub use enums::*;
pub use msg_props::MsgProps;
pub use msg_recipient::MsgRecipient;
pub use queue::Queue;
