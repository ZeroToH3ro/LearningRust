fn main () {
    let vec = vec![1, 2, 3, 4];

    let plus_one: Vec<_> = vec.iter().map(|num| num + 1).collect();
    println!("{:?}", plus_one);
}
