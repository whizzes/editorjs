use serde::{Deserialize, Serialize};

use super::{Block, Data};

pub const HEADER_BLOCK_TYPE: &str = "header";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeaderData {
    text: String,
    level: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeaderBlock {
    id: String,
    r#type: &'static str,
    data: HeaderData,
}

impl Block for HeaderBlock {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn r#type(&self) -> &'static str {
        HEADER_BLOCK_TYPE
    }

    fn data(&self) -> super::Data {
        Data::Header(self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::HeaderBlock;

    const HEADER_OUTPUT_EXAMPLE: &str = r#"
    {
        "time": 1550476186479,
        "blocks": [
           {
              "id": "oUq2g_tl8y",
              "type": "header",
              "data": {
                 "text": "Editor.js",
                 "level": 2
              }
           }
        ]
    }
    "#;
}
