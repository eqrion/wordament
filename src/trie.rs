use std::collections::HashMap;
use std::str::Chars;

pub struct TrieNode
{
    pub terminal: bool,
    pub children: HashMap<char, Box<TrieNode>>
}

impl TrieNode
{
    pub fn new() -> TrieNode
    {
        TrieNode {
            terminal: false,
            children: HashMap::new()
        }
    }

    pub fn next(&self, data: &str) -> Option<&TrieNode>
    {
        self.next_chars(data.chars())
    }
    pub fn next_chars(&self, mut chars: Chars) -> Option<&TrieNode>
    {
        if let Some(token) = chars.next()
        {        
            if let Some(ref mut next) = self.next_char(token)
            {
                next.next_chars(chars)
            }
            else
            {
                None
            }
        }
        else
        {
            Some(self)
        }
    }
    pub fn next_char(&self, token: char) -> Option<&TrieNode>
    {
        if let Some(ref b) = self.children.get(&token)
        {
            Some(&b)
        }
        else
        {
            None
        }
    }
    
    pub fn contains(&self, data: &str) -> bool
    {
        self.contains_chars(data.chars())
    }
    pub fn contains_chars(&self, mut chars: Chars) -> bool
    {
        if let Some(token) = chars.next()
        {
            if let Some(ref b) = self.children.get(&token)
            {
                b.contains_chars(chars)
            }
            else
            {
                false
            }
        }
        else
        {
            self.terminal
        }
    }

    pub fn insert(&mut self, data: &str)
    {
        self.insert_chars(data.chars());
    }
    pub fn insert_chars(&mut self, mut chars: Chars)
    {                    
        while let Some(token) = chars.next()
        {
            if token < '\''
            {
                continue;
            }
            
            if let None = self.children.get(&token)
            {
                self.children.insert(
                    token,
                    Box::new(TrieNode::new())
                );
            }
            
            if let Some(ref mut next) = self.children.get_mut(&token)
            {
                next.insert_chars(chars);
            }
            
            return;
        }

        self.terminal = true;
    }
}
