use std::collections::HashMap;
use std::collections::HashSet;

struct Board{
    id: i32,
    grid: [[bool; 5]; 5],
    positions: HashMap<i32, (i32,i32)>,
    bingo_grid: [[i32; 5]; 5]
}

impl Board {

    pub fn check_bingo(&mut self, number: i32) -> bool{
        let number_position = self.positions.get(&number);
        if number_position.is_none(){
            return false;
        }
        let coord = number_position.unwrap();
        self.grid[coord.0 as usize][coord.1 as usize] = true;

        let row = &self.grid[coord.0 as usize];
        let mut column : Vec<bool> =  vec![];
        for i in 0..5{
            column.push(self.grid[i as usize][coord.1 as usize]);
        }

        let mut is_bingo = true;
        for i in 0..5{
            is_bingo = is_bingo && row[i];
        }
        if is_bingo{
            return true;
        }

        is_bingo = true;
        for i in 0..5{
            is_bingo = is_bingo && column[i];
        }
        return is_bingo;
    }

    pub fn note_position(&mut self, number: i32, coord: (i32,i32)){
        self.positions.insert(number, coord);
        self.bingo_grid[coord.0 as usize][coord.1 as usize] = number;
    }

    pub fn sum_unmarked(&mut self) -> i32{
        let mut sum = 0;
        for i in 0..5{
            for j in 0..5{
                if self.grid[i][j] != true{
                    sum += self.bingo_grid[i][j];
                }
            }
        }
        return sum;
    }

    pub fn info(&mut self){
        self.grid.iter().for_each(|it|{
            println!("{:#?}", it);
        });

        for (key, value) in &self.positions {
            println!("{}: {},{}", key, value.0, value.1);
        }

        self.bingo_grid.iter().for_each(|it|{
            println!("{:#?}", it);
        });

    }
}

pub fn bingo(input: &str, part_b: bool) -> i32 {
    let mut iter = input.split("\n\n");
    let numbers = iter.next().unwrap();
    // println!("{numbers}");

    let mut boards : Vec<Board> = vec![];
    let mut id = 1;

    for grid in iter{
        // println!("{grid}");
        let mut x = 0;

        let mut board : Board = Board{id: id, grid: [[false; 5]; 5], positions: HashMap::new(), bingo_grid: [[0; 5]; 5]};

        for row in grid.split('\n'){
            let mut y = 0;
            for numstring in row.split_ascii_whitespace(){
                let number = numstring.to_string().parse::<i32>().unwrap();
                board.note_position(number, (x,y));
                y += 1;
            }
            x += 1;
        }
        // board.info();
        boards.push(board);
        id+=1;
        // println!("");
    }

    let mut winning_score : i32 = 0;
    let mut winner_ids : HashSet<i32> = HashSet::new();
    for numstring in numbers.split(','){
        // println!("{numstring}");
        let number = numstring.to_string().parse::<i32>().unwrap();
        for board in boards.iter_mut(){
            if winner_ids.contains(&board.id){
                continue;
            }

            let is_bingo = board.check_bingo(number);
            if is_bingo == true {
                winner_ids.insert(board.id);
                let unmarked_sum = board.sum_unmarked();
                winning_score = unmarked_sum * number;
                println!("Winning score: {winning_score}");
                if part_b == false{
                    return winning_score;
                }
            }
        }
    }

    if part_b == true {
        return winning_score;
    }

    return 0;
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
        println!("Running part 1 on example: 4512");
        assert_eq!(super::bingo(include_str!("example.txt") , false),4512);
    }

    #[test]
    fn part_a() {
        let result = super::bingo(include_str!("input.txt"), false);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 1924");
        let result = super::bingo(include_str!("example.txt"), true);
        assert_eq!(result,1924);
    }

    #[test]
    fn part_b() {
        let result = super::bingo(include_str!("input.txt"), true);
        println!("Running part 2 on input: {result}");
        assert_eq!(result,1924);
    }

}
