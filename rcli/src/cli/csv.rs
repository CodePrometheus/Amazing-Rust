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

use std::fmt;
use clap::Parser;
use csv::{Reader, StringRecord};
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug)]
pub struct CsvRecord {
    pub headers: Option<StringRecord>,
    pub records: Vec<StringRecord>,
}

impl CsvRecord {
    pub fn new(headers: Option<StringRecord>, records: Vec<StringRecord>) -> Self {
        Self { headers, records }
    }
}

#[derive(Debug, Clone, Parser)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
    Raw,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "raw")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

impl TryFrom<Reader<File>> for CsvRecord {
    type Error = anyhow::Error;

    fn try_from(mut rdr: Reader<File>) -> Result<Self, Self::Error> {
        let headers = rdr.headers()?.clone();
        let records = rdr.records().collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            headers: Some(headers),
            records,
        })
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "raw" => Ok(OutputFormat::Raw),
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid CSV output format")),
        }
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
            OutputFormat::Raw => "raw",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
            OutputFormat::Raw => "raw",
        };
        write!(f, "{}", s)
    }
}

impl From<CsvRecord> for String {
    fn from(csv_record: CsvRecord) -> Self {
        let mut ret = vec![];
        if let Some(headers) = csv_record.headers {
            ret.push(headers.clone());
        }
        ret.extend(csv_record.records.clone());
        ret.iter()
            .map(|record| record.iter().collect::<Vec<_>>().join(","))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<CsvRecord> for Vec<Value> {
    fn from(csv_record: CsvRecord) -> Self {
        let mut ret = vec![];
        if let Some(headers) = csv_record.headers {
            for record in csv_record.records {
                let record = headers.iter().zip(record.iter()).collect::<Value>();
                ret.push(record);
            }
        } else {
            for record in csv_record.records {
                let record = record.iter().collect::<Value>();
                ret.push(record);
            }
        }
        ret
    }
}

#[derive(Debug, Serialize)]
pub struct TomlStruct {
    items: Vec<Value>,
}

impl TomlStruct {
    pub fn new(items: Vec<Value>) -> Self {
        Self { items }
    }
}