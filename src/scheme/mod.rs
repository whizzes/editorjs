use serde::{Deserialize, Serialize};

pub mod header;

use self::header::HeaderData;

/// Block Data Types from standard EditorJS
#[derive(Debug, Deserialize, Serialize)]
pub enum Data {
    Header(HeaderData),
    Paragraph,
    List,
}

/// A trait implemented by every Block from the EditorJS
/// JSON output
pub trait Block {
    fn id(&self) -> String;
    fn r#type(&self) -> &'static str;
    fn data(&self) -> Data;
}

/// Represtents the top-level object for the EditorJS output
#[derive(Debug, Deserialize, Serialize)]
pub struct Scheme {
    time: u64,
    blocks: Vec<Box<dyn Block>>,
    version: &'static str,
}
