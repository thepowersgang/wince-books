#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// NOTE: Bindgen doesn't import these properly use to them using a macro
macro_rules! scan_error { ($code:expr) => {(0x80000000 /*ERROR_BIT*/ | 0x20000000 /*USER_BIT*/ | $code )} }
pub const E_SCN_SUCCESS				: DWORD = 0;
pub const E_SCN_OPENINGACTIVEKEY	: DWORD = scan_error!(0x0001);
pub const E_SCN_READINGACTIVEKEY	: DWORD = scan_error!(0x0002);
pub const E_SCN_OPENINGPARAMKEY		: DWORD = scan_error!(0x0003);
pub const E_SCN_READINGPARAMKEY		: DWORD = scan_error!(0x0004);
pub const E_SCN_NOTENOUGHMEMORY		: DWORD = scan_error!(0x0005);
pub const E_SCN_INVALIDDVCCONTEXT	: DWORD = scan_error!(0x0006);
pub const E_SCN_INVALIDOPENCONTEXT	: DWORD = scan_error!(0x0007);
pub const E_SCN_NOTINITIALIZED		: DWORD = scan_error!(0x0008);
pub const E_SCN_CANTLOADDEVICE		: DWORD = scan_error!(0x0009);
pub const E_SCN_INVALIDDEVICE		: DWORD = scan_error!(0x000A);
pub const E_SCN_DEVICEFAILURE		: DWORD = scan_error!(0x000B);
pub const E_SCN_CANTSTARTDEVICE		: DWORD = scan_error!(0x000C);
pub const E_SCN_CANTGETDEFAULTS		: DWORD = scan_error!(0x000D);
pub const E_SCN_NOTSTARTED			: DWORD = scan_error!(0x000E);
pub const E_SCN_ALREADYSTARTED		: DWORD = scan_error!(0x000F);
pub const E_SCN_NOTENABLED			: DWORD = scan_error!(0x0010);
pub const E_SCN_ALREADYENABLED		: DWORD = scan_error!(0x0011);
pub const E_SCN_INVALIDIOCTRL		: DWORD = scan_error!(0x0012);
pub const E_SCN_NULLPTR				: DWORD = scan_error!(0x0013);
pub const E_SCN_INVALIDARG			: DWORD = scan_error!(0x0014);
pub const E_SCN_BUFFERSIZEIN		: DWORD = scan_error!(0x0015);
pub const E_SCN_BUFFERSIZEOUT		: DWORD = scan_error!(0x0016);
pub const E_SCN_STRUCTSIZE			: DWORD = scan_error!(0x0017);
pub const E_SCN_MISSINGFIELD		: DWORD = scan_error!(0x0018);
pub const E_SCN_INVALIDHANDLE		: DWORD = scan_error!(0x0019);
pub const E_SCN_INVALIDPARAM		: DWORD = scan_error!(0x001A);
pub const E_SCN_CREATEEVENT			: DWORD = scan_error!(0x001B);
pub const E_SCN_CREATETHREAD		: DWORD = scan_error!(0x001C);
pub const E_SCN_READCANCELLED		: DWORD = scan_error!(0x001D);
pub const E_SCN_READTIMEOUT			: DWORD = scan_error!(0x001E);
pub const E_SCN_READNOTPENDING		: DWORD = scan_error!(0x001F);
pub const E_SCN_READPENDING			: DWORD = scan_error!(0x0020);
pub const E_SCN_BUFFERTOOSMALL		: DWORD = scan_error!(0x0021);
pub const E_SCN_INVALIDSCANBUFFER	: DWORD = scan_error!(0x0022);
pub const E_SCN_READINCOMPATIBLE	: DWORD = scan_error!(0x0023);
pub const E_SCN_NOFEEDBACK			: DWORD = scan_error!(0x0024);
pub const E_SCN_RESTART				: DWORD = scan_error!(0x0025);
pub const E_SCN_NOTSUPPORTED		: DWORD = scan_error!(0x0026);
pub const E_SCN_WRONGSTATE			: DWORD = scan_error!(0x0027);
pub const E_SCN_NOMOREITEMS			: DWORD = scan_error!(0x0028);
pub const E_SCN_CANTOPENREGKEY		: DWORD = scan_error!(0x0029);
pub const E_SCN_CANTREADREGVAL		: DWORD = scan_error!(0x002A);
pub const E_SCN_EXCEPTION			: DWORD = scan_error!(0x002B);
pub const E_SCN_WIN32ERROR			: DWORD = scan_error!(0x002C);
pub const E_SCN_ALREADYINUSE		: DWORD = scan_error!(0x002D);
pub const E_SCN_NOTINUSE			: DWORD = scan_error!(0x002E);
pub const E_SCN_CANTLOADDECODERDLL	: DWORD = scan_error!(0x002F);
pub const E_SCN_INVALIDDECODERDLL	: DWORD = scan_error!(0x0030);
pub const E_SCN_INVALIDSYMBOL		: DWORD = scan_error!(0x0031);
pub const E_SCN_INVALIDLICENSE		: DWORD = scan_error!(0x0032);
pub const E_SCN_NOTINSEQUENCE		: DWORD = scan_error!(0x0033);
pub const E_SCN_DUPLICATESYMBOL		: DWORD = scan_error!(0x0034);
pub const E_SCN_CANTLOADHALDLL		: DWORD = scan_error!(0x0035);
pub const E_SCN_INVALIDHALDLL		: DWORD = scan_error!(0x0036);
pub const E_SCN_CANTLOADCOMPRESSIONDLL: DWORD = scan_error!(0x0037);
pub const E_SCN_I2CFAILURE			: DWORD = scan_error!(0x0038);
pub const E_SCN_DEVICEDISABLED		: DWORD = scan_error!(0x0039);
pub const E_SCN_RSMATTRIBINVALID	: DWORD = scan_error!(0x003A);
pub const E_SCN_INVALIDRESPONSE		: DWORD = scan_error!(0x003B);
pub const E_SCN_MISSING_CONFIG		: DWORD = scan_error!(0x003C);
pub const E_SCN_INVALIDFIRMWARE		: DWORD = scan_error!(0x003D);


//#define SI_ALLOC_ALL(ptr)	{ (ptr)->StructInfo.dwAllocated = sizeof(*(ptr)); }
//#define SI_USED_NONE(ptr)	{ (ptr)->StructInfo.dwUsed = sizeof(STRUCT_INFO); }
//#define SI_INIT(ptr)		{ SI_ALLOC_ALL(ptr); SI_USED_NONE(ptr); }
#[macro_export]
macro_rules! SI_INIT {
	($ptr:expr) => {{ let ptr: &mut _ = $ptr; ptr.StructInfo.dwAllocated = ::core::mem::size_of_val(ptr) as _; ptr.StructInfo.dwUsed = ::core::mem::size_of_val(&ptr.StructInfo) as _; }}
}

impl Copy for SCAN_FINDINFO_A {}
impl Clone for SCAN_FINDINFO_A { fn clone(&self) -> Self { *self } }

