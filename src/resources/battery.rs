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
use battery::{Manager, State, units::time::minute};

pub struct Battery {
    manager: Manager,
}

impl Battery {
    pub fn new() -> Self {
        Self {
            manager: Manager::new().unwrap(),
        }
    }
}

impl Monitor for Battery {
    fn read(&mut self) -> String {
        let battery = self.manager.batteries().unwrap().next();

        match battery {
            Some(battery) => match battery {
                Ok(battery) => {
                    if battery.state() == State::Charging {
                        match battery.time_to_full() {
                            Some(time) => format!(
                                "{:.2}% : {:.0} Min to full",
                                battery.state_of_charge().value * 100.0,
                                time.get::<minute>()
                            ),
                            None => format!(
                                "{:.2}% : ? Min to full",
                                battery.state_of_charge().value * 100.0,
                            ),
                        }
                    } else {
                        format!("{:.2}%", battery.state_of_charge().value * 100.0)
                    }
                }
                Err(_) => String::from("No Battery found"),
            },
            None => String::from("No Battery found"),
        }
    }
}
