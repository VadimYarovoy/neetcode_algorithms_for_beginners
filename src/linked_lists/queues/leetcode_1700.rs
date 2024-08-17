use std::collections::VecDeque;

#[allow(dead_code)]
fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut students: VecDeque<i32> = students.into();

    let mut hungry = students.len() as i32;

    'outer: for sw in sandwiches {
        let curr_len = students.len();
        let mut skiped = 0;

        loop {
            if curr_len < skiped || students.is_empty() {
                break 'outer;
            }
            if students[0] == sw {
                students.pop_front();
                hungry -= 1;
                break;
            } else {
                let rounded = students.pop_front().unwrap();
                students.push_back(rounded);
                skiped += 1;
            }
        }
    }

    hungry
}

#[cfg(test)]
mod count_students_tests {
    use super::count_students;

    #[test]
    fn no_hungry() {
        assert_eq!(count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
    }

    #[test]
    fn some_hungry() {
        assert_eq!(
            count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
