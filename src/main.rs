
fn add_kill(kills: i32) -> i32 {
    kills + 1
}

fn main() {
    let kills = 0;
    let updated_kills = add_kill(kills);

    println!("Kills: {updated_kills}");
}

