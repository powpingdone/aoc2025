use std::{collections::HashSet, ops::RangeInclusive};

fn parse_line(inp: &str) -> Box<[RangeInclusive<u64>]> {
    inp.split(",")
        .map(|x| {
            let (l, r) = x.split_once('-').unwrap();
            let l = l.parse::<u64>().unwrap();
            let r = r.parse::<u64>().unwrap();
            l..=r
        })
        .collect::<Box<[_]>>()
}

fn range_find_p1(range: RangeInclusive<u64>) -> Vec<u64> {
    let mut ret = vec![];
    for i in range {
        // get size of number
        let digits: usize = i.to_string().len();
        // if odd, bail. we cant evenly split 1234123
        // (it also doesn't mirror)
        if digits % 2 == 1 {
            continue;
        }
        // get the power required to split in half
        let ten_pow = 10u64.pow(digits as u32 / 2);
        // the div gets the top half, the mod gets the lower
        if i / ten_pow == i % ten_pow {
            ret.push(i);
        }
    }
    ret
}

fn range_find_p2(range: RangeInclusive<u64>) -> Vec<u64> {
    let mut ret = vec![];
    for i in range {
        if i < 11 {
            continue;
        }
        let old_i = i;
        let digits = i.to_string().len();
        let i = i.to_string();
        // digit_len == 1 check
        let uniq = HashSet::<char>::from_iter(i.chars());
        if uniq.len() == 1 {
            ret.push(old_i);
            continue;
        }
        // digit_len ++ check
        for digit_len in 2..=(digits / 2) {
            // digit_len must be a multiple of the length
            // otherwise we have issues with overflow
            if i.len() % digit_len != 0 {
                continue;
            }
            // for each, take a subslice of the current string
            // and compare a smaller subsplice of that string
            let cmp_slice = &i[0..digit_len];
            let mut i = i.as_str();
            while cmp_slice.len() < i.len() {
                // break loop if this is not equal
                if cmp_slice != &i[0..digit_len] {
                    break;
                }
                i = &i[digit_len..];
            }
            // if the final one is eq, then the whole thing repeats
            if cmp_slice == i {
                ret.push(old_i);
                break;
            }
        }
    }
    ret
}

fn main() {
    let finp = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let parsed = parse_line(finp.trim());
    let mut repeatedp1 = vec![];
    let mut repeatedp2 = vec![];
    for range in parsed {
        repeatedp1.extend(range_find_p1(range.clone()));
        repeatedp2.extend(range_find_p2(range));
    }
    dbg!(repeatedp1.iter().sum::<u64>());
    dbg!(repeatedp2.iter().sum::<u64>());
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_STR: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn pt1() {
        let ranges = parse_line(TEST_STR);
        let repeated = ranges
            .into_iter()
            .map(|x| range_find_p1(x))
            .flatten()
            .collect::<Vec<_>>();
        dbg!(&repeated);
        assert_eq!(repeated.iter().sum::<u64>(), 1227775554);
    }

    #[test]
    fn pt2() {
        let ranges = parse_line(TEST_STR);
        let repeated = ranges
            .into_iter()
            .map(|x| range_find_p2(x))
            .flatten()
            .collect::<Vec<_>>();
        dbg!(&repeated);
        assert_eq!(repeated.iter().sum::<u64>(), 4174379265);
    }
}
