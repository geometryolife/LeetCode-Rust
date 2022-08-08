struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &item in letters.iter() {
            if item > target {
                return item;
            }
        }

        return letters[0];
    }
}
