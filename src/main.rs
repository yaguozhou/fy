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

use std::error::Error;
use std::time::Duration;

use clap::{App, Arg};
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use serde_json::json;

use objs::*;

mod objs;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("to_translate")
            .required(true)
            .help("word to translate")
        )
        .get_matches();

    let to_translate = matches.value_of("to_translate").unwrap();

    let url = "http://dict.youdao.com/jsonapi?";
    let dicts = [["blng_sents_part", "rel_word", "simple", "phrs", "meta", "ec"]];

    let dicts = json!(
        {
            "count":99,
            "dicts":dicts
        }
    ).to_string();
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(5))
        .user_agent("curl")
        .build()?;

    let resp: Response = client.get(url)
        .query(&[("q", to_translate)])
        .query(&[("dicts", dicts)])
        .send()?;
    let json_result: FyResult = resp.json()?;
    println!("{}", &json_result.text());

    Ok(())
}
