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

use std::net;

use interfaces::Interface;

use crate::monitor::Monitor;

pub struct Network {
    interfaces: Vec<Interface>,
    rounds: i32,
}

impl Network {
    pub fn new() -> Self {
        let interfaces = Interface::get_all()
            .expect("Could not get any network Interfaces. Network component disabled.");

        Network {
            interfaces,
            rounds: 0,
        }
    }
}

impl Monitor for Network {
    fn read(&mut self) -> String {
        let mut result: String = String::new();

        for interface in self
            .interfaces
            .iter()
            .filter(|x| x.is_up() && x.is_running() && !x.is_loopback())
        {
            let mut nic_addr: String = String::new();

            interface.addresses.iter().for_each(|x| {
                match x.addr {
                    Some(x) => {
                        nic_addr = match x {
                            net::SocketAddr::V4(ref y) => format!("{}", y.ip()),
                            net::SocketAddr::V6(ref y) => format!("{}", y.ip()),
                        }
                    }
                    None => {}
                };
            });

            if nic_addr.is_empty() {
                continue;
            }

            result = result + " " + &interface.name + " " + &nic_addr
        }

        self.rounds = (self.rounds + 1) % 10;

        result
    }
}
