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

use std::{fs::File, os::unix::prelude::AsRawFd};

use crate::monitor::Monitor;
#[cfg(target_os = "linux")]
use nix::libc::ioctl;
#[cfg(target_os = "freebsd")]
use nix::libc::request_code_read;

#[cfg(target_os = "freebsd")]
pub struct Volume {
    mixer: File,
}

#[cfg(target_os = "linux")]
pub struct Volume {}

#[cfg(target_os = "freebsd")]
impl Volume {
    pub fn new() -> Self {
        let mixer = File::open("/dev/mixer").expect("Could not open mixer file.");

        Self { mixer }
    }
}

#[cfg(target_os = "linux")]
impl Volume {
    pub fn new() -> Self {
        Self {}
    }
}

impl Monitor for Volume {
    #[cfg(target_os = "freebsd")]
    fn read(&mut self) -> String {
        const SPI_IOC_MAGIC: u8 = b'M';

        let result: i32 = -1;
        let device: u8 = 0; // We always want "vol". See sys/soundcard.sh:1009

        unsafe {
            ioctl(
                self.mixer.as_raw_fd(),
                request_code_read!(SPI_IOC_MAGIC, device, std::mem::size_of::<i32>()),
                &result,
            );
            (result & 0x7f).to_string()
        }
    }

    #[cfg(target_os = "linux")]
    fn read(&mut self) -> String {
        "Not yet implemented".to_owned()
    }
}
