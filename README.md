# Binary Heap
Toy project implementing a generic, binary min heap in Rust.

A simple usage example:

```rust
use heap::MinHeap;


fn main(){
    let values = vec![10, 2, 0, -3, 7, 41, 1];
    let mut heap = MinHeap::from_seq(&values);

    // get min
    println!("Get minimum and remove it: {:?}", heap.extract_min());
    println!("Get next minimum and keep it: {:?}", heap.min());

    // insert
    heap.insert(7);
    println!("Added a new element greater than the minimum. New minimum: {:?}", heap.min());
    heap.insert(-7);
    println!("Added a new element lower than the minimum. New minimum: {:?}", heap.min());

    // sort
    let sorted = MinHeap::sort(values);
    println!("Just use the heap to sort the elements: {:?}", sorted);
}

```
