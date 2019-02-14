use std::vec::Vec;
use std::rc::Rc;
use std::cmp;
use std::i32;

fn get_bit_size(num: u32) -> u32 {
    ((num as f64).log2().floor() as u32) + 1
}

fn to_binary_vec(num: u32) -> Vec<bool> {
    let bit_size = get_bit_size(num);

    (0..bit_size).map(|i| {
        let bit_index = (bit_size - 1) - i;
        (0b1 << bit_index & num) >> bit_index == 1
    }).collect::<Vec<bool>>()
}

fn to_fixed_binary_vec(num: u32, bit_size: u32) -> Vec<bool> {
    (0..(bit_size + 1)).map(|i| {
        let bit_index = bit_size - i;
        (0b1 << bit_index & num) >> bit_index == 1
    }).collect::<Vec<bool>>()
}

fn get_least_sum(input: Vec<i32>, set_pattern: Vec<bool>) -> Option<i32> {
    if input.len() != set_pattern.len() {
        return None;
    }
    let (sum1, sum2) = input.iter()
                            .zip(set_pattern)
                            .inspect(|(x,y)| println!("3: num:{:?} take:{:?}", x, y))
                            .fold((0, 0), |(sum_left, sum_right),(num, should_take)|{
                                if should_take {
                                    (sum_left + num, sum_right)
                                } else {
                                    (sum_left, sum_right + num)
                                }
                            });

    Some((sum1 - sum2).abs())
}

fn get_least_sum_sets(input: Vec<i32>) -> i32 {

    let input_ref = Rc::new(input);

    (0..2_u32.pow(input_ref.len() as u32))
        .map(|x| to_fixed_binary_vec(x, get_bit_size(input_ref.len() as u32)))
        .map(|pattern| get_least_sum(input_ref.to_vec(), pattern))
        .fold(i32::MAX, |current_min, x| cmp::min(x.unwrap_or(i32::MAX), current_min))
}

#[cfg(test)]
mod sum_subset_tests {
    use super::*;

    #[test]
    fn get_bit_size_gets_bit_size() {
        assert_eq!(get_bit_size(0), 1);
        assert_eq!(get_bit_size(1), 1);
        assert_eq!(get_bit_size(2), 2);
        assert_eq!(get_bit_size(3), 2);
        assert_eq!(get_bit_size(4), 3);
        assert_eq!(get_bit_size(5), 3);
        assert_eq!(get_bit_size(6), 3);
        assert_eq!(get_bit_size(7), 3);
        assert_eq!(get_bit_size(8), 4);
    }

    #[test]
    fn to_binary_array_converts_to_array() {
        assert_eq!(to_binary_vec(0), [false]);
        assert_eq!(to_binary_vec(1), [true]);
        assert_eq!(to_binary_vec(2), [true, false]);
        assert_eq!(to_binary_vec(3), [true, true]);
        assert_eq!(to_binary_vec(4), [true, false, false]);
        assert_eq!(to_binary_vec(5), [true, false, true]);
        assert_eq!(to_binary_vec(6), [true, true, false]);
        assert_eq!(to_binary_vec(7), [true, true, true]);
        assert_eq!(to_binary_vec(8), [true, false, false, false]);
    }

    #[test]
    fn get_least_sum_sets_gets_right_answer(){
        assert_eq!(get_least_sum_sets(vec![1, 6, 11, 5]), 1);
    }
}
