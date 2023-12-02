#[derive(Debug)]
pub struct Stack<T> {
    list: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { list: Vec::new() }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
    pub fn add(&mut self, element: T) {
        self.list.push(element)
    }
    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }
}
