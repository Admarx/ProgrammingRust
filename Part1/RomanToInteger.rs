impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut amountInt = 0;
        let asc = s.as_bytes();
        let length = s.len();
        let mut i = 0;

        while i < length
        {
            if asc[i] == 'M' as u8
            {
                i+=1;
                amountInt+=1000;
            }
             else if asc[i]=='D' as u8
            {
                i+=1;
                amountInt+=500;
            }
            else if asc[i]=='C' as u8
            {
                if i+1!=length
                {
                    if asc[i+1]=='M' as u8
                    {
                        i+=1;
                        amountInt+=900;
                    }
                    else if asc[i+1] == 'D' as u8
                    {
                        i+=1;
                        amountInt+=400;
                    }
                    else
                    {
                        amountInt+=100;  
                    }
                }
                else
                {
                    amountInt+=100; 
                }
                i+=1;
            }
            else if asc[i]=='L' as u8
            {
                i+=1;
                amountInt+=50;
            }
            else if asc[i]=='X' as u8
            {
                if i+1!=length
                {
                    if asc[i+1]=='C' as u8
                    {
                        i+=1;
                        amountInt+=90;
                    }
                    else if asc[i+1] == 'L' as u8
                    {
                        i+=1;
                        amountInt+=40;
                    }
                    else
                    {
                        amountInt+=10;  
                    }
                }
                else
                {
                    amountInt+=10; 
                }
                i+=1;
            }
            else if asc[i]=='V' as u8
            {
                i+=1;
                amountInt+=5;
            }
            else if asc[i]=='I' as u8
            {
                if i+1!=length
                {
                    if asc[i+1]=='X' as u8
                    {
                        i+=1;
                        amountInt+=9;
                    }
                    else if asc[i+1] == 'V' as u8
                    {
                        i+=1;
                        amountInt+=4;
                    }
                    else
                    {
                        amountInt+=1;  
                    }
                }
                else
                {
                    amountInt+=1; 
                }
                i+=1;
            }          
        }       
        return amountInt;
    }
}