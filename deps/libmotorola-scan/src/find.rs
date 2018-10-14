
use super::ffi;
use super::cstr_to_str;
use super::{to_result, Error};

/// Structure representing an enumerated scanner
pub struct FindInfo(ffi::tagSCAN_FINDINFO_A);
impl ::core::fmt::Debug for FindInfo {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		write!(f, "FindInfo {{ szDeviceName: {:?}, szPortName: {:?}, szFriendlyName: {:?}, szRegistryBasePath: {:?} }}",
			self.device_name(), self.port_name(), self.friendly_name(), self.registry_base_path()
			)
	}
}
impl FindInfo
{
	pub fn device_name(&self) -> &str {
		cstr_to_str(&self.0.szDeviceName)
	}
	pub fn port_name(&self) -> &str {
		cstr_to_str(&self.0.szPortName)
	}
	pub fn friendly_name(&self) -> &str {
		cstr_to_str(&self.0.szFriendlyName)
	}
	pub fn registry_base_path(&self) -> &str {
		cstr_to_str(&self.0.szRegistryBasePath)
	}
}

/// Create an iterator over available scanner devices
pub fn enumerate_devices() -> Result<FindIterator,Error>
{
	// SAFE: Zero is valid for this C structure
	let mut info: ffi::tagSCAN_FINDINFO_A = unsafe { ::core::mem::zeroed() };
	ffi::SI_INIT!(&mut info);
	let mut handle = ::core::ptr::null_mut();
	// SAFE: Hopefully-valid FFI
	let res = unsafe { ffi::SCAN_FindFirst_A(&mut info, &mut handle) };
	match to_result(res)
	{
	Ok( () ) => Ok(FindIterator {
		handle: handle,
		info: info,
		next_valid: true,
		}),
	Err(Error::NoMoreItems) => {
		// TODO Does this need to call SCAN_FindClose? It will, but is that a problem?
		Ok(FindIterator {
			handle: handle,
			info: info,
			next_valid: false,
			})
		}
	Err(v) => {
		Err(v)
		}
	}
}

/// Iterator over available scanners, returned by self::enumerate_devices
pub struct FindIterator
{
	handle: ffi::HANDLE,
	info: ffi::SCAN_FINDINFO_A,
	next_valid: bool,
}
impl Iterator for FindIterator
{
	type Item = Result<FindInfo, Error >;
	fn next(&mut self) -> Option< Result<FindInfo, Error> >
	{
		if self.next_valid
		{
			self.next_valid = false;
			let rv = FindInfo(self.info.clone());
			match to_result(unsafe { ffi::SCAN_FindNext_A(&mut self.info, self.handle) })
			{
			Ok( () ) => { self.next_valid = true; },
			Err(Error::NoMoreItems) => {},
			Err(e) => return Some( Err(e) )
			}
		
			Some( Ok(rv) )
		}
		else
		{
			None
		}
	}
}
impl ::core::ops::Drop for FindIterator
{
	fn drop(&mut self) {
		unsafe { ffi::SCAN_FindClose(self.handle); }
	}
}
