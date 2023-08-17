pub struct Operator<'a, T>(Box<dyn Fn(T, T) -> T + 'a>);

impl<'a, T> Operator<'a, T> {
    pub fn new(f: impl Fn(T, T) -> T + 'a) -> Self {
        Self(Box::new(f))
    }

    pub fn evaluate(&self, left: T, right: T) -> T {
        self.0(left, right)
    }
}
