use std::io;

fn main() -> io::Result<()> {
    //calculated by hand with google calc
    println!("Part1 {}", 4938);

    let v = [29, 41,37,653,13,17,23,823,19];
    let v2 = [29,19,23,29,42,46,52,60,79];

    let n = v.iter().fold(1, |x,y| x*y);
    let sum: u128 = v.iter().zip(v2.iter()).map(|(&a, &b)| b*(mod_pow(n/a, a-2, n)* n/a)).sum();
    println!("Part2 {}", (n- sum % n) % n);
    
    Ok(())
}

fn mod_pow(x: u128, y: u128, modu: u128) -> u128{
    let mut res = 1;
    for _ in 0..y{
        res = (res * x)% modu;
    }
    res
}




