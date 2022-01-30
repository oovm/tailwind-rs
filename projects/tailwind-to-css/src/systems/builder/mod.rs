use std::{collections::BTreeSet, fmt::Debug};

use crate::{systems::instruction::TailwindInstruction, *};

pub use self::base62::{Base62, BASE62};

mod base62;
mod methods;
mod setter;

///
#[derive(Debug)]
pub struct TailwindBuilder {
    ///
    pub preflight: PreflightSystem,
    /// All dynamic color properties
    ///
    /// Only determined when packing
    pub palettes: PaletteSystem,
    /// All dynamic break points
    ///
    /// Only determined when packing
    pub screens: BreakPointSystem,
    /// All dynamically registered font properties
    ///
    /// Only determined when packing
    pub fonts: FontSystem,
    /// All dynamically registered effect properties
    ///
    /// Only determined when packing
    pub effects: EffectSystem,
    pub(crate) objects: BTreeSet<CssInstance>,
    pub(crate) bundles: BTreeSet<CssBundle>,
}

impl TailwindBuilder {
    /// ## Trace mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn trace(&mut self, style: &str, obfuscate: bool) -> Result<String> {
        let out = try_trace(self, style, obfuscate)?;
        Ok(out.as_traced())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn inline(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::Inline)?;
        Ok(out.as_inlined())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn scope(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::Scoped)?;
        Ok(out.as_scope())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn data_key(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::DataKey)?;
        Ok(out.as_dataset())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn data_value(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::DataValue)?;
        Ok(out.as_dataset())
    }
    /// Bundle all used stylesheets
    pub fn bundle(&self) -> Result<String> {
        let mut out = String::with_capacity(1024 * 10);
        if !self.preflight.disable {
            out.push_str(&self.preflight.to_string());
        }
        for item in &self.objects {
            item.write_css(&mut out)?;
        }
        for item in &self.bundles {
            item.write_css(&mut out)?;
        }
        Ok(out)
    }
}

fn parse_tailwind(input: &str) -> Result<Vec<TailwindInstruction>> {
    let styles = tailwind_ast::parse_tailwind(input)?;
    Ok(styles.into_iter().map(TailwindInstruction::from).collect())
}

fn try_trace(tw: &mut TailwindBuilder, style: &str, obfuscate: bool) -> Result<CssBundle> {
    let parsed = parse_tailwind(style)?;
    let mut out = CssBundle::default();
    for item in parsed {
        let i = CssInstance::new(&*item.get_instance()?, tw, obfuscate);
        out.add_trace(&i);
        tw.objects.insert(i);
    }
    Ok(out)
}

fn try_inline(tw: &mut TailwindBuilder, style: &str, mode: CssInlineMode) -> Result<CssBundle> {
    let parsed = parse_tailwind(style)?;
    let mut out = CssBundle::default();
    for item in parsed {
        let i = CssInstance::new(&*item.get_instance()?, tw, true);
        match &i.inlineable {
            true => out.add_inline(i),
            false => {
                out.add_trace(&i);
                tw.objects.insert(i);
            },
        };
    }
    out.set_mode(mode);
    tw.bundles.insert(out.to_owned());
    Ok(out)
}
