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

use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(&self) -> Result<User> {
        let mut user = self._priv_build()?;
        println!("build - {:?}", user);
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }
    pub fn dob(&mut self, value: &str) -> &mut Self {
        self.dob = DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();
        self
    }
}

fn main() -> Result<()> {
    let user = User::build()
        .name("zhouzixin")
        .skill("rust")
        .skill("java")
        .email("zhouzixin@apache.org")
        .dob("2001-01-01T00:00:00Z")
        .build()?;
    println!("{:?}", user);
    Ok(())
}
