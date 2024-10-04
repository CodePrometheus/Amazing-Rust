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

use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

// pretend this is a heavy operation, CPU intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        // a.len => a.data.len() (Deref trait)
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
