mod trie;
mod word;

use word::*;
use trie::*;

use std::collections::BinaryHeap;

#[derive(Debug)]
enum Tile
{
    None,
    Normal(String),
    EitherOr(String, String),
    Prefix(String),
    Suffix(String),
}
impl Tile
{
    fn parse(data: &str) -> Tile
    {
        if data.starts_with("-")
        {
            Tile::Prefix(data.trim_left_matches("-").to_owned())
        }
        else if data.ends_with("-")
        {
            Tile::Suffix(data.trim_right_matches("-").to_owned())
        }
        else if data.contains("/")
        {
            let mut split = data.split("/");
            let a = split.next().unwrap();
            let b = split.next().unwrap();

            Tile::EitherOr(a.to_owned(), b.to_owned())
        }
        else
        {
            Tile::Normal(data.to_owned())
        }
    }
}
type Board = [[Tile; 4]; 4];

fn find_words_string(
    text: &str,
    suffix: bool,
    board: &Board,
    x: usize, y: usize,
    cur_node: &TrieNode,
    cur_word: &mut String,
    markers: &mut [[bool; 4]; 4],
    out: &mut BinaryHeap<Word>
)
{
    if let Some(node) = cur_node.next(&text)
    {
        cur_word.push_str(text);
        markers[y][x] = true;
        
        if node.terminal && cur_word.len() > 2
        {
            out.push(
                Word::new(cur_word.clone())
            );
        }

        if !suffix
        {
            if x != 0
            {
                find_words(board, x - 1, y, false, node, cur_word, markers, out);
            }
            if x != 3
            {
                find_words(board, x + 1, y, false, node, cur_word, markers, out);
            }
            if y != 0
            {
                find_words(board, x, y - 1, false, node, cur_word, markers, out);
            }
            if y != 3
            {
                find_words(board, x, y + 1, false, node, cur_word, markers, out);
            }
            
            if x != 0 && y != 0
            {
                find_words(board, x - 1, y - 1, false, node, cur_word, markers, out);
            }
            if x != 3 && y != 0
            {
                find_words(board, x + 1, y - 1, false, node, cur_word, markers, out);
            }
            if x != 0 && y != 3
            {
                find_words(board, x - 1, y + 1, false, node, cur_word, markers, out);
            }
            if x != 3 && y != 3
            {
                find_words(board, x + 1, y + 1, false, node, cur_word, markers, out);
            }
        }
        
        markers[y][x] = false;

        for _ in 0..text.len()
        {
            cur_word.pop();
        }
    }
}

fn find_words(
    board: &Board,
    x: usize, y: usize,
    first: bool,
    cur_node: &TrieNode,
    cur_word: &mut String,
    markers: &mut [[bool; 4]; 4],
    out: &mut BinaryHeap<Word>
)
{
    if x >= 4 || y >= 4 || markers[y][x]
    {
        return;
    }

    match board[y][x]
    {
        Tile::Normal(ref text) => {
            find_words_string(&text, false, board, x, y, cur_node, cur_word, markers, out);
        }
        Tile::EitherOr(ref a, ref b) => {
            find_words_string(&a, false, board, x, y, cur_node, cur_word, markers, out);
            find_words_string(&b, false, board, x, y, cur_node, cur_word, markers, out);
        }
        Tile::Prefix(ref text) => {
            if first
            {
                find_words_string(&text, false, board, x, y, cur_node, cur_word, markers, out);
            }
        }
        Tile::Suffix(ref text) => {
            find_words_string(&text, true, board, x, y, cur_node, cur_word, markers, out);
        }
        Tile::None => {}
    }
    
}

fn crack_wordament(board: &Board, sentinel: &TrieNode)
{
    let mut markers = [[false; 4]; 4];
    let mut word = String::new();
    let mut result = BinaryHeap::<Word>::new();
    
    for i in 0..4
    {
        for j in 0..4
        {
            find_words(board, i, j, true, sentinel, &mut word, &mut markers, &mut result);
            
            assert!(markers == [[false; 4]; 4]);
            assert!(word.len() == 0);
        }
    }

    while let Some(r) = result.pop()
    {
        println!("{}", r.text);
    }
}

fn main()
{
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::BufReader;
    
    let mut sentinel = TrieNode::new();

    let f = File::open("resources/wordlist").unwrap();
    let reader = BufReader::new(f);

    let mut count = 0;
    
    for line in reader.lines()
    {
        sentinel.insert(&line.unwrap().to_lowercase());
        count += 1;
    }
    
    println!("{:?} words loaded.", count);
    
    loop
    {
        println!("Enter a board: ");

        let mut board = [
            [Tile::None, Tile::None, Tile::None, Tile::None],
            [Tile::None, Tile::None, Tile::None, Tile::None],
            [Tile::None, Tile::None, Tile::None, Tile::None],
            [Tile::None, Tile::None, Tile::None, Tile::None]
            ];
        
        for i in 0..4
        {
            let mut buffer = String::new();
            
            std::io::stdin().read_line(&mut buffer).unwrap();

            let mut toks = buffer.split_whitespace();
            
            board[i][0] = Tile::parse(toks.next().unwrap());
            board[i][1] = Tile::parse(toks.next().unwrap());
            board[i][2] = Tile::parse(toks.next().unwrap());
            board[i][3] = Tile::parse(toks.next().unwrap());
        }
        
        crack_wordament(&board, &sentinel);
    }
}
