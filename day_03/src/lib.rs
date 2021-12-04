pub fn max_bit(bits: &Vec<i64>) -> i64{
    let mut max_bit = 0;
    if bits[1] >= bits[0]{
        max_bit = 1;
    }
    max_bit
}

pub fn min_bit(bits: &Vec<i64>) -> i64{
    let mut min_bit = 0;
    if bits[1] < bits[0]{
        min_bit = 1;
    }
    min_bit
}

pub fn part_a(input: &str) -> i64 {
    let mut bits : Vec<Vec<i64>> = vec![];
    let mut iter = input.lines();
    let first_bit : &str = iter.next().unwrap();
    for _i in 0..first_bit.len(){
        bits.push(vec![0,0]);
    }

    for line in input.split('\n'){
        let line_bits : Vec<char> = line.chars().collect();
        for i in 0..line_bits.len(){
            let bit : char = line_bits[i];
            match bit {
                '0' => {bits[i][0] += 1;}
                '1' => {bits[i][1] += 1;}
                _ => panic!(),
            }

        }
    }

    let mut gamma : i64 = 0;
    let mut epsilon : i64 = 0;
    for i in 0..bits.len(){
        gamma += max_bit(&bits[i]) << bits.len()-i-1;
        epsilon += min_bit(&bits[i]) << bits.len()-i-1;
    }

    bits.iter().for_each(|it| {
        println!("{:#?}", it);
   });

    println!("Gamma: {gamma} Epsilon {epsilon}");

    return gamma*epsilon;
}

pub fn part_b(input: &str) -> i64 {
    let line_chars : Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width : usize = line_chars[0].len();
    let carbon_binary : String = part_b_helper(width, line_chars.clone(), false).unwrap_or_default().into_iter().collect();
    let oxygen_binary : String = part_b_helper(width, line_chars.clone(), true).unwrap_or_default().into_iter().collect();
    println!("Oxygen: {oxygen_binary}, Carbon: {carbon_binary}");
    let oxygen = i64::from_str_radix(&oxygen_binary, 2).unwrap_or_default();
    let carbon = i64::from_str_radix(&carbon_binary, 2).unwrap_or_default();
    return oxygen*carbon
}

pub fn part_b_helper(width: usize, mut line_chars: Vec<Vec<char>>, get_majority: bool) -> Option<Vec<char>> {
    let mut ones = 0;
    let mut zeros = 0;
    let mut majority;
    let mut minority;
    println!("Width: {width}");
    println!("Lines: {}",line_chars.len());
    for i in 0..width{
        for idx in 0..line_chars.len(){
            match line_chars[idx][i] {
                '0' => { zeros += 1}
                '1' => { ones += 1}
                _ => panic!(),
            }
        }
        if ones >= zeros {
            majority = '1';
            minority = '0';
        } else {
            majority = '0';
            minority = '1';
        }

        if get_majority == true{
            line_chars.retain(|line| line[i] == majority);
            // println!("Majority?: {:#?}", line_chars);
        } else {
            line_chars.retain(|line| line[i] == minority);
            // println!("Minority?: {:#?}", line_chars);
        }
        zeros = 0;
        ones = 0;
        if line_chars.len() == 1{
            return Some(line_chars[0].clone())
        }
    }

    if line_chars.len() == 1{
        Some(line_chars[0].clone())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 1988");
        assert_eq!(super::part_a(include_str!("example.txt")),198);
    }

    #[test]
    fn part_a() {
        let result = super::part_a(include_str!("input.txt"));
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 230");
        assert_eq!(super::part_b(include_str!("example.txt")),230);
    }

    #[test]
    fn part_b() {
        let result = super::part_b(include_str!("input.txt"));
        println!("Running part 2 on input: {result}");
    }

}
