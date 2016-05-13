// This file is part of rust1. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust1/master/COPYRIGHT. No part of rust1, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust1. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust1/master/COPYRIGHT.


use std::time::Duration;


// A 64-bit number, signed or unsigned, can accommodate 1,000 years in nanoseconds
pub type UnsignedNanosecond = u64;

pub type SignedNanosecond = i64;

pub fn nanoseconds_to_duration(nanoseconds: UnsignedNanosecond) -> Duration
{
	let seconds = nanoseconds / 1_000_000_000;
	let nanoseconds = (nanoseconds % 1_000_000_000) as u32;
	Duration::new(seconds, nanoseconds)
}
