use nom::{
    character::complete::{line_ending, multispace1, u64},
    multi::separated_list1,
    IResult, ToUsize,
};

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<usize>;

    fn parse(_input: &str) -> IResult<&str, Self::Input> {
        separated_list1(multispace1, elf_foods)(_input)
    }

    type Output1 = usize;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        *_input.iter().max().unwrap_or(&0)
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        let mut elves = _input.clone();
        elves.sort();
        elves.reverse();
        elves.iter().take(3).sum()
    }
}

fn elf_foods(input: &str) -> IResult<&str, usize> {
    let (rest, elf_foods) = separated_list1(line_ending, u64)(input)?;
    let elf_foods = elf_foods.iter().map(ToUsize::to_usize).sum();
    Ok((rest, elf_foods))
}
