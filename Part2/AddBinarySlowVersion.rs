impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::from("");
        let mut vecresult: Vec<String> = Vec::new();
        if a=="0" || a==""
        {
            return b;
        }
        if b=="0" || b==""
        {
            return a;
        }
        let bytesA = a.as_bytes();
        let bytesB = b.as_bytes();

        if a.len() > b.len()
        {
           let mut zapas = 0;
           let mut wynik = 0;
           let mut i = 0;
           while i < b.len()
           {
               wynik = bytesA[bytesA.len()-i-1]%48 + bytesB[bytesB.len()-i-1]%48 + zapas;
               if wynik==0
               {
                   result = format!("{}{}", "0".to_string(), result.to_string());
                   zapas = 0;
               }
               else if wynik==1
               {
                   result = format!("{}{}", "1".to_string(), result.to_string());
                   zapas = 0;
               }               
               else if wynik==2
               {
                   result = format!("{}{}", "0".to_string(), result.to_string());
                   zapas = 1;
               }
               else if wynik==3
               {
                   result = format!("{}{}", "1".to_string(), result.to_string());
                   zapas = 1;
               }  
               i+=1;
           }
           let mut j = bytesB.len();
           while j < a.len()
           {
               wynik = bytesA[bytesA.len()-j-1]%48 + zapas;
               if wynik==0
               {
                   result = format!("{}{}", "0".to_string(), result.to_string());
                   zapas = 0;
               }
               else if wynik==1
               {
                   result = format!("{}{}", "1".to_string(), result.to_string());
                   zapas = 0;
               }               
               else if wynik==2
               {
                   result = format!("{}{}", "0".to_string(), result.to_string());
                   zapas = 1;
               }

               j+=1;
           }
            if zapas==1
            {
                   result = format!("{}{}", "1".to_string(), result);
            }

            return result;
        }
        else if a.len() == b.len()
        {
           let mut zapas = 0;
           let mut wynik = 0;
           let mut i = 0;
           while i < b.len()
           {
              wynik = bytesA[bytesA.len()-i-1]%48 + bytesB[bytesB.len()-i-1]%48 + zapas;
              if wynik==0
              {
                   result = format!("{}{}", "0".to_string(), result.to_string());
                  zapas = 0;
              }
              else if wynik==1
              {
                   result = format!("{}{}", "1".to_string(), result.to_string());
                  zapas = 0;
              }               
              else if wynik==2
              {
                   result = format!("{}{}", "0".to_string(), result.to_string());
                  zapas = 1;
              }
              else if wynik==3
              {
                   result = format!("{}{}", "1".to_string(), result.to_string());
                  zapas = 1;
              }  
              i+=1;
            }
            if zapas==1
            {
                   result = format!("{}{}", "1".to_string(), result);
            }
            return result.to_string();
        }
        else
        {
           let wynik = Solution::add_binary(b,a);
           return wynik;
        }
    }
}