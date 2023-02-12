pub trait IMountSequence {

    fn Init(&mut self) -> i32;
    fn Dispose(&self);
    fn Shutdown(&self);
    fn Flush(&self);

}