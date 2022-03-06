use std::collections::HashMap;
use std::collections::HashSet;
// use std::cmp;
//
// For reference
//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

pub fn day8(input: &str, partb: bool) -> i64{
    let unique_segments : [u8; 10] = [6,2,5,5,4,5,6,3,7,6];

    let lines : Vec<&str> = input.split_terminator('\n').collect();
    let mut part_b : i64 = 0;
    let mut part_a : i64 = 0;

    for line in lines.iter(){
        let mut combinations : HashMap<i32, HashSet<char>> = HashMap::new();
        let mut output_sum = 0;

        let values : Vec<&str> = line.split('|').collect();
        let input_values : Vec<&str> = values[0].split_ascii_whitespace().map(|str| str.trim()).collect();
        let output_values : Vec<&str> = values[1].split_ascii_whitespace().map(|str| str.trim()).collect();
        let mut five_chars : Vec<HashSet<char>> = vec![];
        let mut six_chars : Vec<HashSet<char>> = vec![];

        for output in output_values.iter(){
            if output.len() != 5 && output.len() != 6{
                part_a += 1;
            }
        }

        for input_str in input_values.iter() {
            if input_str.len() == 5 {
                five_chars.push(input_str.chars().collect());
            } else if input_str.len() == 6 {
                six_chars.push(input_str.chars().collect());
            } else if input_str.len() == 2 {
                combinations.insert(1, input_str.chars().collect());
            } else if input_str.len() == 3 {
                combinations.insert(7, input_str.chars().collect());
            } else if input_str.len() == 4 {
                combinations.insert(4, input_str.chars().collect());
            } else if input_str.len() == 7 {
                combinations.insert(8, input_str.chars().collect());
            } else {
                println!("Unmatched input_str {} len {}",input_str,input_str.len());
            };
        }

        for combination in five_chars {
            let intersection_4 : HashSet<_> = combination.intersection(&combinations[&4]).collect();
            if intersection_4.len() == 2{
                combinations.insert(2,combination);
            } else {
                if combinations[&1].is_subset(&combination){
                    combinations.insert(3, combination);
                } else{
                    combinations.insert(5, combination);
                }
            }
        }

        for combination in six_chars {
            if combinations[&4].is_subset(&combination){
                combinations.insert(9, combination);
            } else {
                if combinations[&7].is_subset(&combination){
                    combinations.insert(0, combination);
                } else{
                    combinations.insert(6, combination);
                }
            }
        }

        for output in output_values{
            // println!("Matching output: {}", output);
            for (num,combination) in combinations.iter() {
                let output_chars : HashSet<char> = output.chars().collect();
                if output_chars == *combination  {
                    // println!("Matched combination: {:?}, {:?}", output_chars, combination);
                    output_sum = output_sum*10 + num;
                    break;
                }
            }
        }
        println!("Output sum for line {} : {}",line, output_sum);
        part_b += output_sum as i64;
    }

    if partb == false{
        return part_a;
    }

    return part_b;
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 26");
        assert_eq!(super::day8(include_str!("example2.txt"), false),26);
    }

    // #[test]
    // fn part_a_example_2() {
    //     println!("Running part 1 on example: 5934");
    //     assert_eq!(super::day7(include_str!("example.txt"), false),5934);
    // }


    #[test]
    fn part_a() {
        let result = super::day8(include_str!("input.txt"), false);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 5353");
        let result = super::day8(include_str!("example.txt"), true);
        assert_eq!(result,5353);
    }

    #[test]
    fn part_b_example2() {
        println!("Running part 2 on example2: 61229");
        let result = super::day8(include_str!("example2.txt"), true);
        assert_eq!(result,61229);
    }


    #[test]
    fn part_b() {
        let result = super::day8(include_str!("input.txt"), true);
        println!("Running part 2 on input: {result}");
    }

}
