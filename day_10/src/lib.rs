use std::collections::HashMap;
use std::collections::HashSet;
// use std::cmp;
use std::collections::VecDeque;

pub fn day10(input: &str, is_partb: bool) -> u64{
    let lines : Vec<&str> = input.split_terminator('\n').collect();
    let openers : HashMap<char, char> = HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
        ('<','>')
    ]);
    let closers : HashMap<char, char> = HashMap::from([
        (')','('),
        (']','['),
        ('}','{'),
        ('>','<')
    ]);

    let points : HashMap<char, u64> = HashMap::from([
        (')',3),
        (']',57),
        ('}',1197),
        ('>',25137)
    ]);

    let points2 : HashMap<char, u64> = HashMap::from([
        (')',1),
        (']',2),
        ('}',3),
        ('>',4)
    ]);


    let mut parta: u64 = 0;
    let mut partb: u64 = 0;

    let mut scores : Vec<u64> = vec![];

    for line in lines{
        let mut stack : VecDeque<char> = VecDeque::new();
        let mut is_corrupted = false;
        for chr in line.chars(){
            if openers.contains_key(&chr){
                stack.push_front(chr);
            } else if closers.contains_key(&chr){
                if stack[0] == closers[&chr]{
                    stack.pop_front();
                } else {
                    // Illegal character
                    is_corrupted = true;
                    let point = points[&chr];
                    parta += point;
                    break;
                }
            }
        }

        if is_partb && is_corrupted == false {
            let mut score = 0;
            while !stack.is_empty(){
                score = score*5 + points2[&openers[&stack[0]]];
                stack.pop_front();
            }
            scores.push(score);
        }
    }

    if is_partb == false{
        return parta;
    }

    scores.sort();
    let mid = scores.len() / 2;
    let median = scores[mid];

    return median;
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 26397");
        assert_eq!(super::day10(include_str!("example.txt"), false),26397);
    }

    // #[test]
    // fn part_a_example_2() {
    //     println!("Running part 1 on example: 5934");
    //     assert_eq!(super::day7(include_str!("example2.txt"), false),5934);
    // }


    #[test]
    fn part_a() {
        let result = super::day10(include_str!("input.txt"), false);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 288957");
        let result = super::day10(include_str!("example.txt"), true);
        assert_eq!(result,288957);
    }

    // #[test]
    // fn part_b_example2() {
    //     println!("Running part 2 on example2: 61229");
    //     let result = super::day10(include_str!("example2.txt"), true);
    //     assert_eq!(result,61229);
    // }


    #[test]
    fn part_b() {
        let result = super::day10(include_str!("input.txt"), true);
        println!("Running part 2 on input: {result}");
    }

}
