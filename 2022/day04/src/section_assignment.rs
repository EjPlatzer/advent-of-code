use std::str::FromStr;

pub struct SectionAssignment {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum SectionAssignmentParseError {
    Unknown(String),
    InvalidStart,
    InvalidEnd,
}

impl SectionAssignment {
    pub fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end <= other.end && self.end >= other.start)
    }
}

impl FromStr for SectionAssignment {
    type Err = SectionAssignmentParseError;

    fn from_str(assignment: &str) -> Result<Self, Self::Err> {
        let (start, end) =
            assignment
                .split_once('-')
                .ok_or(SectionAssignmentParseError::Unknown(String::from(
                    "Could not parse assignment {assignment}",
                )))?;

        let start = start
            .parse()
            .or(Err(SectionAssignmentParseError::InvalidStart))?;
        let end = end
            .parse()
            .or(Err(SectionAssignmentParseError::InvalidEnd))?;

        Ok(SectionAssignment { start, end })
    }
}
