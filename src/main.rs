use rand::Rng;
use std::collections::HashMap;
use std::fs;
use priority_queue::PriorityQueue;

pub const LETTERS: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

/*
  'c' = correct
  'u' = used
  'n' = no
  'w' = who knows
*/
fn filter_words(mut words: PriorityQueue<String, i32>, word: String, correct_word: &String) -> PriorityQueue<String, i32> {
  let mut arr: Vec<i32> = vec![0, 0, 0, 0, 0];

  for (index, character) in word.chars().enumerate() {
    println!("{}, {}", index, character);
    if correct_word.contains(character) {
      println!("word contains {}, {}", correct_word.chars().nth(index).unwrap(), character);
      if correct_word.chars().nth(index).unwrap() == character {
        arr[index] = 2;
      } else {
        arr[index] = 1;
      }
    }
  }

  println!("{}, {}, {:?}", correct_word, word, arr);

  for (w, score) in words.iter_mut() {
    *score = 0;

    for i in 0..4 {
      if w.chars().nth(i).unwrap() == w.chars().nth(i).unwrap() {
        *score += arr[i];
      }
    }
    
    // for (index, character) in word.chars().enumerate() {
    //   // do something with character `c` and index `i`
    //   if w.contains(character) {
    //     *score += 1;

    //   }
    //   // for (w_i, w_c) in w.chars().enumerate() {
    //   //   if word_c == w_c {
    //   //     *score += 1;
    //   //     if word_i == w_i {
    //   //       *score += 1;
    //   //     }
    //   //   }
    //   // }
    // }
  }
  
  words
}

fn read_file() -> Vec<String> {
  let arch = "src/words.txt";
  let mut words: Vec<String> = vec![];

  let contents = fs::read_to_string(arch).expect("Something went wrong reading the file");

  for line in contents.lines() {
    words.push(line.to_string());
  }

  words
}

fn wordle() {
  let mut pq = PriorityQueue::new();

  for word in read_file() {
    pq.push(word.clone(), 0);
  }

  let correct_word: String = "crazy".to_string(); // loose
  // println!("correct_word {:?}\n", correct_word);

  let mut word: String;

  println!("start");
  let mut tries: i32 = 0;
  loop {
    
    // println!("loop");
    tries += 1;
    word = match pq.peek() {
      Some(value) => {
        println!("{:?}", value.clone());
        value.0.clone()
      },
      None => {
        println!("queue empty");
        return
      }
    };

    // println!("random word generated {:?} {}\n", word, pos);

    if word == correct_word || tries >= 6 {
      break;
    }
    
    pq.pop();
    
    // println!("letters filtered{:?}", letters);

    pq = filter_words(pq, word, &correct_word);

    // println!("{:?}", pq);

    println!("{}", pq.is_empty());

    // println!("words filtered {:?}", words);
  }

  println!("tries: {:?}", tries);
}

fn main() {
  wordle();
  // println!("Hello, world!");
}
