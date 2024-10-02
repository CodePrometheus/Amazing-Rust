// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::cli::{CsvRecord, OutputFormat, TomlStruct};
use csv::{Reader, ReaderBuilder};
use std::fs;
use serde_json::Value;

#[allow(dead_code)]
pub fn process_csv_old(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers.iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        _ => serde_json::to_string_pretty(&ret)?,
    };
    fs::write(output, content).expect("Failed to write to file");
    Ok(())
}

pub fn process_csv(
    input: &str,
    output: &str,
    format: OutputFormat,
    delimiter: char,
    header: bool,
) -> anyhow::Result<()> {
    let csv_record = read_csv(input, delimiter, header)?;
    let contents = csv_convert(csv_record, format)?;
    output_contents(output, &contents);
    Ok(())
}

pub fn output_contents(output: &str, contents: &str) {
    if output != "-" {
        fs::write(output, contents).unwrap();
    } else {
        println!("{}", contents);
    }
}

fn csv_convert(csv_record: CsvRecord, format: OutputFormat) -> anyhow::Result<String> {
    match format {
        OutputFormat::Raw => Ok(csv_record.into()),
        OutputFormat::Json => {
            let contents: Vec<Value> = csv_record.into();
            Ok(serde_json::to_string_pretty(&contents)?)
        }
        OutputFormat::Yaml => {
            let contents: Vec<Value> = csv_record.into();
            Ok(serde_yaml::to_string(&contents)?)
        }
        OutputFormat::Toml => {
            let contents = TomlStruct::new(csv_record.into());
            Ok(toml::to_string(&contents)?)
        }
    }
}

fn read_csv(input: &str, delimiter: char, header: bool) -> anyhow::Result<CsvRecord> {
    let rdr = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_path(input)?;

    let csv_record: CsvRecord = rdr.try_into()?;
    if header {
        Ok(csv_record)
    } else {
        Ok(CsvRecord::new(None, csv_record.records))
    }
}