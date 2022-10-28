fn main() {
    //Accessing and changing existing data
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;
    //c[6] = 2; Cannot assign values this way, index out of bounds
    println!("{:?}", c); //[1, 2, 3, 2, 1]

    //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.pop(); //[1] : : Remove an element from the end

    // 🔎 Capacity and reallocation
    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

    // These are all done without reallocating...
    for i in 0..10 {
        e.push(i);
    }
    // ...but this may make the vector reallocate
    e.push(11);
}
