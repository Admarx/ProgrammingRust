impl Solution {
    pub fn is_ugly(n: i32) -> bool {
         let mut m = n;
         
         if m > 1
            {
                while m > 1
                {
                    if m % 5 == 0
                    {
                        m = m / 5;
                        continue;
                    }
                    else if m % 3 == 0
                    {
                        m = m / 3;
                        continue;
                    }
                    else if m % 2 == 0
                    {
                        m = m / 2;
                        continue;
                    }
                    else
                    {
                        return false;
                    }
                }
            }
            else if m <= 0
            {
                return false;
            }
            return true;
    }
}