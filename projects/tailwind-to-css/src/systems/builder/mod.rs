mod dynamic;
mod inlined;
mod methods;
mod setter;
use crate::{
    systems::{builder::inlined::Inlined, instruction::TailwindInstruction},
    *,
};
use std::{collections::BTreeSet, fmt::Debug};

#[derive(Debug)]
pub struct TailwindBuilder {
    // pub apply: BTreeMap<String, CssAttributes>,
    pub obfuscate: bool,
    ///
    pub objects: BTreeSet<Box<dyn TailwindInstance>>,
    ///
    pub preflight: PreflightSystem,
    ///
    pub palettes: PaletteSystem,
    ///
    pub screens: BreakPointSystem,
    ///
    pub fonts: FontSystem,
}

impl TailwindBuilder {
    /// ## Inline mode
    ///
    ///
    /// # Returns
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags.
    ///
    /// **`.bundle()` is required even for inline, because some directives cannot be inline.**
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
    #[cfg_attr(not(debug_assertions), track_caller)]
    pub fn trace(&mut self, style: &str) -> String {
        self.try_trace(style).unwrap()
    }
    /// Safe version of [`TailwindBuilder::trace`]
    pub fn try_trace(&mut self, style: &str) -> Result<String> {
        let parsed = parse_tailwind(style)?;
        let mut out = vec![];
        for i in parsed.into_iter() {
            let instance = i.get_instance()?;
            out.push(instance.id());
            self.objects.insert(instance);
        }
        Ok(out.join(" "))
    }

    /// ## Scope mode
    ///
    ///
    /// # Returns
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
    #[cfg_attr(not(debug_assertions), track_caller)]
    pub fn scope(&self, style: &str) {
        let _id = style;
    }
    /// Safe version of [`TailwindBuilder::scope`]
    pub fn try_scope() {}

    /// ## DataTW mode
    ///
    ///
    /// # Returns
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
    #[cfg_attr(not(debug_assertions), track_caller)]
    pub fn dataset() {}

    /// ## Inline mode(no bundle)
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
    #[cfg_attr(not(debug_assertions), track_caller)]
    pub fn inline(&mut self, style: &str) -> Inlined {
        self.try_inline(style).unwrap()
    }
    /// Safe version of [`TailwindBuilder::inline`]
    pub fn try_inline(&mut self, style: &str) -> Result<Inlined> {
        let parsed = parse_tailwind(style)?;
        let mut out = Inlined::default();
        for item in parsed {
            match item.inlineable() {
                true => out.add_style(item.to_string()),
                false => {
                    out.add_class(item.id());
                    self.objects.insert(item.get_instance()?);
                },
            }
        }
        Ok(out)
    }
    /// Bundle all used stylesheets
    #[inline]
    #[cfg_attr(not(debug_assertions), track_caller)]
    pub fn bundle(&self) -> String {
        self.try_bundle(1024 * 10).unwrap()
    }
    /// Safe version of [`TailwindBuilder::bundle`]
    pub fn try_bundle(&self, cap: usize) -> Result<String> {
        let mut out = String::with_capacity(cap);
        if !self.preflight.disable {
            self.preflight.write_css(&mut out, self)?;
        }
        for item in &self.objects {
            item.write_css(&mut out, self)?;
        }
        Ok(out)
    }
}

fn parse_tailwind(input: &str) -> Result<Vec<TailwindInstruction>> {
    let styles = tailwind_ast::parse_tailwind(input)?;
    Ok(styles.into_iter().map(TailwindInstruction::from).collect())
}
