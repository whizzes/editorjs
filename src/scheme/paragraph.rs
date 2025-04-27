use serde::{Deserialize, Serialize};

pub const PARAGRAPH_BLOCK_TYPE: &str = "paragraph";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ParagraphData {
    text: String,
}

impl ParagraphData {
    pub fn new(text: String) -> Self {
        ParagraphData { text }
    }
}

#[cfg(test)]
mod tests {
    use crate::scheme::{Block, BlockData, Scheme};

    use super::ParagraphData;

    const PARAGRAPH_OUTPUT_EXAMPLE: &str = r#"
    {
        "time": 1550476186479,
        "version": "2.12.4",
        "blocks": [
          {
            "id": "zbGZFPM-iI",
            "type": "paragraph",
            "data": {
               "text": "Hey. Meet the new Editor. On this page you can see it in action — try to edit this text. Source code of the page contains the example of connection and configuration."
            }
          }
        ]
    }
    "#;

    #[test]
    fn builds_paragraph_from_output() {
        let block: Scheme = serde_json::from_str(PARAGRAPH_OUTPUT_EXAMPLE).unwrap();
        let paragraph = block.blocks[0].data.clone();

        match paragraph {
            BlockData::Paragraph(paragraph) => {
                assert_eq!(
                    paragraph.text,
                    "Hey. Meet the new Editor. On this page you can see it in action — try to edit this text. Source code of the page contains the example of connection and configuration."
                );
            }
            _ => panic!("Expected paragraph block"),
        }
    }

    #[test]
    fn searializes_paragraph_from_scheme() {
        let mut scheme = Scheme::new();
        let paragraph = ParagraphData::new("Hey. Meet the new Editor. On this page you can see it in action — try to edit this text. Source code of the page contains the example of connection and configuration.".to_string());
        let time = scheme.time;
        let block = Block::new(BlockData::Paragraph(paragraph));
        let expect = format!(
            r#"{{"time":{time},"version":"2.12.4","blocks":[{{"id":"{block_id}","type":"paragraph","data":{{"text":"Hey. Meet the new Editor. On this page you can see it in action — try to edit this text. Source code of the page contains the example of connection and configuration."}}}}]}}"#,
            block_id = block.id,
            time = time
        );

        scheme.add_block(block);

        assert_eq!(scheme.to_json(), expect);
    }
}
