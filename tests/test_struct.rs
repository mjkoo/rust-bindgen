use support::assert_bind_eq;

#[test]
fn with_anon_struct() {
    assert_bind_eq("headers/struct_with_anon_struct.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub bar: Struct_Unnamed1,
        }
        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_Unnamed1 {
            pub a: ::libc::c_int,
            pub b: ::libc::c_int,
        }
        impl ::std::clone::Clone for Struct_Unnamed1 {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_struct_array() {
    assert_bind_eq("headers/struct_with_anon_struct_array.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub bar: [Struct_Unnamed1; 2usize],
            pub baz: [[[Struct_Unnamed2; 4usize]; 3usize]; 2usize],
        }

        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }

        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_Unnamed1 {
            pub a: ::libc::c_int,
            pub b: ::libc::c_int,
        }

        impl ::std::clone::Clone for Struct_Unnamed1 {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }

        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_Unnamed2 {
            pub a: ::libc::c_int,
            pub b: ::libc::c_int,
        }

        impl ::std::clone::Clone for Struct_Unnamed2 {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Struct_Unnamed2 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_struct_pointer() {
    assert_bind_eq("headers/struct_with_anon_struct_pointer.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub bar: *mut Struct_Unnamed1,
        }
        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_Unnamed1 {
            pub a: ::libc::c_int,
            pub b: ::libc::c_int,
        }
        impl ::std::clone::Clone for Struct_Unnamed1 {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_union() {
    assert_bind_eq("headers/struct_with_anon_union.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub bar: Union_Unnamed1,
        }
        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        pub struct Union_Unnamed1 {
            pub _bindgen_data_: [u32; 1usize],
        }
        impl Union_Unnamed1 {
            pub unsafe fn a(&mut self) -> *mut ::libc::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::libc::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::clone::Clone for Union_Unnamed1 {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Union_Unnamed1 {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_unnamed_struct() {
    assert_bind_eq("headers/struct_with_anon_unnamed_struct.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub _bindgen_data_1_: [u32; 2usize],
        }
        impl Struct_foo {
            pub unsafe fn a(&mut self) -> *mut ::libc::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::libc::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(4))
            }
        }
        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_anon_unnamed_union() {
    assert_bind_eq("headers/struct_with_anon_unnamed_union.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub _bindgen_data_1_: [u32; 1usize],
        }
        impl Struct_foo {
            pub unsafe fn a(&mut self) -> *mut ::libc::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn b(&mut self) -> *mut ::libc::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
        }
        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_nesting() {
    assert_bind_eq("headers/struct_with_nesting.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_foo {
            pub a: ::libc::c_uint,
            pub _bindgen_data_1_: [u32; 1usize],
        }
        impl Struct_foo {
            pub unsafe fn b(&mut self) -> *mut ::libc::c_uint {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn c1(&mut self) -> *mut ::libc::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn c2(&mut self) -> *mut ::libc::c_ushort {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(2))
            }
            pub unsafe fn d1(&mut self) -> *mut ::libc::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(0))
            }
            pub unsafe fn d2(&mut self) -> *mut ::libc::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(1))
            }
            pub unsafe fn d3(&mut self) -> *mut ::libc::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(2))
            }
            pub unsafe fn d4(&mut self) -> *mut ::libc::c_uchar {
                let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
                ::std::mem::transmute(raw.offset(3))
            }
        }
        impl ::std::clone::Clone for Struct_foo {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn containing_fwd_decl_struct() {
    assert_bind_eq("headers/struct_containing_forward_declared_struct.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_a {
            pub val_a: *mut Struct_b,
        }

        impl ::std::clone::Clone for Struct_a {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Struct_a {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }

        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_b {
            pub val_b: ::libc::c_int,
        }

        impl ::std::clone::Clone for Struct_b {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Struct_b {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_bitfields() {
    assert_bind_eq("headers/struct_with_bitfields.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_bitfield {
            pub _bindgen_bitfield_1_: ::libc::c_ushort,
            pub e: ::libc::c_int,
            pub _bindgen_bitfield_2_: ::libc::c_uint,
            pub _bindgen_bitfield_3_: ::libc::c_uint,
        }

        impl ::std::clone::Clone for Struct_bitfield {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::default::Default for Struct_bitfield {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn with_fwd_decl_struct() {
    assert_bind_eq("headers/forward_declared_struct.h", "
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_a {
            pub b: ::libc::c_int,
        }
        impl ::std::clone::Clone for Struct_a {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_a {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Copy)]
        pub struct Struct_c {
            pub d: ::libc::c_int,
        }
        impl ::std::clone::Clone for Struct_c {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_c {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}


#[test]
fn packed_struct() {
    assert_bind_eq("headers/struct_with_packing.h", "
        #[repr(C, packed)]
        #[derive(Copy)]
        pub struct Struct_a {
            pub b: ::libc::c_char,
            pub c: ::libc::c_short,
        }
        impl ::std::clone::Clone for Struct_a {
            fn clone(&self) -> Self { *self }
        }
        impl ::std::default::Default for Struct_a {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

