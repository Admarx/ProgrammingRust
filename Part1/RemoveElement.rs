impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
                
        nums.retain(|&x| x != val);
        let mut howMany = nums.iter().count();
        return howMany as i32;

    }
}
