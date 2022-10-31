fn main() {
    let data : Vec<_> = vec![12,2,3,12,5,10]
                        .iter()
                        .map(|number| number+2)
                        .filter(|number| number > &10)
                        .collect();
    for num in data { 
        println!("{:?}", num);
    }

}