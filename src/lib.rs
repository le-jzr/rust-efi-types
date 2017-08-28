#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![no_std]

#![feature(const_fn)]
#![feature(untagged_unions)]

mod ctypes {
    pub type c_void = ();
    pub type c_char = u8;
    pub type c_uchar = u8;
    pub type c_schar = i8;
    pub type c_ushort = u16;
    pub type c_short = i16;
    pub type c_uint = u32;
    pub type c_int = i32;
    pub type c_ulong = u64;
    pub type c_long = i64;
    pub type c_ulonglong = u64;
    pub type c_longlong = i64;
}

// Variadic functions -- Rust doesn't know how to call them.
type EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = usize;
type EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES = usize;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

