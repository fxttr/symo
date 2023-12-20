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

use crate::monitor::Monitor;
use pipewire::spa::ReadableDict;
use pipewire::types::ObjectType;
use pipewire::{Context, MainLoop};
use std::sync::Arc;
use std::thread;

pub struct Volume {
    volume: Arc<String>,
}


impl Volume {
    pub fn new() -> Self {
        let mut result = Self {
            volume: Arc::new(String::from("-")),
        };

        result.start_main_loop();

        result
    }

    fn start_main_loop(&mut self) -> () {
        thread::spawn(move || {
        let main_loop = MainLoop::new().expect("Failed to create main loop");
        let context = Context::new(&main_loop).expect("Failed to create context");
        let core = context.connect(None).unwrap();
        let registry = core.get_registry().expect("Failed to create registry");

        let _ = registry
            .add_listener_local()
            .global(move |global| {
                if global.type_ == ObjectType::Port {
                    let props = global.props.as_ref().unwrap();
                    let test = props.get("port.name").unwrap();
                    println!("{}", test.to_owned());
                }
            })
            .register();

        main_loop.run();
    });
    }
}

impl Monitor for Volume {
    fn read(&mut self) -> String {
        self.volume.to_string()
    }
}
