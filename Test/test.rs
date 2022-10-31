fn main () {

}

mod test {
    use crate::*;

    #[test]
    fn check() {
        let result = caps("trinh ngoc minh");
        let expect = String::from("TRINH NGOC MINH");
        assert_eq!(result, expect, "String should be all uppercase");
    }
}