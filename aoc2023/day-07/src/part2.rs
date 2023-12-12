use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
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
struct Hand(String, u64);

fn sorter(a: &char, b: &char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    } else {
        match (a, b) {
            ('A', _) => return Ordering::Greater,
            (_, 'A') => return Ordering::Less,
            ('K', _) => return Ordering::Greater,
            (_, 'K') => return Ordering::Less,
            ('Q', _) => return Ordering::Greater,
            (_, 'Q') => return Ordering::Less,
            ('J', _) => return Ordering::Less,
            (_, 'J') => return Ordering::Greater,
            ('T', _) => return Ordering::Greater,
            (_, 'T') => return Ordering::Less,
            (a, b) => {
                // dbg!(a, b);
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

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type = determine_hand_type(&self);
        let other_hand_type = determine_hand_type(&other);
        if self_hand_type.0 as u8 > other_hand_type.0 as u8 {
            return Ordering::Greater;
        } else if (self_hand_type.0 as u8) < (other_hand_type.0 as u8) {
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
                        ('J', _) => return Ordering::Less,
                        (_, 'J') => return Ordering::Greater,
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

// todo: determine handtype return a new bigger card by replaceing J's
fn determine_hand_type(hand: &Hand) -> (HandType, Hand) {
    let upgrade_char = replacing_character(&hand.0);
    let new_hand = match upgrade_char {
        Some(c) => hand.0.replace('J', &c.to_string()),
        None => hand.0.clone(),
    };
    // dbg!(&hand.0, &new_hand);
    let sorted_hand = &mut (new_hand).chars().collect::<Vec<char>>();
    sorted_hand.sort_by(|a, b| sorter(a, b));
    // dbg!(sorted_hand);
    let chunked_in_group = chunk_same_characters(&sorted_hand.iter().collect::<String>());
    if chunked_in_group.len() == 1 {
        return (HandType::FIVEOFKIND, Hand(new_hand.to_string(), hand.1));
    } else if chunked_in_group.len() == 2 {
        if chunked_in_group.iter().any(|group| group.len() == 4) {
            return (HandType::FOUROFKIND, Hand(new_hand.to_string(), hand.1));
        } else {
            return (HandType::FULLHOUSE, Hand(new_hand.to_string(), hand.1));
        }
    } else if chunked_in_group.len() == 3 {
        if chunked_in_group.iter().any(|group| group.len() == 3) {
            return (HandType::THREEOFKIND, Hand(new_hand.to_string(), hand.1));
        } else {
            return (HandType::TWOPAIR, Hand(new_hand.to_string(), hand.1));
        }
    } else if chunked_in_group.len() == 4 {
        return (HandType::ONEPAIR, Hand(new_hand.to_string(), hand.1));
    } else {
        return (HandType::HIGHCARD, Hand(new_hand.to_string(), hand.1));
    }
}

fn replacing_character(hand_string: &String) -> Option<char> {
    if !hand_string.contains("J") {
        return None;
    }

    let sorted_hand = &mut (hand_string).chars().collect::<Vec<char>>();
    sorted_hand.sort_by(|a, b| sorter(a, b));
    // dbg!(&hand.0, &sorted_hand);
    let chunked_in_group = chunk_same_characters(&sorted_hand.iter().collect::<String>());
    if chunked_in_group.len() == 1 {
        return Some('K');
    } else if chunked_in_group.len() == 2 {
        let other = chunked_in_group[1].chars().nth(0).unwrap();
        return Some(other);
    } else if chunked_in_group.len() == 3 {
        if chunked_in_group.iter().any(|group| group.len() == 3) {
            let a = chunked_in_group[1].chars().nth(0).unwrap();
            let b = chunked_in_group[2].chars().nth(0).unwrap();
            match (
                chunked_in_group[0].len(),
                chunked_in_group[1].len(),
                chunked_in_group[2].len(),
            ) {
                (3, 1, 1) => return Some(b),
                (1, 3, 1) => return Some(a),
                (1, 1, 3) => return Some(b),
                _ => !unreachable!(),
            }
        } else {
            let a = chunked_in_group[1].chars().nth(0).unwrap();
            let b = chunked_in_group[2].chars().nth(0).unwrap();
            match (
                chunked_in_group[0].len(),
                chunked_in_group[1].len(),
                chunked_in_group[2].len(),
            ) {
                (2, 2, 1) => return Some(a),
                (2, 1, 2) => return Some(b),
                (1, 2, 2) => return Some(b),
                _ => !unreachable!(),
            };
        }
    } else if chunked_in_group.len() == 4 {
        let a = chunked_in_group[1].chars().nth(0).unwrap();
        let b = chunked_in_group[2].chars().nth(0).unwrap();
        let c = chunked_in_group[3].chars().nth(0).unwrap();
        match (
            chunked_in_group[0].len(),
            chunked_in_group[1].len(),
            chunked_in_group[2].len(),
            chunked_in_group[3].len(),
        ) {
            (2, 1, 1, 1) => return Some(c),
            (1, 2, 1, 1) => return Some(a),
            (1, 1, 2, 1) => return Some(b),
            (1, 1, 1, 2) => return Some(c),
            _ => !unreachable!(),
        };
    } else {
        let d = chunked_in_group[4].chars().nth(0).unwrap();
        return Some(d);
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
        let score = l.get(1).unwrap().parse::<u64>().unwrap();
        let y: (HandType, Hand) = determine_hand_type(&Hand(hand.to_string(), score));
        x.push(y.1);
    });
    heap_sort(&mut x);
    let ans = x
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + hand.1 * (rank + 1) as u64);
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input/test/input-2.txt");
        let output = process(input);
        assert_eq!(output, "5905");
    }
    #[test]
    fn ranking(){
        let mut hands: Vec<Hand> = vec![];
        let h = determine_hand_type(&Hand("8TJJA".to_string(), 1));
        dbg!(&h);
        hands.push(h.1);
        let h = determine_hand_type(&Hand("8QQJ2".to_string(), 1));
        dbg!(&h);
        hands.push(h.1);
        let h = determine_hand_type(&Hand("8K8KA".to_string(), 1));
        dbg!(&h);
        hands.push(h.1);
        let h = determine_hand_type(&Hand("8KQQK".to_string(), 1));
        dbg!(&h);
        hands.push(h.1);
        let h = determine_hand_type(&Hand("8KK86".to_string(), 1));
        dbg!(&h);
        hands.push(h.1);

        heap_sort(&mut hands);

        let sol: Vec<&str> = vec![
            "8QQQ2",
            "8TAAA",
            "8KK86",
            "8KQQK",
            "8K8KA",
        ];

        dbg!(&hands, &sol);

        hands.iter().rev().zip(sol.iter()).for_each(|(h,&s)|{
            // dbg!(&h.0, s);
            // h.0 != s
            // if h.0 != s
            assert_eq!(h.0, s)
        });
    }

    #[test]
    fn compare_hands() {
        let h1 = Hand("KK677".to_string(), 765);
        let h2 = Hand("K9JJ9".to_string(), 765);
        assert_eq!(h1.cmp(&h2), Ordering::Less);
    }

    #[test]
    fn compare_conversion0() {
        let h1 = Hand("K9JJ9".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FOUROFKIND);
        assert_eq!(nh_hand.0, "K9999".to_string());
    }

    #[test]
    fn compare_conversion1() {
        let h1 = Hand("32T3K".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::ONEPAIR);
        assert_eq!(nh_hand.0, "32T3K".to_string());
    }

    #[test]
    fn compare_conversion2() {
        let h1 = Hand("J2222".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FIVEOFKIND);
        assert_eq!(nh_hand.0, "22222".to_string());
    }
    #[test]
    fn compare_conversion3() {
        let h1 = Hand("2JJJJ".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FIVEOFKIND);
        assert_eq!(nh_hand.0, "22222".to_string());
    }
    #[test]
    fn compare_conversion4() {
        let h1 = Hand("T55J5".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FOUROFKIND);
        assert_eq!(nh_hand.0, "T5555".to_string());
    }

    #[test]
    fn compare_conversion5() {
        let h1 = Hand("23333".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FOUROFKIND);
        assert_eq!(nh_hand.0, "23333".to_string());
    }
    #[test]
    fn compare_conversion6() {
        let h1 = Hand("J222J".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FIVEOFKIND);
        assert_eq!(nh_hand.0, "22222".to_string());
    }

    #[test]
    fn compare_conversion7() {
        let h1 = Hand("J2K77".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::THREEOFKIND);
        assert_eq!(nh_hand.0, "72K77".to_string());
    }
    
    #[test]
    fn compare_conversion8() {
        let h1 = Hand("8TJJA".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::THREEOFKIND);
        assert_eq!(nh_hand.0, "8TAAA".to_string());
    }
    
    #[test]
    fn compare_conversion9() {
        let h1 = Hand("KTJJT".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FOUROFKIND);
        assert_eq!(nh_hand.0, "KTTTT".to_string());
    }
    
    #[test]
    fn compare_conversion10() {
        let h1 = Hand("666JJ".to_string(), 0);
        let (nh_type, nh_hand) = determine_hand_type(&h1);
        assert_eq!(nh_type, HandType::FIVEOFKIND);
        assert_eq!(nh_hand.0, "66666".to_string());
    }
}
