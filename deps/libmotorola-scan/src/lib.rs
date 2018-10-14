//
//
//
#![no_std]
extern crate libmotorola_scan_sys as ffi;

macro_rules! err_list {
	($( $ffi_name:ident => $enum_name:ident : $msg:expr,)*) => {
		#[derive(Debug)]
		pub enum Error
		{
			$($enum_name,)*
			Unknown(u32),
		}
		impl ::core::fmt::Display for Error {
			fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
				match *self
				{
				$( Error::$enum_name => write!(f, "{}", $msg), )*
				Error::Unknown(v) => write!(f, "Unknown error {:#x}", v),
				}
			}
		}
		fn to_result(v: ffi::DWORD) -> Result<(),Error> {
			match v
			{
			ffi::E_SCN_SUCCESS => Ok( () ),
			$( ffi::$ffi_name => Err( Error::$enum_name ), )*
			v @ _ => Err(Error::Unknown(v as u32)),
			}
		}
	}
}
err_list! {
	E_SCN_OPENINGACTIVEKEY => OpeningActiveKey : "An error occurred opening the active driver registry key.",
	E_SCN_READINGACTIVEKEY => ReadingActiveKey : "An error occurred reading the active driver registry key.",
	E_SCN_OPENINGPARAMKEY => OpeningParamKey : "An error occurred opening the registry key containing the driver's settings.",
	E_SCN_READINGPARAMKEY => ReadingParamKey : "An error occurred reading the registry key containing the driver's settings.",
	E_SCN_NOTENOUGHMEMORY => NotEnoughMemory : "An attempt to allocate memory failed.",
	E_SCN_INVALIDDVCCONTEXT => InvalidDvcContext : "An invalid device context ID was used.",
	E_SCN_INVALIDOPENCONTEXT => InvalidOpenContext : "An attempt was made to access an invalid open context.",
	E_SCN_NOTINITIALIZED => NotInitialized : "The driver was accessed before a successful initialization.",
	E_SCN_CANTLOADDEVICE => CantLoadDevice : "The physical device driver (PDD) could not be loaded.",
	E_SCN_INVALIDDEVICE => InvalidDevice : "The physical device driver (PDD) DLL did not contain the required entry points.",
	E_SCN_DEVICEFAILURE => DeviceFailure : "Required device is not present, already in use or not functioning properly.",
	E_SCN_CANTSTARTDEVICE => CantStartDevice : "The device could not be started.",
	E_SCN_CANTGETDEFAULTS => CantGetDefaults : "The default parameters could not be obtained from the physical device driver (PDD). ",
	E_SCN_NOTSTARTED => NotStarted : "An attempt was made to use or stop the scanner device and it was not started. ",
	E_SCN_ALREADYSTARTED => AlreadyStarted : "An attempt was made to start the device when the device was already started.",
	E_SCN_NOTENABLED => NotEnabled : "An attempt was made to access the scanner device and it was not enabled.",
	E_SCN_ALREADYENABLED => AlreadyEnabled : "An attempt was made to enable scanning when scanning was already enabled.",
	E_SCN_INVALIDIOCTRL => InvalidIoCtrl : "The control code passed to DeviceIoControl is invalid.",
	E_SCN_NULLPTR => NullPtr : "A NULL pointer was passed for a required argument.",
	E_SCN_INVALIDARG => InvalidArg : "A passed argument is out of range.",
	E_SCN_BUFFERSIZEIN => BufferSizeIn : "The size of the buffer passed as an input to DeviceIoControl is less than sizeof(STRUCT_INFO) or is less than the size specified in StructInfo.dwUsed.",
	E_SCN_BUFFERSIZEOUT => BufferSizeOut : "The size of the buffer passed as an output to DeviceIoControl is less than sizeof(STRUCT_INFO) or is less than the size specified in StructInfo.dwAllocated.",
	E_SCN_STRUCTSIZE => StructSize : "A STRUCT_INFO structure field is invalid. Either dwAllocated or dwUsed is less than the size of STRUCT_INFO or dwUsed is greater than dwAllocated.",
	E_SCN_MISSINGFIELD => MissingField : "The size of a structure specified in a StructInfo is too small to contain a required field.",
	E_SCN_INVALIDHANDLE => InvalidHandle : "An invalid handle was passed to a function.",
	E_SCN_INVALIDPARAM => InvalidParam : "The value of a parameter either passed as an argument to a function or as a member of a structure is out of range or conflicts with other parameters.",
	E_SCN_CREATEEVENT => CreateEvent : "Unable to create a required event.",
	E_SCN_CREATETHREAD => CreateThread : "Unable to create a required thread.",
	E_SCN_READCANCELLED => ReadCancelled : "A read request was cancelled.",
	E_SCN_READTIMEOUT => ReadTimeout : "A read request timed out.",
	E_SCN_READNOTPENDING => ReadNotPending : "Attempt to cancel a read that is not pending.",

	E_SCN_NOMOREITEMS => NoMoreItems : "No more items are available to be returned from SCAN_FindFirst/SCAN_FindNext.",
}

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
			match ::to_result(unsafe { ffi::SCAN_FindNext_A(&mut self.info, self.handle) })
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

pub fn enumerate_devices() -> Result<FindIterator,Error>
{
	// SAFE: Zero is valid for this C structure
	let mut info: ffi::tagSCAN_FINDINFO_A = unsafe { ::core::mem::zeroed() };
	ffi::SI_INIT!(&mut info);
	let mut handle = ::core::ptr::null_mut();
	// SAFE: Hopefully-valid FFI
	let res = unsafe { ffi::SCAN_FindFirst_A(&mut info, &mut handle) };
	match ::to_result(res)
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


fn cstr_to_str(s: &[i8]) -> &str {
	let len = s.iter().position(|&v| v==0).unwrap_or(0);
	::core::str::from_utf8( unsafe { ::core::slice::from_raw_parts(s.as_ptr() as *const u8, len) } ).unwrap()
}

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
			ffi::SCAN_Disable(self.handle);
			ffi::SCAN_Close(self.handle);
		}
	}
}

pub struct ScanBuffer( *mut ffi::SCAN_BUFFER_A );
impl ScanBuffer
{
	fn new(is_text: bool, size: usize) -> Result<ScanBuffer,Error> {
		// SAFE: FFI, no memory arguments.
		let r = unsafe { ffi::SCAN_AllocateBuffer_A(if is_text { 1 } else { 0 }, size as ffi::DWORD) };
		if r == ::core::ptr::null_mut() {
			// SAFE: FFI, No side-effects, no arguments
			Err( ::to_result( unsafe { ffi::GetLastError() } ).unwrap_err() )
		}
		else {
			Ok( ScanBuffer(r) )
		}
	}
	pub fn new_text(size: usize) -> Result<ScanBuffer,Error> {
		Self::new(true, size)
	}
	pub fn new_binary(size: usize) -> Result<ScanBuffer,Error> {
		Self::new(false, size)
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
		unsafe { ffi::SCAN_DeallocateBuffer_A(self.0) };
	}
}
