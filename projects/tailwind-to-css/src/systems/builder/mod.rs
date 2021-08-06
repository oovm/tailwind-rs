mod setter;
use crate::{systems::instruction::TailwindInstruction, *};
use std::{collections::BTreeSet, fmt::Debug};
use tailwind_ast::parse_tailwind;

#[derive(Debug)]
pub struct TailwindBuilder {
    // pub apply: BTreeMap<String, CssAttributes>,
    pub objects: BTreeSet<Box<dyn TailwindInstance>>,
    pub preflight: PreflightSystem,
    pub screens: BreakPointSystem,
    pub colors: PaletteSystem,
    pub fonts: FontSystem,
}

impl TailwindBuilder {
    #[inline]
    pub fn clear(&mut self) {
        self.objects.clear()
    }
    #[inline]
    #[track_caller]
    pub fn trace(&mut self, style: &str) -> String {
        self.try_trace(style).unwrap()
    }
    pub fn try_trace(&mut self, style: &str) -> Result<String> {
        let parsed = Self::parse_tailwind(style)?;
        let out: Vec<String> = parsed.iter().map(|s| s.id()).collect();
        for i in parsed.into_iter() {
            self.objects.insert(i.get_instance()?);
        }
        Ok(out.join(" "))
    }

    pub fn inline(&self, style: &str) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        let parsed = match Self::parse_tailwind(style) {
            Ok(o) => o,
            Err(_) => return out,
        };
        for item in parsed {
            out.extend(item.attributes(self))
        }
        out
    }
    pub fn bundle(&self) -> String {
        self.bundle_objects(1024 * 10).unwrap_or_default()
    }

    fn bundle_objects(&self, cap: usize) -> Result<String> {
        let mut out = String::with_capacity(cap);
        self.preflight.write_css(&mut out, self)?;
        for item in &self.objects {
            item.write_css(&mut out, self)?;
        }
        Ok(out)
    }
    pub fn scope() {}
    pub fn scope_data() {}

    pub fn parse_tailwind(input: &str) -> Result<Vec<TailwindInstruction>> {
        let styles = parse_tailwind(input)?;
        Ok(styles.into_iter().map(TailwindInstruction::from).collect())
    }
}
