extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=SCNAPI32");

	let mut builder = bindgen::Builder::default()
		.clang_arg("-I../../ThirdParty/EMDK/Include/ArmV4")
        .header("bindgen-wrapper.h")
		;
	for v in [
		"SCANNER_TYPE_LASER",
		"SCANNER_TYPE_IMAGER",
		"SCANNER_TYPE_CAMERA",
		"TRANSPORT_TYPE_INTERNAL",
		"TRANSPORT_TYPE_SERIAL_SSI",
		"TRANSPORT_TYPE_BLUETOOTH_SSI",
		].iter()
	{
		builder = builder.whitelisted_var(v);
	}
	for f in [
		"GetLastError",
		"SCAN_FindFirst_A", "SCAN_FindNext_A",
		"SCAN_FindFirst_W", "SCAN_FindNext_W",
		"SCAN_FindClose",
		"SCAN_Open", "SCAN_Close", "SCAN_Enable", "SCAN_Disable",
		"SCAN_GetVersionInfo", "SCAN_GetDeviceInfo",
		"SCAN_GetSupportedDecoders", "SCAN_GetEnabledDecoders", "SCAN_SetEnabledDecoders",
		"SCAN_GetDecoderParams", "SCAN_SetDecoderParams", "SCAN_GetUPCEANParams", "SCAN_SetUPCEANParams",
		"SCAN_GetScanParameters", "SCAN_SetScanParameters",
		"SCAN_GetInterfaceParams", "SCAN_SetInterfaceParams", "SCAN_GetReaderParams", "SCAN_SetReaderParams",

		"SCAN_AllocateBuffer_A", "SCAN_DeallocateBuffer_A",
		"SCAN_AllocateBuffer_W", "SCAN_DeallocateBuffer_W",
		"SCAN_ReadLabelWait_A", "SCAN_ReadLabelEvent_A", "SCAN_ReadLabelMsg_A",
		"SCAN_ReadLabelWait_W", "SCAN_ReadLabelEvent_W", "SCAN_ReadLabelMsg_W", "SCAN_CancelRead",
		"SCAN_GetSoftTrigger", "SCAN_SetSoftTrigger",
		"SCAN_Flush",
		].iter()
	{
		builder = builder.whitelisted_function(f);
	}

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
