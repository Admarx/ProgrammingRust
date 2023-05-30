impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let dwa = vec![String::from("a"), String::from("b"), String::from("c")];
        let trzy = vec![String::from("d"), String::from("e"), String::from("f")];
        let cztery = vec![String::from("g"), String::from("h"), String::from("i")];
        let piec = vec![String::from("j"), String::from("k"), String::from("l")];
        let szesc = vec![String::from("m"), String::from("n"), String::from("o")];
        let siedem = vec![String::from("p"), String::from("q"), String::from("r"), String::from("s")];
        let osiem = vec![String::from("t"), String::from("u"), String::from("v")];
        let dziewiec = vec![String::from("w"), String::from("x"), String::from("y"), String::from("z")];
        
        let mut strtmp: Vec<String> = Vec::new();
        let mut wynik: Vec<String> = Vec::new();
        let mut tmp: Vec<String> = Vec::new();

        for (i, c) in digits.chars().enumerate()
        {
            match c
            {
                '2' => strtmp = dwa.clone(),
                '3' => strtmp = trzy.clone(),
                '4' => strtmp = cztery.clone(),
                '5' => strtmp = piec.clone(),
                '6' => strtmp = szesc.clone(),
                '7' => strtmp = siedem.clone(),
                '8' => strtmp = osiem.clone(),
                '9' => strtmp = dziewiec.clone(),
                _ => strtmp = Vec::new(),
            }

            if wynik.iter().count() == 0
            {
                wynik = strtmp.clone();
            }
            else
            {
                tmp = wynik.clone();
                for x in tmp.iter()
                {
                    for j in strtmp.iter()
                    {
                        wynik.push(format!("{}{}",x,j));
                    }
                }
            }
        }
        wynik.retain(|x| x.len() == digits.len());   
        return wynik;
    }
}