//! Result (error) types
use super::ffi;

macro_rules! err_list {
	($( $ffi_name:ident => $enum_name:ident : $msg:expr,)*) => {
		#[derive(Debug)]
		pub enum Error
		{
			$(
			#[doc=$msg]
			$enum_name,
			)*
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
		pub fn to_result(v: ffi::DWORD) -> Result<(),Error> {
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
