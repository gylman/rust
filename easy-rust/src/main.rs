extern crate cairo_felt;

fn main() {
    let a: cairo_felt::Felt252 = cairo_felt::felt_str!(
        "3618502788666131106986593281521497120414687020801267626233049500247285301247"
    );
    let b: cairo_felt::Felt252 = cairo_felt::felt_str!(
        "-3618502788666131213697322783095070105623107215331596699973092056135872020481"
    );
    let c = cairo_felt::felt_str!("-106710729501573572985208420194530329073740042555888586719234");
    let d = cairo_felt::felt_str!("1000");
    let e = cairo_felt::felt_str!("0");
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);
    println!("d: {:?}", e);
}
