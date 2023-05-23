use linked_list::LinkedList;
pub mod linked_list;
use linked_list::ComputeNorm;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    let mut list1: LinkedList<f64> = LinkedList::new();
    assert!(list1.is_empty());
    assert_eq!(list1.get_size(), 0);
    let vec_str = vec![1.0, 2.0, 3.0];
    for s in vec_str {
        list1.push_front(s.clone());
    }
    println!("list1 = {}", list1);

    // test Clone
    let list_2 = list1.clone();

    // test PartialEq
    println!("list1 == list_2: {}", list1 == list_2);

    // test ComputeNorm
    println!("compute_norm(list1) = {}", list1.compute_norm());

    // test Iterator and IntoIterator
    for val in &list1 {
        println!("{}", val);
    }

    for val in list1 {
        println!("{}", val);
    }
}
