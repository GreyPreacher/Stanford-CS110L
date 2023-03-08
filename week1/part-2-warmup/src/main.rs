/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for i in v.iter() {
        new_vec.push(i + n);
    }
    new_vec
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in 0..v.len() {
        v[i] = v[i] +n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut hash_set = HashSet::new(); //do not forget mut
    let mut new_vec: Vec<i32> = Vec::new();
    for i in 0..v.len() {
        if hash_set.contains(&v[i]) {
            continue;
        } else {
            hash_set.insert(v[i]);
            new_vec.push(v[i]);
        }
    }
    v.clear();
    for i in 0..new_vec.len() {
        v.push(new_vec[i]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
