use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let now = Instant::now();
    //let card_pub : u64 = 5764801;
    //let door_pub : u64 = 17807724;
    let card_pub : u64 = 19774466;
    let door_pub : u64 = 7290641;

    let mut card_loop_size = 0;
    let mut door_loop_size = 0;
    

    let mut value = 1;
    let mut i = 0;
    loop {
        value *= 7;
        value %= 20201227;
        i += 1;

        if value == card_pub && card_loop_size == 0{
            card_loop_size = i;
        } else if value == door_pub && door_loop_size == 0{
            door_loop_size = i;
        }

        if card_loop_size > 0 && door_loop_size > 0 {
            break;
        }
    }

    let mut value = 1;
    for _i in 0..card_loop_size {
        value *= door_pub;
        value %= 20201227;
    }
    println!("Part1 {}", value);

    println!("{}", now.elapsed().as_millis());
    Ok(())
}