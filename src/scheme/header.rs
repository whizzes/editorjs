use serde::{Deserialize, Serialize};

pub const HEADER_BLOCK_TYPE: &str = "header";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeaderData {
    text: String,
    level: u8,
}

impl HeaderData {
    pub fn new(text: String, level: u8) -> Self {
        HeaderData { text, level }
    }
}

#[cfg(test)]
mod tests {
    use crate::scheme::{Block, BlockData, Scheme};

    use super::HeaderData;

    const HEADER_OUTPUT_EXAMPLE: &str = r#"
    {
        "time": 1550476186479,
        "version": "2.12.4",
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

    #[test]
    fn builds_header_from_output() {
        let block: Scheme = serde_json::from_str(HEADER_OUTPUT_EXAMPLE).unwrap();
        let header = block.blocks[0].data.clone();

        match header {
            BlockData::Header(header) => {
                assert_eq!(header.text, "Editor.js");
                assert_eq!(header.level, 2);
            }
            _ => panic!("Expected header block"),
        }
    }

    #[test]
    fn searializes_header_from_scheme() {
        let mut scheme = Scheme::new();
        let header = HeaderData::new("Editor.js".to_string(), 2);
        let time = scheme.time;
        let block = Block::new(BlockData::Header(header));
        let expect = format!(
            r#"{{"time":{time},"version":"2.12.4","blocks":[{{"id":"{block_id}","type":"header","data":{{"text":"Editor.js","level":2}}}}]}}"#,
            block_id = block.id,
            time = time
        );

        scheme.add_block(block);

        assert_eq!(scheme.to_json(), expect);
    }
}
