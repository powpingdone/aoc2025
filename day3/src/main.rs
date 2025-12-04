fn process_input(inp: &str) -> Box<[Box<[u8]>]> {
    inp.split("\n")
        .map(|x| x.trim().bytes().map(|x| x - 48).collect())
        .filter(|x: &Box<[_]>| !x.is_empty())
        .collect()
}

fn day3_p1(jolts: &[u8]) -> u8 {
    let mut tens = 0;
    // find highest lower bound
    for (e, i) in jolts.iter().enumerate() {
        if *i > jolts[tens] && e <= jolts.len() - 2 {
            tens = e;
        }
    }
    // starting on the lower bound's next digit, find the next highest digit
    let mut ones = tens + 1;
    for (e, i) in jolts[ones..].iter().enumerate() {
        if *i > jolts[ones] {
            ones = tens + 1 + e;
        }
    }
    // constuct the final joltage
    10 * jolts[tens] + jolts[ones]
}

fn day3_p2(jolts: &[u8]) -> u64 {
    let mut vec = jolts.to_vec();
    'outer: while vec.len() > 12 {
        for i in 0..vec.len() - 1 {
            // if the most sigificant digit is smaller than the least significant digit
            // remove it
            if vec[i] < vec[i + 1] {
                vec.remove(i);
                continue 'outer;
            }
        }
        // if there are no digits that are like that, remove the least significant digit
        vec.pop();
    }
    vec.into_iter().fold(0u64, |acc, x| acc * 10 + x as u64)
}

fn main() {
    let input = process_input(&std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap());
    let mut p1jolts = vec![];
    let mut p2jolts = vec![];
    for battery_pack in input {
        p1jolts.push(day3_p1(&battery_pack));
        p2jolts.push(day3_p2(&battery_pack));
    }
    let p1jolts_sum = p1jolts.into_iter().map(|x| x as u64).sum::<u64>();
    let p2jolts_sum = p2jolts.into_iter().sum::<u64>();
    dbg!(p1jolts_sum);
    dbg!(p2jolts_sum);
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn p1() {
        let input = process_input(INPUT);
        let ret = input
            .into_iter()
            .map(|x| day3_p1(&x) as u64)
            .collect::<Box<_>>();
        dbg!(&ret);
        assert_eq!(ret.into_iter().sum::<u64>(), 357);
    }
    #[test]
    fn p2() {
        let input = process_input(INPUT);
        let ret = input
            .into_iter()
            .map(|x| day3_p2(&x) as u64)
            .collect::<Box<_>>();
        dbg!(&ret);
        assert_eq!(ret.into_iter().sum::<u64>(), 3121910778619);
    }
}
