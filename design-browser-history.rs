struct BrowserHistory {
    history: Vec<String>,
    current_index: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![homepage],
            current_index: 0,
        }
    }
    
    fn visit(&mut self, url: String) {
        self.current_index += 1;
        if self.current_index < self.history.len() {
            self.history.truncate(self.current_index);
        }
        self.history.push(url);
    }
    
    fn back(&mut self, steps: i32) -> String {
        let steps_back = steps as usize;
        if steps_back > self.current_index {
            self.current_index = 0;
        } else {
            self.current_index -= steps_back;
        }
        self.history[self.current_index].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let steps_forward = steps as usize;
        if self.current_index + steps_forward >= self.history.len() {
            self.current_index = self.history.len() - 1;
        } else {
            self.current_index += steps_forward;
        }
        self.history[self.current_index].clone()
    }
}
