use std::collections::{HashMap, HashSet};

fn is_valid_soln(soln: &HashMap<char, u8>, factors: &HashMap<char, i64>) -> bool {
    factors.iter().try_fold(0_i64, |acc, (el, &fac)| {
        soln.get(el).map(|&sub| acc + (fac * sub as i64))
    }) == Some(0)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut split = input.split("==");
    let (exp, ans) = match (split.next(), split.next()) {
        (Some(exp), Some(ans)) => (
            exp.split('+').map(str::trim).collect::<Vec<_>>(),
            ans.trim(),
        ),
        _ => return None,
    };
    let mut cannot_be_zero = HashSet::new();
    let factors = exp.iter().fold(HashMap::new(), |mut acc, &el| {
        let len = el.len();
        el.chars().rev().enumerate().for_each(|(idx, char)| {
            if idx == len - 1 {
                cannot_be_zero.insert(char);
            }
            *acc.entry(char).or_insert(0) += 10_i64.pow(idx as u32)
        });
        acc
    });
    let len = ans.len();
    let ans_factors = ans
        .chars()
        .rev()
        .enumerate()
        .fold(factors, |mut acc, (idx, char)| {
            if idx == len - 1 {
                cannot_be_zero.insert(char);
            }
            *acc.entry(char).or_insert(0) -= 10_i64.pow(idx as u32);
            acc
        });

    let chars: Vec<char> = ans_factors.keys().copied().collect();

    fn backtrack(
        idx: usize,
        chars: &Vec<char>,
        mapping: &mut HashMap<char, u8>,
        used: &mut HashSet<u8>,
        ans_factors: &HashMap<char, i64>,
        rem: &[u8],
        cannot_be_zero: &HashSet<char>,
    ) -> Option<HashMap<char, u8>> {
        if idx == chars.len() {
            if is_valid_soln(mapping, ans_factors) {
                return Some(mapping.clone());
            }
            return None;
        }

        let ch = chars[idx];
        for &digit in rem {
            if used.contains(&digit) {
                continue;
            }
            if digit == 0 && cannot_be_zero.contains(&ch) {
                continue;
            }

            mapping.insert(ch, digit);
            used.insert(digit);

            if let Some(soln) = backtrack(idx + 1, chars, mapping, used, ans_factors, rem, cannot_be_zero) {
                return Some(soln);
            }

            mapping.remove(&ch);
            used.remove(&digit);
        }

        None
    }

    backtrack(
        0,
        &chars,
        &mut HashMap::new(),
        &mut HashSet::new(),
        &ans_factors,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        &cannot_be_zero,
    )
}
