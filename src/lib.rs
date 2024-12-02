use std::{fs::read_to_string, usize};

pub fn find_safe_reports(input_path: &str, tolerance: u32) -> u32 {
    let puzzle = extract_puzzle(input_path);

    safe_reports(&puzzle, tolerance)
}

fn extract_puzzle(input_path: &str) -> Puzzle {
    let mut puzzle = Puzzle::new();
    read_to_string(input_path)
    .unwrap()
    .lines()
    .for_each(|line| puzzle.add_report(line));

    puzzle
}

fn safe_reports(puzzle: &Puzzle, tolerance: u32) -> u32 {
    if puzzle.reports.is_empty() {
        return 0;
    }
    let mut safe_count = 0;
    puzzle.reports.iter().for_each(|report| {
        if report.is_safe(tolerance as usize) {
            safe_count += 1;
        }
    });
    safe_count
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Puzzle {
    reports: Vec<Report>
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Report {
    values: Vec<u32>
}

impl Report {

    fn is_safe_at_index(&self, index: usize) -> bool {
        if index == self.values.len() - 1 {
            return true;
        }
        let value = self.values[index];
        let next_value = self.values[index + 1];
    
        value.abs_diff(next_value) > 0 && value.abs_diff(next_value) < 4
    }
    
    fn is_wright_order(&self, index: usize) -> bool {
        if index == self.values.len() - 1 {
            return true;
        }
        let value = self.values[index];
        let next_value = self.values[index + 1];
    
        if self.is_ascending() {
            return next_value > value;
        }
        next_value < value
    }
    
    fn is_ascending(&self) -> bool {
        self.values.len() >= 2 && self.values[0] < self.values[1]
    }

    fn is_safe(&self, tolerance: usize) -> bool {
        if self.values.is_empty() {
            return false;
        }
        
        if tolerance == 0 {
            return self.values.iter().enumerate().all(|(i, _)| self.is_safe_at_index(i) && self.is_wright_order(i));
        }

        let n = self.values.len();
        let required_len = n.saturating_sub(tolerance);

        self.get_subsequences(required_len)
        .iter()
        .any(|subsequence| Report { values: subsequence.clone() }.is_safe(0))
    }
    
    fn get_subsequences(&self, length: usize) -> Vec<Vec<u32>> {
        let mut subsequences = Vec::new();
        let values = &self.values;
        let mut current = Vec::new();
        generate_subsequences(values, 0, length, &mut current, &mut subsequences);
        subsequences
    }
}



fn generate_subsequences(
    values: &[u32],
    start: usize,
    length: usize,
    current: &mut Vec<u32>,
    result: &mut Vec<Vec<u32>>,
) {
    if current.len() == length {
        result.push(current.clone());
        return;
    }
    for i in start..values.len() {
        current.push(values[i]);
        generate_subsequences(values, i + 1, length, current, result);
        current.pop();
    }
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            reports: Vec::new()
        }
    }

    fn add_report(&mut self, line: &str) {
        let numbers: Vec<u32> = line
        .split_whitespace() 
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

        self.reports.push(Report {
            values: numbers
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_safe_reports_0_when_tolerant_but_over_on_first_element() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 7, 6, 3, 1]
            }]
        }, 1), 1);
    }

    #[test]
    fn should_safe_reports_0_when_tolerant_but_over_on_last_element() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![9, 7, 6, 5, 1]
            }]
        }, 1), 1);
    }

    #[test]
    fn should_safe_reports_0_when_tolerant_but_over() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![9, 7, 6, 2, 1]
            }]
        }, 1), 0);
    }

    #[test]
    fn should_safe_reports_1_when_tolerant() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 3, 2]
            }]
        }, 1), 1);
    }


    #[test]
    fn should_safe_reports_0_when_safe_distance_but_not_always_increasing() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 3, 1]
            }]
        }, 0), 0);
    }

    #[test]
    fn should_safe_reports_2_when_two_reports_safe() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 3]
            }, Report {
                values: vec![1, 2]
            }]
        }, 0), 2);
    }
   
    #[test]
    fn should_safe_reports_1_when_one_reports_is_containing_1_5() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 5]
            }]
        }, 0), 0);
    }

    #[test]
    fn should_safe_reports_1_when_one_reports_is_containing_1_1() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 1]
            }]
        }, 0), 0);
    }


    #[test]
    fn should_safe_reports_1_when_one_reports_is_containing_1_2() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![Report {
                values: vec![1, 2]
            }]
        }, 0), 1);
    }

    #[test]
    fn should_safe_reports_0_when_empty_reports() {
        assert_eq!(safe_reports(&Puzzle {
            reports: vec![]
        }, 0), 0);
    }

    #[test]
    fn should_extract_puzzle() {
        assert_eq!(extract_puzzle("tests/resources/puzzle.txt"), 
        Puzzle {
            reports: vec![Report {
                values: vec![7, 6, 4, 2, 1]
            },
            Report {
                values: vec![1, 2, 7, 8, 9]
            },
            Report {
                values: vec![9, 7, 6, 2, 1]
            },
            Report {
                values: vec![1, 3, 2, 4, 5]
            },
            Report {
                values: vec![8, 6, 4, 4, 1]
            },
            Report {
                values: vec![1, 3, 6, 7, 9]
            }]
        })
        
    }

    
}