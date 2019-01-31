mod zombie;

fn main() {
    let zombie = zombie::Zombie {
        name: "Serega".to_string(),
        ..Default::default()
    };
    println!(
        "Here is the new zombie: {}, and does it dead? {}",
        zombie.name, zombie.is_dead
    );
}
