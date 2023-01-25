use std::fs;

use section_assignment::SectionAssignment;

mod section_assignment;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input");

    let assignment_pairs = input.lines();

    let assignments = assignment_pairs.map(|assignment_pair| {
        assignment_pair
            .split_once(',')
            .expect("Incorrect assignment pair {assignment_pair}")
    });

    let assignments = assignments.map(|(left, right)| {
        let left: SectionAssignment = left
            .parse()
            .expect("Could not parse left hand assignment {left}");

        let right: SectionAssignment = right
            .parse()
            .expect("Could not parse right hand assignment {right}");

        (left, right)
    });

    let fully_contained_assignments = assignments
        .clone()
        .filter(|(left, right)| (*left).fully_contains(right) || (*right).fully_contains(left))
        .count();

    let overlapping = assignments
        .filter(|(left, right)| (*left).overlaps(right) || (*right).overlaps(left))
        .count();

    println!("{fully_contained_assignments}");
    println!("{overlapping}");
}
