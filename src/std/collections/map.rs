#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Map<I, F> {
    pub iter: I,
    f: F,
}

impl<I, F> Map<I, F> {
    pub fn new(iter: I, f: F) -> Map<I, F> {
        Map { iter, f }
    }

    pub fn into_inner(self) -> I {
        self.iter
    }
}

impl<I, F, A, B> Iterator for Map<I, F>
where
    I: Iterator<Item = A>,
    F: FnMut(A) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}
