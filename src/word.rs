pub struct Word
{
    pub text: String,
}
impl Word
{
    pub fn new(text: String) -> Word
    {
        Word { text: text }
    }
    pub fn score(&self) -> usize
    {
        let mut score: usize = 0;

        for token in self.text.chars()
        {
            score += match token
            {
                'a' | 'e' | 's' => 1,
                'i' | 'o' | 'u' => 2,
                'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'l' | 'm' | 'n' | 't' => 3,
                'j' | 'k' | 'r' => 4,
                'p' | 'q' | 'v' | 'w' => 5,
                'x' | 'y' | 'z' => 6,
                _ => 1,
            }
        }

        score
    }
}

use std::cmp::*;

impl PartialEq for Word
{
    fn eq(&self, other: &Self) -> bool
    {
        self.text.eq(&other.text)
    }
}
impl Eq for Word
{}
impl PartialOrd for Word
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}
impl Ord for Word
{
    fn cmp(&self , other: &Self) -> Ordering
    {
        let order = other.score().cmp(&self.score());

        if let Ordering::Equal = order
        {
            self.text.cmp(&other.text)
        }
        else
        {
            order
        }
    }
}
