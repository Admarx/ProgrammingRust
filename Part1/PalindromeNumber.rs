impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        let asc = str.as_bytes();

        let mut i = 0;
        let mut j = str.len();

        while i < j-1 {
            if asc[i] != asc[j-1]
            {
                return false;
            }
            else
            {
                i+=1;
                j-=1;
            }
        }
        return true;
    }
}