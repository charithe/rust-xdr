#![crate_id = "xdr#0.1"]
#![crate_type = "lib"]

//! Implementation of unpacking routines for External Data Representation (XDR) format.
//! Follows the RFC at https://tools.ietf.org/html/rfc4506 
//! To Do:
//!     - Implement quadruple precision floats
//!     - Implement structs and unions
//! Copyright 2014 Charith Ellawala: charith {at} lucideelectricdreams {dot} com
pub mod xdr {
    use std::io::{MemReader,IoResult};
    use std::str;
    use std::vec::Vec;

    pub type XdrResult<T> = Result<T,&'static str>;

    static PADDING_MULTIPLE:uint = 4;

    /// Struct holding a buffer of bytes encoded using XDR
    pub struct Xdr {
        reader: MemReader
    }

    pub trait XdrPrimitive {
        fn read_from_xdr(x: &mut Xdr, _: Option<Self>) -> XdrResult<Self>;
    }

    impl XdrPrimitive for u32 {
        fn read_from_xdr(x: &mut Xdr, _: Option<u32>) -> XdrResult<u32> {
            read_val(x.reader.read_be_u32()) 
        }
    }

    impl XdrPrimitive for i32 {
        fn read_from_xdr(x: &mut Xdr, _:Option<i32>) -> XdrResult<i32> {
            read_val(x.reader.read_be_i32()) 
        }
    }

    impl XdrPrimitive for u64 {
        fn read_from_xdr(x: &mut Xdr, _: Option<u64>) -> XdrResult<u64> {
            read_val(x.reader.read_be_u64())
        }
    }

    impl XdrPrimitive for i64 {
        fn read_from_xdr(x: &mut Xdr, _:Option<i64>) -> XdrResult<i64> {
            read_val(x.reader.read_be_i64()) 
        }
    }

    impl XdrPrimitive for f32 {
        fn read_from_xdr(x: &mut Xdr, _:Option<f32>) -> XdrResult<f32> {
            read_val(x.reader.read_be_f32()) 
        }
    }

    impl XdrPrimitive for f64 {
        fn read_from_xdr(x: &mut Xdr, _:Option<f64>) -> XdrResult<f64> {
            read_val(x.reader.read_be_f64()) 
        }
    }

    impl XdrPrimitive for bool {
        fn read_from_xdr(x: &mut Xdr, _:Option<bool>) -> XdrResult<bool> {
            match read_val(x.reader.read_be_u32()) {
                Ok(0) => Ok(false),
                Ok(1) => Ok(true),
                Ok(_) => Err("Boolean values must be between 0 and 1"),
                Err(e) => Err(e)
            }
        }
    }

    fn read_val<T>(val: IoResult<T>) -> XdrResult<T> {
        match val {
            Ok(v) => Ok(v),
            Err(e) => Err(e.desc)
        }
    }


    impl Xdr {
        
        /// Create a new instance of a reader using the provided byte vector. 
        pub fn new(data : &[u8]) -> Xdr {
            Xdr { reader: MemReader::new(Vec::from_slice(data)) }
        }

        /// Read a primitive (u32, i32, u64, i64, f32 and f64) type from the buffer
        pub fn unpack_primitive<T:XdrPrimitive>(&mut self) -> XdrResult<T> {
            XdrPrimitive::read_from_xdr(self, None::<T>)
        }

        /// Read a 32-bit unsigned integer
        pub fn unpack_uint(&mut self) -> XdrResult<u32> {
            self.unpack_primitive()
        }

        /// Read a 32-bit signed integer
        pub fn unpack_int(&mut self) -> XdrResult<i32> {
            self.unpack_primitive()
        }

        /// Read a 64-bit unsigned integer 
        pub fn unpack_ulong(&mut self) -> XdrResult<u64> {
            self.unpack_primitive()
        }

        /// Read a 64 bit signed integer
        pub fn unpack_long(&mut self) -> XdrResult<i64> {
            self.unpack_primitive()
        }

        /// Read a 32-bit float
        pub fn unpack_float(&mut self) -> XdrResult<f32> {
            self.unpack_primitive()
        }

        /// Read a 64-bit double
        pub fn unpack_double(&mut self) -> XdrResult<f64> {
            self.unpack_primitive()
        }

        /// Read a boolean
        pub fn unpack_boolean(&mut self) -> XdrResult<bool> {
            self.unpack_primitive()
        }

        /// Read a byte array of the specified length
        pub fn unpack_bytes(&mut self, num_bytes: uint) -> XdrResult<~[u8]> {
            let unpack_len = if num_bytes % PADDING_MULTIPLE != 0 {
                num_bytes + (PADDING_MULTIPLE - (num_bytes % PADDING_MULTIPLE))
            }
            else {
                num_bytes
            };

            match read_val(self.reader.read_exact(unpack_len)) {
                Ok(v) => Ok(v.slice_to(num_bytes).to_owned()),
                Err(e) => Err(e)
            }
        }

        /// Read a variable length byte array
        pub fn unpack_varlen_bytes(&mut self) -> XdrResult<~[u8]> {
            let len_result:XdrResult<u32> = self.unpack_primitive();
            match len_result {
                Ok(len) => self.unpack_bytes(len as uint),
                Err(e) => Err(e)
            }
        }

        /// Read a UTF-8 string
        pub fn unpack_string(&mut self) -> XdrResult<~str> {
            match self.unpack_varlen_bytes() {
                Ok(slice) => match str::from_utf8_owned(slice) { 
                    Some(s) => Ok(s),
                    None => Err("Failed to create string")
                },
                Err(e) => Err(e)
            }
        }

        /// Unpack an array of primitives with a known length
        pub fn unpack_array<T:XdrPrimitive + Clone>(&mut self, num_elements: uint) -> XdrResult<~[T]> {
            /* the elegant solution is to do this:
                Vec::from_fn(num_elements, |_| {  self.unpack_primitive().unwrap()  });
              but I am not aware of a way to handle error conditions in the closure and return early from the method
            */
            let mut tmp_vec:Vec<T> = Vec::with_capacity(num_elements);
            for _ in range(0,num_elements) {
                tmp_vec.push(try!(self.unpack_primitive()))
            }

            Ok(tmp_vec.as_slice().to_owned())
        }

        /// Unpack an array of primitives with an unknown length
        pub fn unpack_varlen_array<T:XdrPrimitive + Clone>(&mut self) -> XdrResult<~[T]> {
            let len:u32 = try!(self.unpack_primitive());
            self.unpack_array(len as uint)
        }
    }
}

mod xdr_tests;
