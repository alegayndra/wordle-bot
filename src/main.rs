use rand::Rng;
use std::collections::HashMap;

const LETTERS: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const WORDS: [&str; 12] = ["point", "proxy", "blaze", "kinky", "saber", "loose", "goofy", "abuse", "adult", "award", "basis", "beach"];

/*
  'c' = correct
  'u' = used
  'n' = no
  'w' = who knows
*/
fn filter_words(mut words: Vec<String>, letters: &HashMap<String, (char, Vec<i32>)>) -> Vec<String> {
  for l in LETTERS {
    match letters.get(&l.to_string()) {
      Some((c, v)) => {
        match c {
          'c' => {
            for i in v {
              words.retain(|word| word.chars().nth(*i as usize).unwrap().to_string() == l.to_string());
            }
            // println!("letter correct {} {:?}", l, words);
          },
          'n' => {
            words.retain(|word| !word.contains(l));
            // println!("letter incorrect {} {:?}", l, words);
          },
          'u' => {
            words.retain(|word| {word.contains(l)});
            // println!("letter other {} {:?}", l, words);
          },
          _ => {}
        };
      },
      None => println!("")
    };
  }
  words
}


fn filter_letters(word: &String, correct_word: &String, mut letters: HashMap<String, (char, Vec<i32>)>) -> HashMap<String, (char, Vec<i32>)> {
  for w in word.as_bytes() {
    let mut is_in_word = false;
    for c in correct_word.as_bytes() {
      if w == c {
        letters.remove(&(*w as char).to_string());
        letters.insert((*w as char).to_string().clone(), ('u', vec![]));
        is_in_word = true;
      }
    }
    if !is_in_word {
      letters.remove(&(*w as char).to_string());
      letters.insert((*w as char).to_string().clone(), ('n', vec![]));
    }
  }

  for i in 0..5 {
    let l = word.as_bytes()[i];
    if l == correct_word.as_bytes()[i] {
      if letters.contains_key(&(l as char).to_string()) {
        let mut vec_pos: Vec<i32> = match letters.get(&(l as char).to_string()) {
          Some((_, v)) => v.clone(),
          None => vec![]
        };
        vec_pos.push(i as i32);
        letters.remove(&(l as char).to_string());
        letters.insert((l as char).to_string().clone(), ('c', vec_pos));
      }
    }
  }

  letters
}

fn wordle() {
  // let mut letters: Vec<(String, char)> = vec![];
  let mut letters: HashMap<String, (char, Vec<i32>)> = HashMap::new();
  for letter in LETTERS {
    letters.insert(letter.to_string(), ('w', vec![]));
  }

  let mut words: Vec<String> = vec![];
  for word in WORDS {
    words.push(word.to_string());
  }

  // println!("words {:?}", words);

  let mut rng = rand::thread_rng();
  let mut pos: usize;

  let correct_word = words[5].clone(); // loose
  // println!("correct_word {:?}\n", correct_word);

  let mut word: String;

  println!("start");
  let mut tries: i32 = 0;
  loop {
    // println!("loop");
    tries += 1;
    pos = rng.gen_range(0..words.len());
    word = words[pos].clone();

    println!("random word generated {:?}\n", word);

    if word == correct_word {
      break;
    }

    // println!("not correct word");

    letters = filter_letters(&word, &correct_word, letters);
    
    // println!("letters filtered{:?}", letters);

    words = filter_words(words,&letters);

    // println!("words filtered {:?}", words);
  }

  println!("tries: {:?}", tries);
}

fn main() {
  wordle();
  // println!("Hello, world!");
}
