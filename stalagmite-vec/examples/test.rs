use stalagmite_vec::ZZVector;

pub fn main() {
    let inp = vec![0,2,4,6,1,3,5];
    let v = ZZVector::new(inp);
    println!("{}", v);
}
