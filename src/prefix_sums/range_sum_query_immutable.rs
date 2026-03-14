pub struct NumArray {
    prefix_sums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sums = Vec::new();
        let mut running_sum = 0;

        prefix_sums.push(running_sum);

        for num in nums {
            running_sum += num;
            prefix_sums.push(running_sum);
        }

        Self { prefix_sums }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sums[right as usize + 1] - self.prefix_sums[left as usize]
    }
}
