use core::panic::PanicInfo;

pub struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
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
        let val = self.data.pop();

        if !self.data.is_empty() {
            self.shift_down(0);
        }

        val
    }

    pub fn peek(&self) -> Option<&i32> {
        self.data.first()
    }

    fn shift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[parent] < self.data[i] {
                self.data.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn shift_down(&mut self, mut i: usize) {
        let n = self.data.len();

        loop {
            let mut largest = i;
            let (l, r) = (i * 2 + 1, i * 2 + 2);
            if l < n && self.data[l] > self.data[largest] {
                largest = l;
            }
            if r < n && self.data[r] > self.data[largest] {
                largest = r;
            }
            if i == largest {
                break;
            }
            self.data.swap(i, largest);
            i = largest;
        }
    }
}
