use std::time::{SystemTime, UNIX_EPOCH};

use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

pub mod header;

pub const VERSION: &str = "2.12.4";

pub type BlockId = String;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockData {
    Header(header::HeaderData),
}

impl BlockData {
    pub fn r#type(&self) -> &str {
        match self {
            BlockData::Header(_) => header::HEADER_BLOCK_TYPE,
        }
    }
}

/// A EditorJS editor content block
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub(crate) id: BlockId,
    pub(crate) r#type: String,
    pub(crate) data: BlockData,
}

impl Block {
    pub fn new(data: BlockData) -> Self {
        Block {
            id: Self::generate_id(),
            r#type: data.r#type().to_string(),
            data,
        }
    }

    pub fn id(&self) -> BlockId {
        self.id.clone()
    }

    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

    pub fn data(&self) -> BlockData {
        self.data.clone()
    }

    fn generate_id() -> BlockId {
        let mut rng = rand::thread_rng();
        let id: String = rand::distributions::Alphanumeric
            .sample_iter(&mut rng)
            .take(10)
            .map(char::from)
            .collect();

        id
    }
}

/// Represtents the top-level object for the EditorJS output
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scheme {
    time: u128,
    version: &'static str,
    blocks: Vec<Block>,
}

impl Scheme {
    pub fn new() -> Self {
        Scheme {
            time: Self::timestamp(),
            version: VERSION,
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn timestamp() -> u128 {
        let start = SystemTime::now();

        start.duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}
