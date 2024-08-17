#![allow(dead_code)]
struct BrowserHistory {
    pos: usize,
    data: Vec<String>,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            pos: 0,
            data: vec![homepage],
        }
    }

    fn visit(&mut self, url: String) {
        self.pos += 1;
        self.data.splice(self.pos.., std::iter::once(url));
    }

    fn back(&mut self, steps: i32) -> String {
        self.pos = self.pos.saturating_sub(steps as usize);
        self.data[self.pos].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.pos = (self.pos + steps as usize).min(self.data.len() - 1);
        self.data[self.pos].clone()
    }
}
