use log::error;
use tailwind_css::TailwindBuilder;
use tl::{parse, Node, ParserOptions};

use crate::{config::HtmlConfig, Result};

#[cfg(test)]
mod test;

impl HtmlConfig {
    pub fn trace_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            trace_class(node, tw);
        }
        Ok(dom.inner_html())
    }
}

fn trace_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    match tw.try_trace(class.try_as_utf8_str()?) {
        Ok(o) => {
            class.set(o).ok()?;
        },
        Err(e) => error!("{}", e),
    }
    Some(())
}
