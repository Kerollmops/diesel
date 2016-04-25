extern crate byteorder;

use std::error::Error;
use std::io::prelude::*;
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use backend::Backend;
use types::{self, FromSql, ToSql, IsNull};

impl<DB: Backend<RawValue=[u8]>> FromSql<types::SmallInt, DB> for i16 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error+Send+Sync>> {
        let mut bytes = not_none!(bytes);
        debug_assert!(bytes.len() <= 2, "Received more than 2 bytes decoding i16. \
                      Was an Integer expression accidentally identified as SmallInt?");
        bytes.read_i16::<BigEndian>().map_err(|e| Box::new(e) as Box<Error+Send+Sync>)
    }
}

impl<DB: Backend> ToSql<types::SmallInt, DB> for i16 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error+Send+Sync>> {
        out.write_i16::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error+Send+Sync>)
    }
}

impl<DB: Backend<RawValue=[u8]>> FromSql<types::Integer, DB> for i32 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error+Send+Sync>> {
        let mut bytes = not_none!(bytes);
        debug_assert!(bytes.len() <= 4, "Received more than 4 bytes decoding i32. \
                      Was a BigInteger expression accidentally identified as Integer?");
        bytes.read_i32::<BigEndian>().map_err(|e| Box::new(e) as Box<Error+Send+Sync>)
    }
}

impl<DB: Backend> ToSql<types::Integer, DB> for i32 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error+Send+Sync>> {
        out.write_i32::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error+Send+Sync>)
    }
}

impl<DB: Backend<RawValue=[u8]>> FromSql<types::BigInt, DB> for i64 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error+Send+Sync>> {
        let mut bytes = not_none!(bytes);
        debug_assert!(bytes.len() <= 8, "Received more than 8 bytes decoding i64. \
                      Was an expression of a different type misidentified as BigInteger?");
        bytes.read_i64::<BigEndian>().map_err(|e| Box::new(e) as Box<Error+Send+Sync>)
    }
}

impl<DB: Backend> ToSql<types::BigInt, DB> for i64 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error+Send+Sync>> {
        out.write_i64::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error+Send+Sync>)
    }
}
