use crate::solution::Solution;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Solution {
    let indices_sum = input
        .split("\n\n")
        .map(|pair| pair.lines().collect_tuple::<(_, _)>().unwrap())
        .map(|packets| (parse(packets.0.as_bytes(), &mut 1), parse(packets.1.as_bytes(), &mut 1)))
        .map(|(p1, p2)| p1.cmp(&p2))
        .enumerate()
        .filter(|(_, ord)| ord.is_lt())
        .map(|(i, _)| i + 1)
        .sum();

    Solution::USize(indices_sum)
}

pub fn part_two(input: &str) -> Solution {
    let divider_1 = Element::List(vec![Element::Num(2)]);
    let divider_2 = Element::List(vec![Element::Num(6)]);

    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse(l.as_bytes(), &mut 1))
        .collect_vec();

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    // Uses Ord::cmp
    packets.sort_unstable();

    let divider_1_pos = packets.iter().position(|e| *e == divider_1).unwrap() + 1;
    let divider_2_pos = packets.iter().position(|e| *e == divider_2).unwrap() + 1;

    Solution::USize(divider_1_pos * divider_2_pos)
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Element {
    Num(u8),
    List(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Compare two numbers
            (Element::Num(left), Element::Num(right)) => left.cmp(right),

            // Compare two lists
            (Element::List(left), Element::List(right)) => left
                .iter()
                .zip(right.iter())
                .find_map(|(left_elem, right_elem)| {
                    let ord = left_elem.cmp(right_elem);
                    if ord.is_eq() {
                        None
                    } else {
                        Some(ord)
                    }
                })
                .unwrap_or_else(|| left.len().cmp(&right.len())),

            // Compare number to list (mixed type)
            (Element::Num(n), Element::List(_)) => Element::List(vec![Element::Num(*n)]).cmp(other),

            // Compare list to number (mixed type)
            (Element::List(_), Element::Num(n)) => self.cmp(&Element::List(vec![Element::Num(*n)])),
        }
    }
}

// Partial order cmp is not used here, but still needs to be implemented.
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &[u8], pos: &mut usize) -> Element {
    let mut elems: Vec<Element> = Vec::new();
    let mut depth = 1;
    let mut items_to_skip = 0;

    for i in *pos..input.len() {
        let c = input[i];
        if items_to_skip > 0 {
            items_to_skip -= 1;
            continue;
        }

        match c {
            b',' => (),
            b'[' => {
                depth += 1;
                *pos += 1;
                let inner = parse(input, pos);
                elems.push(inner);
                items_to_skip += *pos - i;
            },
            b']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            },
            _ => {
                let mut digit = c - 48;

                // 10 is the only possible two digit number.
                if *input.get(i + 1).unwrap_or(&255) == b'0' {
                    digit = 10;
                    items_to_skip += 1;
                    *pos += 1;
                }

                elems.push(Element::Num(digit));
            }
        }

        *pos += 1;
    }

    Element::List(elems)
}
