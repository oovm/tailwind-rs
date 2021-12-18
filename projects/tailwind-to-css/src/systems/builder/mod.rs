use std::{collections::BTreeSet, fmt::Debug};

use crate::{systems::instruction::TailwindInstruction, *};

mod methods;
mod setter;

///
#[derive(Debug)]
pub struct TailwindBuilder {
    ///
    pub obfuscate: bool,
    /// The set of attributes that are allowed to be used in the `tailwind`
    pub objects: BTreeSet<CssInstance>,
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
}

impl TailwindBuilder {
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
    #[track_caller]
    pub fn inline(&mut self, style: &str) -> Result<CssBundle> {
        let parsed = parse_tailwind(style)?;
        let mut out = CssBundle::default();
        for item in parsed {
            let i = CssInstance::new(&*item.get_instance()?, self);
            match i.inlinable {
                true => out.insert(i.clone()),
                false => self.objects.insert(i),
            };
        }
        out.set_inline(true);
        Ok(out)
    }
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
    #[track_caller]
    pub fn trace(&mut self, style: &str) -> Result<CssBundle> {
        let parsed = parse_tailwind(style)?;
        let mut out = CssBundle::default();
        for item in parsed {
            let i = CssInstance::new(&*item.get_instance()?, self);
            out.insert(i.clone());
            self.objects.insert(i);
        }
        Ok(out)
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
        Ok(out)
    }
}

fn parse_tailwind(input: &str) -> Result<Vec<TailwindInstruction>> {
    let styles = tailwind_ast::parse_tailwind(input)?;
    Ok(styles.into_iter().map(TailwindInstruction::from).collect())
}
