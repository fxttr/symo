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

mod config;
mod date;
mod edition;
mod network;
mod volume;
mod jails;
mod resources;

use config::Config;
use jails::Jails;
use resources::memory::Memory;
use x11::xlib::{XOpenDisplay, XRootWindow, XDefaultScreen, _XDisplay, XStoreName, XFlush};
use std::path::Path;
use crate::{date::Date, network::Network, volume::Volume};
use std::time::Duration;
use std::{thread, ptr};
use std::ffi::CString;

fn main() {
    let duration = Duration::from_millis(1000);
    let config: Config = Config::new(Path::new("config.toml")).unwrap();
    let mut network: Network = Network::new();
    let mut jails: Jails = Jails::new();
    let volume: Volume = Volume::new();
    let resources: Memory = Memory::new();
    volume.detect();
    
    unsafe {
	let dpy = XOpenDisplay(ptr::null());
	let screen = XDefaultScreen(dpy);
	let root = XRootWindow(dpy, screen);
	
	loop {
	    let jail_changes = jails.check();

	    if jail_changes != ""
	    {
		show_monitor(&jail_changes, dpy, root);
	    }
	
	    put(&format!("      {} %      {}      {} %      {}",
			 resources.read_memory(),
			 network.get_nics(),
			 volume.read(),
			 Date::get(&config.date.format)),
		dpy, root);
	    
	    thread::sleep(duration);
	}
    }
}

fn put(msg: &str, dpy: *mut _XDisplay, root: u64) {
    unsafe {
	let c_msg = CString::new(msg).unwrap();
	
	XStoreName(dpy, root, c_msg.as_ptr());
	XFlush(dpy);
    }
}

fn show_monitor(msg: &str, dpy: *mut _XDisplay, root: u64) {
    let duration = Duration::from_millis(100);
    let len: usize = 20;
    let mut steps: usize = 1;
    
    if msg.chars().count() >= 20 {
	steps = msg.chars().count() - 20 + 1;
    }

    for step in 0..steps {
	put(&truncate(msg, step, len), dpy, root);

	thread::sleep(duration);
    }

    thread::sleep(Duration::from_millis(2000));
}

fn truncate(msg: &str, start: usize, len: usize) -> String {
    msg.chars().skip(start).take(len).collect::<String>()
}
