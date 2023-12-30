#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Pointer<T>(Vec<T>, usize);

impl<T> Pointer<T> {
    pub fn new(source: Vec<T>) -> Self {
        Self(source, 0)
    }

    pub fn from_iter(source: impl Iterator<Item = T>) -> Self {
        Self::new(source.collect())
    }

    fn as_ref(&self) -> &[T] {
        &self.0[self.1..]
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_ref().iter()
    }

    pub fn first(&self) -> Option<&T> {
        self.as_ref().first()
    }
    pub fn get(&self, i: usize) -> Option<&T> {
        self.as_ref().get(i)
    }

    pub fn move_cursor(self, index: usize) -> Self {
        let size = self.0.len();
        Self(self.0, self.1.saturating_add(index).min(size))
    }
}
