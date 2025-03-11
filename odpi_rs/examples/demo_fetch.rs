// Rust port of DemoFetch.c in ODPI-C samples
//
// The following is the original copyright.
//-----------------------------------------------------------------------------
// Copyright (c) 2016, 2022, Oracle and/or its affiliates.
//
// This software is dual-licensed to you under the Universal Permissive License
// (UPL) 1.0 as shown at https://oss.oracle.com/licenses/upl and Apache License
// 2.0 as shown at http://www.apache.org/licenses/LICENSE-2.0. You may choose
// either license.
//
// If you elect to accept the software under the Apache License, Version 2.0,
// the following applies:
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// DemoFetch.c
//   Demos simple fetch of numbers and strings.
//-----------------------------------------------------------------------------

use odpi_rs::stmt::ExecMode;
use odpi_rs::types::{NativeType, OracleType, Rowid};
use odpi_rs::Result;
mod common;

const SQL_TEXT_1: &str = "select IntCol, StringCol, RawCol, rowid \
                          from DemoStrings \
                          where IntCol > :intCol";
const SQL_TEXT_2: &str = "select IntCol \
                          from DemoStrings \
                          where rowid = :1";
const BIND_NAME: &str = "intCol";

#[odpi_rs::main(maybe_async)]
async fn main() -> Result<()> {
    // connect to database
    let conn = common::get_conn(true, None).await?;

    // create variable for storing the rowid of one of the rows
    let rowid_var = conn.new_var(
        OracleType::Rowid,
        NativeType::Rowid,
        1,
        0,
        false,
        false,
        None,
    )?;

    // prepare and execute statement
    let stmt = conn.prepare_stmt(false, SQL_TEXT_1, "")?;
    let bind_value = 7;
    stmt.bind_value_by_name(BIND_NAME, &bind_value)?;
    let num_query_columns = stmt.execute(ExecMode::DEFAULT).await?;
    stmt.define_value(1, OracleType::Number, NativeType::Bytes, 0, false, None)?;

    // fetch rows
    println!("Fetch rows with IntCol > {}", bind_value);
    let mut rowid_as_string = String::new();
    while let Some(_) = stmt.fetch().await? {
        let int_col_value: &str = unsafe { stmt.query_value_unsafe(1)? };
        let string_col_value: &str = unsafe { stmt.query_value_unsafe(2)? };
        let raw_col_value: &[u8] = unsafe { stmt.query_value_unsafe(3)? };
        let rowid_value: Rowid = stmt.query_value(4)?;
        rowid_as_string = rowid_value.string_value()?.into();
        println!(
            "Row: Int = {}, String = '{}', Raw = '{:?}', Rowid = '{}'",
            int_col_value, string_col_value, raw_col_value, rowid_as_string
        );
        rowid_var.set_from_rowid(0, &rowid_value)?;
    }
    println!();

    // display description of each variable
    println!("Display column metadata");
    for i in 0..num_query_columns {
        let query_info = stmt.query_info(i + 1)?;
        println!(
            "('{}', {:?}, {}, {}, {}, {}, {})",
            query_info.name,
            query_info.type_info.oracle_type.unwrap(),
            query_info.type_info.size_in_chars,
            query_info.type_info.client_size_in_bytes,
            query_info.type_info.precision,
            query_info.type_info.scale,
            query_info.null_ok
        );
    }
    println!("");

    println!("Fetch rows with rowid = {}", rowid_as_string);

    // prepare and execute statement to fetch by rowid
    let stmt = conn.prepare_stmt(false, SQL_TEXT_2, "")?;
    stmt.bind_by_pos(1, &rowid_var)?;
    stmt.execute(ExecMode::DEFAULT).await?;

    // fetch rows
    while let Some(_) = stmt.fetch().await? {
        let int_col_value: i64 = stmt.query_value(1)?;
        println!("Row: Int = {}", int_col_value);
    }
    println!("Done.");
    Ok(())
}
