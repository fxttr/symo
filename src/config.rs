/*
 * Copyright (c) 2022, Florian Büstgens
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     1. Redistributions of source code must retain the above copyright
 *        notice, this list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the
 *        documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY Florian Büstgens ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Büstgens BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml::de::Error;

#[derive(Deserialize, Default)]
pub struct Config {
    pub components: Components,
    pub settings: Settings,
    pub date: Date,
}

#[derive(Deserialize)]
pub struct Components {
    pub memory: bool,
    pub ethernet: bool,
    pub battery: bool,
    pub volume: bool,
    pub date: bool,
}

#[derive(Deserialize)]
pub struct Settings {
    pub refresh_intervall: i32,
}

#[derive(Deserialize)]
pub struct Date {
    pub format: String,
}

impl Config {
    pub fn new(path: &Path) -> Result<Config, Error> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(toml::from_str(&content[..])?),
            Err(_) => Ok(Config::default()),
        }
    }
}

impl Default for Components {
    fn default() -> Self {
        Self {
            memory: false,
            ethernet: true,
            battery: false,
            volume: false,
            date: true,
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            format: "%H:%M:%S - %d.%m.%Y".to_string(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            refresh_intervall: 1,
        }
    }
}
