/*
 * Copyright (c) 2020 Yaguo Zhou
 * fy is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *          http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

use std::error::Error;
use std::time::Duration;

use chrono::Local;
use clap::{App, Arg};
use log::LevelFilter;
use objs::*;
use reqwest::Client;
use serde_json::json;
use std::io::Write;

mod objs;

const CONNECT_TIMEOUT_SECS: u64 = 2;
const READ_TIMEOUT_SECS: u64 = 2;
const MAX_TRY: u8 = 3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("to_translate")
                .required(true)
                .help("word to translate"),
        )
        .get_matches();

    let to_translate = matches.value_of("to_translate").unwrap();

    let url = "http://dict.youdao.com/jsonapi?";
    let dicts = [[
        "blng_sents_part",
        "rel_word",
        "simple",
        "phrs",
        "meta",
        "ec",
    ]];

    let dicts = json!(
        {
            "count":99,
            "dicts":dicts
        }
    )
    .to_string();

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .timeout(Duration::from_secs(READ_TIMEOUT_SECS))
        .user_agent("curl")
        .build()?;

    let mut times_left = MAX_TRY;
    loop{
        let response = client
            .get(url)
            .query(&[("q", to_translate)])
            .query(&[("dicts", &dicts)])
            .send()
            .await;
        match response {
            Ok(resp) => {
                let json_result: FyResult = resp.json().await?;
                println!("{}", &json_result.text());
                break;
            }
            Err(e) => {
                debug!("{:?}", e);
                if e.is_timeout() {
                    error!("Network timeout, retrying...");
                    if times_left > 0{
                        times_left -= 1;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    error!("{}", e);
                }
            }
        }
    }

    Ok(())
}

fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
