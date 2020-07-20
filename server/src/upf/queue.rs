#[derive(Debug)]
pub struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    pub fn new() -> Self {
        Queue{ qdata: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.qdata.push(item);
    }

    pub fn pop(&mut self) ->Option<T> {
        let l = self.qdata.len();

        if l > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }
}
