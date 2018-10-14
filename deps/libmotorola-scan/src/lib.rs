//
//
//
//! Wrapper around FFI bindings to motorola's barcode scanner library (SCNAPI32.lib)
#![no_std]
extern crate libmotorola_scan_sys as ffi;


mod error;
pub use self::error::{Error,to_result};

mod find;
pub use self::find::{enumerate_devices,FindIterator,FindInfo};

fn cstr_to_str(s: &[i8]) -> &str {
	let len = s.iter().position(|&v| v==0).unwrap_or(0);
	::core::str::from_utf8( unsafe { ::core::slice::from_raw_parts(s.as_ptr() as *const u8, len) } ).unwrap()
}

/// Scanner handle
pub struct Scanner
{
	handle: ffi::HANDLE,
}
impl Scanner
{
	pub fn open(port: &str) -> Result<Scanner,Error> {
		//let mut handle = ::core::ptr::null_mut();
		//unsafe { SCAN_Open(port.as_
		panic!("TODO: Scanner::open({:?})", port);
	}


	// TODO: Use type-state tricks to ensure that the scanner is enabled/configured before use

	/// Perform a blocking read from the scanner
	pub fn read_blocking(&mut self, buffer: ScanBuffer, timeout_ms: u32) -> Result<ScanBuffer, Error> {
		// SAFE: Pointers passed are maintained as valid, blocks so lifetime isn't an issue
		match ::to_result(unsafe { ffi::SCAN_ReadLabelWait_A(self.handle, buffer.0, timeout_ms as _) })
		{
		Ok(_) => Ok(buffer),
		Err(Error::ReadTimeout) => Ok(buffer),
		Err(e) => Err(e),
		}
	}
}
impl ::core::ops::Drop for Scanner
{
	fn drop(&mut self)
	{
		unsafe {
			// TODO: Disable/Close optional?
			ffi::SCAN_Disable(self.handle);
			ffi::SCAN_Close(self.handle);
		}
	}
}

/// Buffer used for scanner read operations
pub struct ScanBuffer( *mut ffi::SCAN_BUFFER_A );
impl ScanBuffer
{
	/// Create a new text buffer with capacity of `size` code units (TODO: Is this u16s in with the _W variant?)
	pub fn new_text(size: usize) -> Result<ScanBuffer,Error> {
		Self::new(true, size)
	}
	/// Create a new binary buffer with capacity of `size` bytes
	pub fn new_binary(size: usize) -> Result<ScanBuffer,Error> {
		Self::new(false, size)
	}

	// NOTE: Private, use helpers abov
	fn new(is_text: bool, size: usize) -> Result<ScanBuffer,Error> {
		// SAFE: FFI has no dangerous side-effects, no memory arguments.
		let r = unsafe { ffi::SCAN_AllocateBuffer_A(if is_text { 1 } else { 0 }, size as ffi::DWORD) };
		if r == ::core::ptr::null_mut() {
			// SAFE: FFI, No side-effects, no arguments
			Err( ::to_result( unsafe { ffi::GetLastError() } ).unwrap_err() )
		}
		else {
			Ok( ScanBuffer(r) )
		}
	}

	fn inner(&self) -> &ffi::SCAN_BUFFER_A {
		unsafe { &*self.0 }
	}

	pub fn data(&self) -> &[u8] {
		let p = (self as *const _ as usize + self.inner().dwOffsetDataBuff as usize) as *const u8;
		let len = self.inner().dwDataLength as usize;
		// SAFE: ? This trusts that the structure is kept valid
		unsafe {
			::core::slice::from_raw_parts(p, len)
		}
	}
}
impl ::core::ops::Drop for ScanBuffer
{
	fn drop(&mut self)
	{
		// SAFE: This is the matching dealloctor for SCAN_AllocateBuffer_A, pointer is always valid
		unsafe { ffi::SCAN_DeallocateBuffer_A(self.0) };
	}
}
