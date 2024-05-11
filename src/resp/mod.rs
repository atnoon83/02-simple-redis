#![allow(dead_code)]

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use enum_dispatch::enum_dispatch;

mod encode;

/*
- 如何解析 Frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - bulk error: "!<length>\r\n<error>\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk string: "$<length>\r\n<data>\r\n"
    - null bulk string: "$-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
    - null array: "*-1\r\n"
    - null: "_\r\n"
    - boolean: "#<t|f>\r\n"
    - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
 */

#[enum_dispatch]
pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(data: &[u8]) -> Result<RespFrame, String>;
}

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    BulkError(BulkError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct SimpleString {
    pub data: String,
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct SimpleError {
    pub data: String,
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct BulkError {
    pub data: String,
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct BulkString {
    pub data: Vec<u8>,
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct RespNullBulkString;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct RespArray {
    pub data: Vec<RespFrame>,
}

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct RespNullArray;

#[derive(Debug, PartialOrd, Eq, PartialEq)]
pub struct RespNull;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct RespMap {
    pub data: BTreeMap<String, RespFrame>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct RespSet {
    pub data: Vec<RespFrame>,
}

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Deref for BulkError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl SimpleString {
    pub fn new(data: impl Into<String>) -> Self {
        SimpleString { data: data.into() }
    }
}

impl SimpleError {
    pub fn new(data: impl Into<String>) -> Self {
        SimpleError { data: data.into() }
    }
}

impl BulkError {
    pub fn new(data: impl Into<String>) -> Self {
        BulkError { data: data.into() }
    }
}

impl BulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        BulkString { data: data.into() }
    }
}

impl RespArray {
    pub fn new(data: impl Into<Vec<RespFrame>>) -> Self {
        RespArray { data: data.into() }
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap {
            data: BTreeMap::new(),
        }
    }
}

impl Default for RespMap {
    fn default() -> Self {
        Self::new()
    }
}

impl RespSet {
    pub fn new(data: impl Into<Vec<RespFrame>>) -> Self {
        RespSet { data: data.into() }
    }
}
