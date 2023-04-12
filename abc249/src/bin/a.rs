use proconio::input;

fn main() {
    input! {
        a: usize, b:usize,c:usize,d:usize,e:usize,f:usize,x:usize,
    }

    let t1cycle = (a + c) * a;
    let tamari = x % (a + c);
    let mut thasuu = 0;
    if tamari <= a {
        thasuu = tamari;
    } else {
        thasuu = a;
    }

    let a1cycle = (d + f) * d;
    let aamari = x % (d + f);
    let mut ahasuu = 0;
    if aamari <= d {
        ahasuu = aamari;
    } else {
        ahasuu = d;
    }

    let tdistance = ((x / t1cycle) + thasuu) * b;
    let adistance = ((x / a1cycle) + ahasuu) * e;

    if tdistance > adistance {
        println!("Takahashi");
    } else if tdistance < adistance {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
