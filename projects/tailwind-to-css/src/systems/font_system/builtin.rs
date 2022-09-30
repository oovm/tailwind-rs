use super::*;

impl FontSystem {
    /// Builtin fonts' config
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.insert_builtin_size();
        new.insert_builtin_family();
        new.insert_builtin_tracking();
        new
    }
    // https://tailwindcss.com/docs/font-size
    fn insert_builtin_size(&mut self) {
        self.insert_size("xs", FontSize::new(0.75, 1.0));
        self.insert_size("sm", FontSize::new(0.875, 1.25));
        self.insert_size("md", FontSize::new(1.0, 1.5));
        self.insert_size("lg", FontSize::new(1.125, 1.75));
        self.insert_size("xl", FontSize::new(1.25, 1.75));
        self.insert_size("2xl", FontSize::new(1.5, 2.0));
        self.insert_size("3xl", FontSize::new(1.875, 2.25));
        self.insert_size("4xl", FontSize::new(2.25, 2.5));
        self.insert_size("5xl", FontSize::new(3.0, -100.));
        self.insert_size("6xl", FontSize::new(3.75, -100.));
        self.insert_size("7xl", FontSize::new(4.5, -100.));
        self.insert_size("8xl", FontSize::new(6.0, -100.));
        self.insert_size("9xl", FontSize::new(8.0, -100.));
    }
    fn insert_builtin_family(&mut self) {
        self.insert_family("sans", r#"ui-sans-serif"#);
        self.insert_family("serif", r#"ui-serif"#);
        self.insert_family("mono", r#"ui-sans-monospace"#);
    }
    // https://tailwindcss.com/docs/letter-spacing
    fn insert_builtin_tracking(&mut self) {
        self.insert_tracking("tightest", -0.75);
        self.insert_tracking("tighter", -0.05);
        self.insert_tracking("tight", -0.025);
        self.insert_tracking("normal", 0.0);
        self.insert_tracking("wide", 0.025);
        self.insert_tracking("wider", 0.05);
        self.insert_tracking("widest", 0.1);
    }
}
