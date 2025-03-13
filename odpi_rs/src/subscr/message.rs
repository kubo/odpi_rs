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
use crate::subscr::{EventType, OpCode};
use crate::utils::*;
use crate::Result;
use odpi_rs_procmacro::odpic_doc;
use odpic_sys::*;
use std::slice;

#[odpic_doc(name = "dpiSubscrMessage")]
pub struct Message<'a> {
    pub event_type: EventType,
    pub db_name: &'a str,
    pub tables: Vec<MessageTable<'a>>,
    pub queries: Vec<MessageQuery<'a>>,
    pub tx_id: &'a [u8],
    pub registered: bool,
    pub queue_name: &'a str,
    pub consumer_name: &'a str,
    pub aq_msg_id: &'a [u8],
}

impl Message<'_> {
    pub(crate) fn new(message: &dpiSubscrMessage) -> Result<Message> {
        let tables =
            unsafe { slice::from_raw_parts(message.tables, message.numTables.try_into()?) };
        let queries =
            unsafe { slice::from_raw_parts(message.queries, message.numQueries.try_into()?) };
        Ok(Message {
            event_type: message.eventType.try_to_rust()?,
            db_name: (message.dbName, message.dbNameLength).try_to_rust()?,
            tables: tables
                .iter()
                .map(MessageTable::new)
                .collect::<Result<Vec<_>>>()?,
            queries: queries
                .iter()
                .map(MessageQuery::new)
                .collect::<Result<Vec<_>>>()?,
            tx_id: unsafe {
                slice::from_raw_parts(message.txId as *const u8, message.txIdLength.try_into()?)
            },
            registered: message.registered.to_rust(),
            queue_name: (message.queueName, message.queueNameLength).try_to_rust()?,
            consumer_name: (message.consumerName, message.consumerNameLength).try_to_rust()?,
            aq_msg_id: unsafe {
                slice::from_raw_parts(
                    message.aqMsgId as *const u8,
                    message.aqMsgIdLength.try_into()?,
                )
            },
        })
    }
}

#[odpic_doc(name = "dpiSubscrMessageQuery")]
pub struct MessageQuery<'a> {
    pub id: u64,
    pub operation: OpCode,
    pub tables: Vec<MessageTable<'a>>,
}

impl MessageQuery<'_> {
    fn new(query: &dpiSubscrMessageQuery) -> Result<MessageQuery> {
        let tables = unsafe { slice::from_raw_parts(query.tables, query.numTables.try_into()?) };
        Ok(MessageQuery {
            id: query.id,
            operation: query.operation.to_rust(),
            tables: tables
                .iter()
                .map(MessageTable::new)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[odpic_doc(name = "dpiSubscrMessageTable")]
pub struct MessageTable<'a> {
    pub operation: OpCode,
    pub name: &'a str,
    pub rows: Vec<MessageRow<'a>>,
}

impl MessageTable<'_> {
    fn new(table: &dpiSubscrMessageTable) -> Result<MessageTable> {
        let rows = unsafe { slice::from_raw_parts(table.rows, table.numRows.try_into()?) };
        Ok(MessageTable {
            operation: table.operation.to_rust(),
            name: (table.name, table.nameLength).try_to_rust()?,
            rows: rows
                .iter()
                .map(MessageRow::new)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[odpic_doc(name = "dpiSubscrMessageRow")]
pub struct MessageRow<'a> {
    pub operation: OpCode,
    pub rowid: &'a str,
}

impl MessageRow<'_> {
    fn new(row: &dpiSubscrMessageRow) -> Result<MessageRow> {
        Ok(MessageRow {
            operation: row.operation.to_rust(),
            rowid: (row.rowid, row.rowidLength).try_to_rust()?,
        })
    }
}
