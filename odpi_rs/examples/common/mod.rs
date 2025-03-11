// Rust port of SampleLib.c in ODPI-C samples
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
// SampleLib.c
//   Common code used in all samples.
//-----------------------------------------------------------------------------

#![allow(dead_code)]
use odpi_rs::conn::{AccessToken, CommonCreateParams, Conn, Pool};
use odpi_rs::maybe_async;
use odpi_rs::soda;
use odpi_rs::Result;
use std::env;
use std::sync::OnceLock;

static SAMPLE_PARAMS: OnceLock<SampleParams> = OnceLock::new();

pub struct SampleParams {
    main_user_name: String,
    main_password: String,
    proxy_user_name: String,
    proxy_password: String,
    connect_string: String,
    dir_name: String,
}

impl SampleParams {
    /// acquire parameters
    pub fn get() -> &'static SampleParams {
        fn env_var_or(env_name: &str, default: &str) -> String {
            match env::var_os(env_name) {
                Some(env_var) => env_var.into_string().unwrap(),
                None => String::from(default),
            }
        }

        SAMPLE_PARAMS.get_or_init(|| SampleParams {
            main_user_name: env_var_or("ODPIC_SAMPLES_MAIN_USER", "odpicdemo"),
            main_password: env_var_or("ODPIC_SAMPLES_MAIN_PASSWORD", "welcome"),
            proxy_user_name: env_var_or("ODPIC_SAMPLES_PROXY_USER", "odpicdemo_proxy"),
            proxy_password: env_var_or("ODPIC_SAMPLES_PROXY_PASSWORD", "welcome"),
            connect_string: env_var_or("ODPIC_SAMPLES_CONNECT_STRING", "localhost/orclpdb"),
            dir_name: env_var_or("ODPIC_SAMPLES_DIR_NAME", "odpicdemo_dir"),
        })
    }
}

/// connect to the database
#[maybe_async]
pub async fn get_conn(with_pool: bool, common_params: Option<&CommonCreateParams>) -> Result<Conn> {
    let params = SampleParams::get();

    if with_pool {
        let pool = Pool::create(
            &params.main_user_name,
            &params.main_password,
            &params.connect_string,
            common_params,
            None,
        )
        .await?;
        pool.acquire_connection("", "", None).await
    } else {
        Conn::create(
            &params.main_user_name,
            &params.main_password,
            &params.connect_string,
            common_params,
            None,
        )
        .await
    }
}

/// acquire SODA database
pub fn get_soda_db() -> Result<soda::Db> {
    unimplemented!()
}

/// get token and private key
pub fn get_access_token() -> Result<AccessToken> {
    unimplemented!()
}

/// populate access token
pub fn get_access_token_from_env(_env_name: &str) -> Result<AccessToken> {
    unimplemented!()
}
