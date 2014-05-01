#![crate_id = "xdr#0.1"]
#![crate_type = "lib"]

//! Implementation of unpacking routines for External Data Representation (XDR) format.
//! Follows the RFC at https://tools.ietf.org/html/rfc4506 
//! Copyright 2014 Charith Ellawala: charith {at} lucideelectricdreams {dot} com
pub mod xdr {
    use std::io::{MemReader,IoResult};

    pub type XdrResult<T> = Result<T,&'static str>;

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

    fn read_val<T>(val: IoResult<T>) -> XdrResult<T> {
        match val {
            Ok(v) => Ok(v),
            Err(e) => Err(e.desc)
        }
    }


    impl Xdr {
        /// Create a new instance of a reader using the provided byte vector. 
        /// Call the `unpack_*` methods on the returned struct to consume the data
        pub fn new(data : &[u8]) -> Xdr {
            Xdr { reader: MemReader::new(Vec::from_slice(data)) }
        }

        pub fn unpack_primitive<T:XdrPrimitive>(&mut self) -> XdrResult<T> {
            XdrPrimitive::read_from_xdr(self, None::<T>)
        }

        /*
        /// Read a UTF-8 string
        pub fn unpack_string(&mut self) -> Option<~str> {
            match self.unpack_var_bytes() {
                Some(slice) => str::from_utf8(slice).and_then(|s| Some(s.to_owned())),
                None => None
            }
        }

        /// Read variable length byte array
        pub fn unpack_var_bytes(&mut self) -> Option<~[u8]> {
            self.unpack_uint().and_then(|len| self.unpack_fixed_bytes(len as uint))
        }

        /// Read fixed length byte array 
        pub fn unpack_fixed_bytes(&mut self, n: uint) -> Option<~[u8]> {
            let padded_length = Xdr::calc_padded_len(n);

            if self.curr_pos + padded_length > self.size {
                return None
            }

            let slice = self.buffer.slice(self.curr_pos, self.curr_pos + n);
            self.curr_pos += padded_length;

            Some(slice.to_owned())
        }

        fn calc_padded_len(n: uint) -> uint {
            let temp = n % PADDING_MULTIPLIER;
            let padding = if temp > 0 { PADDING_MULTIPLIER - temp } else { 0 };
            n + padding
        }

        /// Read a boolean value
        pub fn unpack_bool(&mut self) -> Option<bool> {
            self.unpack_enum(|v| { match v {
                1 => Some(true),
                0 => Some(false),
                _ => None
            }
            })
        }

        /// Read an enum. The convert function must accept an i32 and return the corresponding enum value
        pub fn unpack_enum<E>(&mut self, convert: |val:i32| -> Option<E>) -> Option<E> {
            self.unpack_int().and_then(convert)
        }

        /// Read an unsigned 32-bit integer
        pub fn unpack_uint(&mut self) -> Option<u32> {
            self.unpack_int_type(INT_BYTES)
        }

        /// Read a signed 32-bit integer
        pub fn unpack_int(&mut self) -> Option<i32> {
            self.unpack_int_type(INT_BYTES)
        }

        /// Read an unsigned 64-bit integer
        pub fn unpack_uhyperint(&mut self) -> Option<u64> {
            self.unpack_int_type(HYPERINT_BYTES)
        }

        /// Read a signed 64-bit integer
        pub fn unpack_hyperint(&mut self) -> Option<i64> {
            self.unpack_int_type(HYPERINT_BYTES)
        }

        fn unpack_int_type<T: Int+FromPrimitive>(&mut self, num_bytes: uint) -> Option<T> {
            if self.curr_pos + num_bytes <= self.size {
                let slice = self.buffer.slice(self.curr_pos, self.curr_pos + num_bytes);
                self.curr_pos += num_bytes;
                let mut shift_amt:T = FromPrimitive::from_uint((num_bytes - 1) * BYTE_LEN).unwrap();
                let mut ret_val:T = FromPrimitive::from_uint(0).unwrap();;
                for i in range(0,num_bytes) {
                    let mask:T = FromPrimitive::from_u8(slice[i]).unwrap();
                    ret_val = ret_val | (mask << shift_amt); 
                    shift_amt = shift_amt - FromPrimitive::from_uint(BYTE_LEN).unwrap();
                }
                Some(ret_val)
            }
            else {
                None
            }
        }
    */
    }
}

mod xdr_tests;
