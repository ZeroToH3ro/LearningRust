fn main(){
    let s1 = String::from("hello");
    s2 = s1;
    let s2 = s1.clone(); // s2 will clone object s1

    println!("s1 = {}, s2 = {}", s1, s2);
}