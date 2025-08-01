use stalagmite_poly::zz_poly::ZZPoly;
use malachite::Integer;
use std::ops::Deref;

fn main() {
    // Deref means we can implicitly treat ZZPoly like an
    // (immutable) Vec<Integer>, so we get some interesting
    // functionality "for free".
    let a = ZZPoly::from(vec![1, 2, 3, -1, 0, 1]);
    let b = &a[0] + &a[1];
    println!("{:?}", b);
    
    let b = a.contains(&Integer::from(1));
    println!("{:?}", b);

    let res: Vec<_> = a.chunk_by(|x, y| *y == x + Integer::from(1)).collect();
    println!("{:?}", res);

    println!("{:?}", a.is_sorted());


    let p1 = ZZPoly::from([5, 1, -10, -1, 2, 1, -3]);
    let v: Vec<_> = p1.iter().rev().map(|x| x.clone()).collect();
    let mut p2 = ZZPoly::from(v);

    let p3 = stalagmite_poly::zz_poly::arithmetic::mul_ks::mul_ks(&p1, &p2);
    p2 *= &p1;
    println!("{:}", p1);
    println!("{:}", p2);
    println!("{:}", p3);
}
