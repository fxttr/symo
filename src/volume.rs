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

use std::fs::File;

use nix::{libc::ioctl, ioctl_read};

pub struct Volume {
    // See sys/soundcard.h:1009
    devices: [&'static str; 25],
    mixer: File
}

impl Volume {
    pub fn new() -> Self {
	let mixer = File::open("/dev/mixer").expect("Could not open mixer file.");
	
	Self {
	    devices: ["vol", "bass", "treble", "synth", "pcm", "speaker", "line", 
		      "mic", "cd", "mix", "pcm2", "rec", "igain", "ogain", 
		      "line1", "line2", "line3", "dig1", "dig2", "dig3", 
		      "phin", "phout", "video", "radio", "monitor"],
	    mixer
	}
    }

    pub fn read(&self) -> i64 {
	let result: i64 = -1;
	const SPI_IOC_MAGIC: u8 = b'M';
	const SPI_IOC_TYPE_MODE: u8 = 1;
	
	ioctl_read!(result, SPI_IOC_MAGIC, SPI_IOC_TYPE_MODE, i64);
	
	result & 0x7f
    }

    fn detect(&self) -> usize {
	self.devices.iter().position(|&x| x == "vol").expect("Could not find device. This should not happen!")
    }
}
