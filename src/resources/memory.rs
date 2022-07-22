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

use sysctl::Sysctl;
use sysctl::Ctl;

use crate::monitor::Monitor;

pub struct Memory {
    
}

impl Memory {
    pub fn new() -> Self {
	Self {}
    }

    fn get_value(&self, ctl: Ctl) -> u64 {
	ctl.value_string().unwrap().parse::<u64>().unwrap()
    }
}

impl Monitor for Memory {
    fn read(&mut self) -> String {
	let physmem = sysctl::Ctl::new("hw.physmem").unwrap();
	let pagesize = sysctl::Ctl::new("hw.pagesize").unwrap();
	let inactive = sysctl::Ctl::new("vm.stats.vm.v_inactive_count").unwrap();
	let cache = sysctl::Ctl::new("vm.stats.vm.v_cache_count").unwrap();
	let free = sysctl::Ctl::new("vm.stats.vm.v_free_count").unwrap();

	let mem_all = self.get_value(physmem);
	let page_size = self.get_value(pagesize);
	
	let mem_inactive = self.get_value(inactive) * page_size;
	let mem_cache = self.get_value(cache) * page_size;
	let mem_free = self.get_value(free) * page_size;

	let total = mem_all - (mem_inactive + mem_cache + mem_free);

	((total as f64 / mem_all as f64) * 100.0).to_string()
    }
}
