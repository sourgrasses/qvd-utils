use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq)]
pub enum QvdSymbol<'a> {
    Ascii(&'a str),
    Date(NaiveDate),
    Money,
    Real(f64),
    Timestamp(NaiveTime),
}

#[derive(Debug, Deserialize)]
pub struct QvdTableHeader {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "CreatorDoc")]
    pub creator_doc: String,
    #[serde(rename = "Fields")]
    pub fields: Fields,
    #[serde(rename = "NoOfRecords")]
    pub no_of_records: u32,
    #[serde(rename = "RecordByteSize")]
    pub record_byte_size: usize,
    #[serde(rename = "Offset")]
    pub offset: usize,
    #[serde(rename = "Length")]
    pub length: usize,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "$value", default)]
    pub headers: Vec<QvdFieldHeader>,
}

#[derive(Debug, Deserialize)]
pub struct QvdFieldHeader {
    #[serde(rename = "FieldName")]
    pub field_name: String,
    #[serde(rename = "Offset")]
    pub offset: usize,
    #[serde(rename = "Length")]
    pub length: usize,
    #[serde(rename = "BitOffset")]
    pub bit_offset: usize,
    #[serde(rename = "BitWidth")]
    pub bit_width: usize,
    #[serde(rename = "Bias")]
    pub bias: i32,
    // #[serde(rename = "NumberFormat")]
    // pub number_format: NumberFormat,
}

#[derive(Debug, Deserialize)]
pub enum FieldType {
    #[serde(rename = "ASCII")]
    Ascii,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "MONEY")]
    Money,
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "TIMESTAMP")]
    Timestamp,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct NumberFormat {
    #[serde(rename = "Type")]
    pub field_type: FieldType,
    #[serde(rename = "nDec")]
    pub ndec: i32,
    #[serde(rename = "UseThou")]
    pub use_thou: bool,
    #[serde(rename = "Fmt")]
    pub fmt: String,
    #[serde(rename = "Dec")]
    pub dec: Option<char>,
    #[serde(rename = "Thou")]
    pub thou: Option<String>,
}
