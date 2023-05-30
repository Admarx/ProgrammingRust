impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut start: i32 = -1;
        let mut koniec: i32 = -1;

        for (x,i) in nums.iter().enumerate()
        {
            if start == -1 && i == &target
            {
                start = x as i32;
            }
            if i == &target
            {
                koniec = x as i32;
            }
        }

        let wynik =  vec![start, koniec];
        return wynik;
    }
}