mod dynamic;
mod inlined;
mod methods;
mod scoped;
mod setter;
use crate::{
    systems::{builder::inlined::Inlined, instruction::TailwindInstruction},
    *,
};
use std::{collections::BTreeSet, fmt::Debug, hash::Hasher};

#[derive(Debug)]
pub struct TailwindBuilder {
    /// Whether to enable class name confusion
    ///
    /// ```html
    /// <img class="_b2JmdXNjYXRl"/>
    /// <img data-tw-b2JmdXNjYXRl/>
    /// <img data-tw="b2JmdXNjYXRl"/>
    /// ```
    pub obfuscate: bool,
    ///
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
            let i = CssInstance::from_trace(&i.get_instance()?, &self);
            out.push(i.get_class());
            self.objects.insert(i);
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
            let i = CssInstance::from_inline(&item.get_instance()?, self);
            out.add_class(i.get_class());
            self.objects.insert(i);
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
            out.push_str(&self.preflight.to_string());
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
