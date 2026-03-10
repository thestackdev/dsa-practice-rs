pub struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        self.shift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let n = self.data.len();

        self.data.swap(0, n - 1);
        let min = self.data.pop();

        if !self.data.is_empty() {
            self.shift_down(0);
        }

        min
    }

    pub fn peek(&self) -> Option<&i32> {
        self.data.first()
    }

    fn shift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn shift_down(&mut self, mut i: usize) {
        let n = self.data.len();

        loop {
            let mut smallest = i;
            let (l, r) = (2 * i + 1, 2 * i + 2);
            if l < n && self.data[l] < self.data[smallest] {
                smallest = l;
            }
            if r < n && self.data[r] < self.data[smallest] {
                smallest = r;
            }
            if i == smallest {
                break;
            }

            self.data.swap(smallest, i);
            i = smallest;
        }
    }
}
