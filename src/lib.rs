// This file is part of timeval. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/timeval/master/COPYRIGHT. No part of timeval, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of timeval. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/timeval/master/COPYRIGHT.


extern crate libc;
extern crate time;
use self::libc::timeval;
use self::time::Timespec;

pub use millisecond::UnsignedMillisecond;
pub use millisecond::SignedMillisecond;
pub mod millisecond;

pub use microsecond::UnsignedMicrosecond;
pub use microsecond::SignedMicrosecond;
pub mod microsecond;

pub use nanosecond::UnsignedNanosecond;
pub use nanosecond::SignedNanosecond;
pub mod nanosecond;

pub fn timeval_to_timespec(value: timeval) -> Timespec
{
	Timespec
	{
		sec: value.tv_sec as i64,
		nsec: (value.tv_usec * 1000) as i32, 
	}
}

/// Can not use the Default trait
#[inline(always)]
pub fn default() -> timeval
{
	timeval
	{
		tv_sec: 0,
		tv_usec: 0
	}
}

pub fn to_milliseconds_rounded_down(value: timeval) -> UnsignedMillisecond
{
	debug_assert!(value.tv_sec >= 0, "We do not support negative timevals (seconds are negative)");
	debug_assert!(value.tv_usec >= 0, "We do not support negative timevals (microseconds are negative)");
	debug_assert!(value.tv_usec <= 1_000_000, "We do not support timevals with microseconds in excess of 1,000,000");
	
	(value.tv_sec as u64 * 1_000 + ((value.tv_usec as u64) / 1_000)) as UnsignedMillisecond
}

pub fn to_microseconds(value: timeval) -> UnsignedMicrosecond
{
	debug_assert!(value.tv_sec >= 0, "We do not support negative timevals (seconds are negative)");
	debug_assert!(value.tv_usec >= 0, "We do not support negative timevals (microseconds are negative)");
	debug_assert!(value.tv_usec <= 1_000_000, "We do not support timevals with microseconds in excess of 1,000,000");
	
	(value.tv_sec as u64 * 1_000_000 + value.tv_usec as u64) as UnsignedMicrosecond
}

pub fn to_nanoseconds(value: timeval) -> UnsignedNanosecond
{
	debug_assert!(value.tv_sec >= 0, "We do not support negative timevals (seconds are negative)");
	debug_assert!(value.tv_usec >= 0, "We do not support negative timevals (microseconds are negative)");
	debug_assert!(value.tv_usec <= 1_000_000, "We do not support timevals with microseconds in excess of 1,000,000");
	
	value.tv_sec as u64 * 1_000_000_000 + ((value.tv_usec as u64) * 1_000) as UnsignedNanosecond
}

pub fn difference_in_microseconds(left: timeval, right: timeval) -> SignedMicrosecond
{
	let left_microseconds = to_microseconds(left) as SignedMicrosecond;
	let right_microseconds = to_microseconds(right) as SignedMicrosecond;
	left_microseconds - right_microseconds
}
