// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::cmp;

pub fn day7(input: &str, partb: bool) -> i64{
    let mut numbers : Vec<i32> = input.split(',').map(|str| str.trim().to_string().parse().unwrap()).collect();

    // Calculate median
    numbers.sort();
    let mid = numbers.len() / 2;
    let median = numbers[mid];

    if partb == false{
        let mut cost = 0;
        for number in numbers.iter(){
            cost += (number-median).abs();
        }
        return cost as i64;
    }

    let mut reference = median;

    loop {
        // println!("Reference: {}", reference);
        let mut new_min = 0.0;
        let mut left_min = 0.0;
        let mut right_min = 0.0;

        for number in numbers.iter(){
            let n = (number-reference).abs() as f64;
            let leftn = (number-(reference-1)).abs() as f64;
            let rightn = (number-(reference+1)).abs() as f64;
            new_min += n/2.0 * (n+1.0);
            left_min += leftn/2.0 * (leftn+1.0);
            right_min += rightn/2.0 * (rightn+1.0);
        }

        if left_min < new_min {
            reference -= 1;
            continue;
        }
        if right_min < new_min {
            reference += 1;
            continue;
        }
        println!("Best reference: {}",reference);
        println!("Least cost fuel: {}",new_min);
        return new_min as i64;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 37");
        assert_eq!(super::day7(include_str!("example.txt"), false),37);
    }

    // #[test]
    // fn part_a_example_2() {
    //     println!("Running part 1 on example: 5934");
    //     assert_eq!(super::day7(include_str!("example.txt"), false),5934);
    // }


    #[test]
    fn part_a() {
        let result = super::day7(include_str!("input.txt"), false);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 168");
        let result = super::day7(include_str!("example.txt"), true);
        assert_eq!(result,168);
    }

    #[test]
    fn part_b() {
        let result = super::day7(include_str!("input.txt"), true);
        println!("Running part 2 on input: {result}");
    }

}
