mod zombie;

fn main() {
    let zombie = zombie::Zombie{ name: "Serega".to_string() };
    println!("Here is the new zombie: {}", zombie.name);
}
