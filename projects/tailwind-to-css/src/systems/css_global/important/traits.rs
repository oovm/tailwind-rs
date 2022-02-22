use super::*;

impl Display for ImportantSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.important {
            true => write!(f, "{} !important;", self.set.iter().join(" ")),
            false => write!(f, "{};", self.set.iter().join(" ")),
        }
    }
}

impl Display for ImportantMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, (important, v)) in &self.map {
            match important {
                true => write!(f, "{}:{}!important;", k, v)?,
                false => write!(f, "{}:{};", k, v)?,
            }
        }
        Ok(())
    }
}

impl AddAssign<Self> for ImportantSet {
    fn add_assign(&mut self, rhs: Self) {
        self.important = rhs.important;
        self.set.extend(rhs.set.into_iter());
    }
}

impl AddAssign<Self> for ImportantMap {
    fn add_assign(&mut self, rhs: Self) {
        self.map.extend(rhs.map.into_iter());
    }
}
