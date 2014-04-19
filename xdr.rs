#![crate_id = "xdr#0.1"]
#![crate_type = "lib"]

//! Implementation of unpacking routines for External Data Representation (XDR) format.
//! Follows the RFC at https://tools.ietf.org/html/rfc4506 
//! Copyright 2014 Charith Ellawala: charith {at} lucideelectricdreams {dot} com
pub mod xdr {
    use std::str;
    use std::cast;

    static PADDING_MULTIPLIER : uint = 4;
    static BYTE_LEN : uint = 8;
    static INT_BYTES : uint = 4;
    static HYPERINT_BYTES : uint  = 8;

    /// Struct holding a buffer of bytes encoded using XDR
    pub struct Xdr<'r> {
        buffer: &'r[u8],
        curr_pos: uint,
        size: uint
    }

    pub enum XdrDataType {
        UINT,
        INT,
        HYPERINT,
        UHYPERINT,
        BOOL,
        STRING
    }

    pub trait XdrPrimitive {}
    impl XdrPrimitive for u32 {}
    impl XdrPrimitive for i32 {}
    impl XdrPrimitive for u64 {}
    impl XdrPrimitive for i64 {}
    impl XdrPrimitive for bool {}
    impl XdrPrimitive for ~str {}

    impl <'r> Xdr<'r> {
        /// Create a new instance of a reader using the provided byte vector. 
        /// Call the `unpack_*` methods on the returned struct to consume the data
        pub fn new(data : &'r[u8]) -> Xdr<'r> {
            Xdr { buffer: data, curr_pos: 0, size: data.len() }
        }

        /// Read a value of the given type
        pub unsafe fn unpack_value<T:XdrPrimitive>(&mut self, data_type: XdrDataType) -> Option<T> {
            match data_type {
                UINT => self.unpack_uint().and_then(|v| Some(cast::transmute(v))),
                INT  => self.unpack_int().and_then(|v| Some(cast::transmute(v))),
                HYPERINT => self.unpack_hyperint().and_then(|v| Some(cast::transmute(v))),
                UHYPERINT => self.unpack_uhyperint().and_then(|v| Some(cast::transmute(v))),
                BOOL => self.unpack_bool().and_then(|v| Some(cast::transmute(v))),
                STRING => self.unpack_string().and_then(|v| Some(cast::transmute(v))),
            }
        }


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

        /// Read an array of data
        pub unsafe fn unpack_fixed_array<T:XdrPrimitive+Clone>(&mut self, data_type: XdrDataType, array_len: uint) -> Option<~[T]> {
            let mut item_array = Vec::with_capacity(array_len);
            for _ in range(0,array_len) {
                let item: Option<T> = self.unpack_value(data_type);
                if item.is_some() {
                    item_array.push(item.unwrap());
                }
                else {
                    return None
                }
            }

            Some(item_array.slice_from(0).to_owned())
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
    }
}

mod xdr_tests;
