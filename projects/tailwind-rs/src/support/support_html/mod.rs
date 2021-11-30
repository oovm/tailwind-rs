use crate::config::HtmlConfig;
use tl::{parse, Node, ParserOptions};
#[cfg(test)]
mod test;

use crate::Result;

impl HtmlConfig {
    pub fn find_all_class(input: &str) -> Result<Vec<&str>> {
        let mut buffer = vec![];
        let dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes() {
            // ignore if any problem
            push_class(node, &mut buffer);
        }
        Ok(buffer)
    }
}

fn push_class<'a>(node: &'a Node, buffer: &mut Vec<&'a str>) -> Option<()> {
    let attributes = node.as_tag()?.attributes();
    let class = attributes.class()?.try_as_utf8_str()?;
    buffer.push(class);
    Some(())
}
