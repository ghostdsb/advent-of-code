use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FIVEOFKIND = 7,
    FOUROFKIND = 6,
    FULLHOUSE = 5,
    THREEOFKIND = 4,
    TWOPAIR = 3,
    ONEPAIR = 2,
    HIGHCARD = 1,
}

#[derive(Debug, Clone, PartialOrd, Eq)]
struct Hand(String, u32);

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type = determine_hand_type(&self);
        let other_hand_type = determine_hand_type(&other);
        if self_hand_type as u8 > other_hand_type as u8 {
            return Ordering::Greater;
        } else if (self_hand_type as u8) < (other_hand_type as u8) {
            return Ordering::Less;
        } else {
            for (char1, char2) in self.0.chars().zip(other.0.chars()) {
                if char1 == char2 {
                    continue;
                } else {
                    match (char1, char2) {
                        ('A', _) => return Ordering::Greater,
                        (_, 'A') => return Ordering::Less,
                        ('K', _) => return Ordering::Greater,
                        (_, 'K') => return Ordering::Less,
                        ('Q', _) => return Ordering::Greater,
                        (_, 'Q') => return Ordering::Less,
                        ('J', _) => return Ordering::Greater,
                        (_, 'J') => return Ordering::Less,
                        ('T', _) => return Ordering::Greater,
                        (_, 'T') => return Ordering::Less,
                        (a, b) => {
                            let x = a.to_digit(10).unwrap();
                            let y = b.to_digit(10).unwrap();
                            if x > y {
                                return Ordering::Greater;
                            } else {
                                return Ordering::Less;
                            }
                        }
                    }
                }
            }
            return Ordering::Equal;
        }
    }
}

fn determine_hand_type(hand: &Hand) -> HandType {
    let sorted_hand = &mut (hand.0).chars().collect::<Vec<char>>();
    sorted_hand.sort();
    // dbg!(sorted_hand);
    let chunked_in_group = chunk_same_characters(&sorted_hand.iter().collect::<String>());
    if chunked_in_group.len() == 1 {
        return HandType::FIVEOFKIND;
    } else if chunked_in_group.len() == 2 {
        if chunked_in_group.iter().any(|group| group.len() == 4) {
            return HandType::FOUROFKIND;
        } else {
            return HandType::FULLHOUSE;
        }
    } else if chunked_in_group.len() == 3 {
        if chunked_in_group.iter().any(|group| group.len() == 3) {
            return HandType::THREEOFKIND;
        } else {
            return HandType::TWOPAIR;
        }
    } else if chunked_in_group.len() == 4 {
        return HandType::ONEPAIR;
    } else {
        return HandType::HIGHCARD;
    }
}

fn chunk_same_characters(input: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut current_chunk = String::new();
    let mut prev_char: Option<char> = None;

    for c in input.chars() {
        match prev_char {
            Some(prev) if prev == c => {
                // Add the character to the current chunk
                current_chunk.push(c);
            }
            Some(_) | None => {
                // Start a new chunk
                if !current_chunk.is_empty() {
                    result.push(current_chunk.clone());
                    current_chunk.clear();
                }
                current_chunk.push(c);
            }
        }

        // Update the previous character
        prev_char = Some(c);
    }

    // Add the last chunk if it's not empty
    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    result
}

fn heap_sort(arr: &mut Vec<Hand>) {
    let n = arr.len();

    // build a max heap by rearranging the array
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // extract the max element from the heap and place it at the end of the array
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

// helper function to turn an array into a max heap
fn heapify(arr: &mut Vec<Hand>, n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // find the largest element among the parent node and its children
    if left < n && arr[left].cmp(&arr[largest]) == Ordering::Greater {
        largest = left;
    }
    if right < n && arr[right].cmp(&arr[largest]) == Ordering::Greater {
        largest = right;
    }

    // if the largest element is not the parent node, swap them and heapify the subtree
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

pub fn process(input: &str) -> String {
    let mut x: Vec<Hand> = vec![];
    input.lines().for_each(|line| {
        let l = line.split_whitespace().collect::<Vec<&str>>();
        let hand = l.get(0).unwrap();
        let score = l.get(1).unwrap().parse::<u32>().unwrap();
        x.push(Hand(hand.to_string(), score));
    });
    heap_sort(&mut x);
    let ans = x
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + hand.1 * (rank + 1) as u32);
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-1.txt");
        let output = process(input);
        assert_eq!(output, "6440");
    }

    #[test]
    fn compare_hands() {
        let h1 = Hand("KK677".to_string(), 765);
        let h2 = Hand("K9JJ9".to_string(), 765);
        assert_eq!(h1 > h2, true);
    }
}
