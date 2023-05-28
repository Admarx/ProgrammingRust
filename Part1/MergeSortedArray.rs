impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut sum = m+n-1;
        let mut i = m-1;
        let mut j = n-1;

        while sum >= 0
        {
            if i>=0 && j>=0
            {
                if nums1[i as usize] > nums2[j as usize]
                {
                    nums1[sum as usize] = nums1[i as usize];
                    sum-=1;
                    i-=1;
                }
                else
                {
                    nums1[sum as usize] = nums2[j as usize];
                    sum-=1;
                    j-=1;  
                }
            }
            else if i>=0
            {
                nums1[sum as usize] = nums1[i as usize];
                sum-=1;
                i-=1;
            } 
            else 
            {
                nums1[sum as usize] = nums2[j as usize];
                sum-=1;
                j-=1;
            }
        }
    }
}