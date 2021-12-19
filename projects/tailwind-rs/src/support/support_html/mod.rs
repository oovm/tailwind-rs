use log::error;
use tl::{parse, Bytes, Node, ParserOptions};

use tailwind_css::TailwindBuilder;

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
    pub fn inline_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            inline_class(node, tw);
        }
        Ok(dom.inner_html())
    }
    pub fn scope_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            scope_class(node, tw);
        }
        Ok(dom.inner_html())
    }
}

fn trace_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    match tw.trace(class.try_as_utf8_str()?) {
        Ok(c) => {
            class.set(c).ok()?;
        },
        Err(e) => error!("{}", e),
    }
    Some(())
}

fn inline_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    let mut style = Bytes::new();
    match tw.inline(class.try_as_utf8_str()?) {
        Ok((c, s)) => {
            class.set(c).ok()?;
            style.set(s).ok()?;
        },
        Err(e) => {
            error!("{}", e);
            return Some(());
        },
    };
    attributes.insert("style", Some(style));
    Some(())
}

fn scope_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    match tw.scope(class.try_as_utf8_str()?) {
        Ok(c) => {
            class.set(c).ok()?;
        },
        Err(e) => {
            error!("{}", e);
        },
    };
    Some(())
}
