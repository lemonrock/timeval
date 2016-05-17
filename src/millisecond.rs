// This file is part of timeval. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/timeval/master/COPYRIGHT. No part of timeval, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of timeval. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/timeval/master/COPYRIGHT.


use std::time::Duration;


pub type UnsignedMillisecond = u64;

pub type SignedMillisecond = i64;

pub fn milliseconds_to_duration(milliseconds: UnsignedMillisecond) -> Duration
{
	let seconds = milliseconds / 1_000;
	let nanoseconds = ((milliseconds % 1_000) * 1_000_000) as u32;
	Duration::new(seconds, nanoseconds)
}
