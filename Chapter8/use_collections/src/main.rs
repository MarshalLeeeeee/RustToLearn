use std::collections::HashMap;
use std::cmp::Ordering;

fn get_mode(v: &Vec<i32>) -> (i32, i32) {
    let mut count_map = HashMap::new();
    for vo in v {
        count_map.entry(*vo).and_modify(|o| *o += 1).or_insert(1);
    }
    let mut max_count_vo: i32 = 0;
    let mut max_count: i32 = -1;
    for (k, v) in count_map.iter() {
        if *v > max_count {
            max_count = *v;
            max_count_vo = *k;
        }
    }
    (max_count_vo, max_count)
}

fn do_get_median(v: &mut Vec<i32>, target: usize, head: usize, tail: usize) -> f64 {
    if head >= tail {
        v[head] as f64
    }
    else {
        let pivot = v[head];
        let mut j = head+1; // the number of object that are <= pivot
        for i in (head+1)..(tail) {
            if v[i] <= pivot {
                if i > j {
                    let tmp = v[i];
                    v[i] = v[j];
                    v[j] = tmp;
                }
                j += 1;
            }
        }
        v[head] = v[j-1];
        v[j-1] = pivot;
        if j-1 < target {
            do_get_median(v, target, j, tail)
        }
        else if j-1 > target {
            do_get_median(v, target, head, j-1)
        }
        else {
            pivot as f64
        }
    }

}

fn get_median(v: &mut Vec<i32>) -> f64 {
    let sz = v.len();
    if sz % 2 == 0 {
        (do_get_median(v, sz / 2 - 1, 0, sz) + do_get_median(v, sz / 2, 0, sz)) / 2.0
    }
    else {
        do_get_median(v, sz / 2 - 1, 0, sz)
    }
}

fn test_vector_statistic() {
    let mut v = vec![10, 5, 3, 9, 5, 2, 6, 8, 4, 7];
    // calc mode
    let (max_count_vo, max_count) = get_mode(&v);
    println!("max_count_vo: {}, max_count: {}", max_count_vo, max_count);
    // calc median
    let median = get_median(&mut v);
    println!("median: {}", median);
}

fn string_modify(s: &str) -> String {
    let mut ss = String::new();
    if !s.is_empty() {
        let first_char_option = s.chars().nth(0);
        let mut first_chat_length = 1;
        if let Some(first_char) = first_char_option {
            first_chat_length = first_char.len_utf8();
        }
        let first_char = &s[..first_chat_length];
        let vowel_str = String::from("aeiouAEIOU");
        if vowel_str.contains(first_char) {
            ss.push_str(&s[..]);
            ss.push_str("-hay");
        }
        else {
            ss.push_str(&s[first_chat_length..]);
            ss.push_str("-");
            ss.push_str(first_char);
            ss.push_str("ay");
        }
    }
    ss
}

fn test_string_modify() {
    let s1 = String::from("apple");
    let s2 = String::from("banana");
    let s3 = String::from("你好");
    println!("pig latin of {} is {}", s1, string_modify(&s1));
    println!("pig latin of {} is {}", s2, string_modify(&s2));
    println!("pig latin of {} is {}", s3, string_modify(&s3));
}

struct Employee {
    name: String,
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        println!("partial eq");
        self.name.eq(&other.name)
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("ord");
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        println!("partial ord");
        self.name.partial_cmp(&other.name)
    }
}

fn test_department_management() {
    let mut departments: HashMap<String, Vec<Employee>> = HashMap::new();
    departments.entry(String::from("Engineering")).or_insert(Vec::new()).push(
        Employee {
            name: String::from("Sally"),
        }
    );
    departments.entry(String::from("Engineering")).or_insert(Vec::new()).push(
        Employee {
            name: String::from("Bob"),
        }
    );
    departments.entry(String::from("Sale")).or_insert(Vec::new()).push(
        Employee {
            name: String::from("John"),
        }
    );
    departments.entry(String::from("Sale")).and_modify(|v| v.sort());
    if let Some(engineer_employees) = departments.get_mut(&String::from("Engineering")) {
        engineer_employees.sort();
        for e in engineer_employees {
            println!("engineer {}", e.name);
        }
    }
}

fn main() {
    test_vector_statistic();
    test_string_modify();
    test_department_management();
}
