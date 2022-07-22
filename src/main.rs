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
mod jails;
mod network;
mod resources;
mod volume;
mod monitor;

use crate::{date::Date, network::Network, volume::Volume};
use config::Config;
use jails::Jails;
use monitor::Monitor;
use resources::battery::Battery;
use resources::memory::Memory;
use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;
use std::time::Duration;
use std::{ptr, thread};
use x11::xlib::{XDefaultScreen, XFlush, XOpenDisplay, XRootWindow, XStoreName, _XDisplay};

fn main() {
    let config: Config = Config::new(Path::new("/usr/local/etc/symo.toml")).unwrap();
    let duration = Duration::from_millis(config.settings.refresh_intervall as u64 * 1000);
    let mut network: Network = Network::new();
    let mut jails: Jails = Jails::new();
    let mut volume: Volume = Volume::new();
    let mut memory: Memory = Memory::new();
    let mut battery: Battery = Battery::new();
    let mut date: Date = Date::new(&config.date.format);

    let mut update_vec: HashMap<(char, char), &dyn Monitor> = HashMap::new();

    watch(&mut update_vec, ('','%'), &memory, config.components.memory);
    watch(&mut update_vec, ('',' '), &network, config.components.ethernet);
    watch(&mut update_vec, ('',' '), &battery, config.components.battery);
    watch(&mut update_vec, ('','%'), &volume, config.components.volume);
    watch(&mut update_vec, ('',' '), &date, config.components.date);
    
    
    unsafe {
        let dpy = XOpenDisplay(ptr::null());
        let screen = XDefaultScreen(dpy);
        let root = XRootWindow(dpy, screen);

        loop {
            let jail_changes = jails.read();
	    let mut msg: String = String::new();
	    msg = msg + "     ";
	    
            if jail_changes != "" {
                show_monitor(&jail_changes, dpy, root);
            }

	    for ((icon, suffix), module) in &update_vec {
		msg = msg + &add(&module.read(), *icon, *suffix);
	    }

            put(
                &msg,
                dpy,
                root,
            );

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

fn add(module: &str, icon: char, suffix: char) -> String {
    icon.to_string() + " " + module + " " + &suffix.to_string() + "     "
}

fn watch(map: &mut HashMap<(char, char), &dyn Monitor>, format_pair: (char, char), module: &'static dyn Monitor, option: bool) {
    if option {
	map.insert(format_pair, module);
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
