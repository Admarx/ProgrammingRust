impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let noTrails = s.trim();
        let words = noTrails.split(' ');
         
        return words.last().unwrap().len() as i32;
    }
}