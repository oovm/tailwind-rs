mod methods;
mod setter;
use crate::{systems::instruction::TailwindInstruction, *};
use std::{collections::BTreeSet, fmt::Debug};

#[derive(Debug)]
pub struct TailwindBuilder {
    // pub apply: BTreeMap<String, CssAttributes>,
    pub obfuscate: bool,
    pub objects: BTreeSet<Box<dyn TailwindInstance>>,
    pub preflight: PreflightSystem,
    pub screens: BreakPointSystem,
    pub colors: PaletteSystem,
    pub fonts: FontSystem,
}

impl TailwindBuilder {
    /// ## Inline mode(no bundle)
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
    #[track_caller]
    pub fn trace(&mut self, style: &str) -> String {
        self.try_trace(style).unwrap()
    }
    /// Safe version of [`TailwindBuilder::trace`]
    pub fn try_trace(&mut self, style: &str) -> Result<String> {
        let parsed = parse_tailwind(style)?;
        let out: Vec<String> = parsed.iter().map(|s| s.id()).collect();
        for i in parsed.into_iter() {
            self.objects.insert(i.get_instance()?);
        }
        Ok(out.join(" "))
    }

    /// ## Inline mode(no bundle)
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
    #[track_caller]
    pub fn scope(&self, style: &str) {
        let _id = style;
    }
    /// Safe version of [`TailwindBuilder::scope`]
    pub fn try_scope() {}

    /// ## Inline mode(no bundle)
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
    #[track_caller]
    pub fn dataset() {}

    /// ## Inline mode(no bundle)
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
    #[track_caller]
    pub fn inline(&self, style: &str) -> String {
        self.try_inline(style).unwrap()
    }
    /// Safe version of [`TailwindBuilder::inline`]
    pub fn try_inline(&self, style: &str) -> Result<String> {
        let parsed = parse_tailwind(style)?;
        let mut set = BTreeSet::new();
        for item in parsed {
            set.extend(item.attributes(self))
        }
        let vec: Vec<_> = set.into_iter().map(|s| s.to_string()).collect();
        Ok(vec.join(""))
    }
    /// Bundle all used stylesheets
    #[inline]
    #[track_caller]
    pub fn bundle(&self) -> String {
        self.try_bundle(1024 * 10).unwrap()
    }
    /// Safe version of [`TailwindBuilder::bundle`]
    pub fn try_bundle(&self, cap: usize) -> Result<String> {
        let mut out = String::with_capacity(cap);
        self.preflight.write_css(&mut out, self)?;
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
