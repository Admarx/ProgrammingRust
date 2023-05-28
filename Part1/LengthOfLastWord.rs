impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let noTrails = s.trim();
        let words = noTrails.split(' ');
        let mut lastWord = "";
        for word in words {
            lastWord = word;
        }
        return lastWord.len() as i32;
    }
}