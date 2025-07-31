use stalagmite_poly2::intpoly::IntPoly;

fn main() {
    let polys = vec![
        IntPoly::from([1, 0]),
        IntPoly::from(vec![0, 1]),
        IntPoly::from(&[2, 3]),
    ];
    let mut result: IntPoly = IntPoly::zero();
    result += &polys[0];
    println!("{:?}", result);
    result += &polys[1];
    println!("{:?}", result);
    result += &polys[2];
    println!("{:?}", result);

    let a = IntPoly::from(vec![1, 2, -10]);
    let b = IntPoly::from(vec![4, 5, 6, 7, -4]);
    let c = a + &b;
    println!("{:?}", c);
    assert_eq!(c, IntPoly::from(vec![5, 7, -4, 7, -4]));

    // let result: IntPoly = polys.iter().sum();
    // println!("{:?}", result);
    // assert_eq!(result, IntPoly::from(vec![3, 4]));
}