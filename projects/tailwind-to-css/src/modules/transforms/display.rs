use super::*;

impl Display for TailwindScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "scale-{}", self.scale),
            Some(true) => write!(f, "scale-x-{}", self.scale),
            Some(false) => write!(f, "scale-y-{}", self.scale),
        }
    }
}

impl TailwindInstance for TailwindScale {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let scale = self.scale as f32 / 100.0;
        let scale = match self.axis {
            None => format!("scale({})", scale),
            Some(true) => format!("scaleX({})", scale),
            Some(false) => format!("scaleY({})", scale),
        };
        css_attributes! {
            "transform" => scale
        }
    }
}

impl Display for TailwindRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rotate-{}", self.deg)
    }
}

impl TailwindInstance for TailwindRotate {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let rotate = format!("rotate({}deg)", self.deg);
        css_attributes! {
            "transform" => rotate
        }
    }
}

impl Display for TailwindTranslate {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTranslate {}

impl Display for TailwindSkew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "skew-x-{}", self.deg),
            false => write!(f, "skew-y-{}", self.deg),
        }
    }
}

impl TailwindInstance for TailwindSkew {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let skew = match self.axis {
            true => format!("skewX({}deg)", self.deg),
            false => format!("skewY({}deg)", self.deg),
        };
        css_attributes! {
            "transform" => skew
        }
    }
}

impl Display for TailwindOrigin {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOrigin {}
