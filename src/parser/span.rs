pub trait Codespan {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn set_start(&mut self, start: usize);
    fn set_end(&mut self, end: usize);
}