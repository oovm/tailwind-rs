use super::*;

impl Debug for Box<dyn TailwindInstance> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id())
    }
}

impl Hash for Box<dyn TailwindInstance> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl PartialEq for Box<dyn TailwindInstance> {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
    }
}

impl Eq for Box<dyn TailwindInstance> {}

impl PartialOrd for Box<dyn TailwindInstance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for Box<dyn TailwindInstance> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}
