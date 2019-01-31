mod zombie;

use zombie::zombie::Zombie;
use zombie::stat;

fn main() {

    let zombie = Zombie {
        name: "Serega".to_string(),
        ..Default::default()
    };

    println!("--------------------");

    println!(
        "Here is the new zombie: {}, and does it dead? {}",
        zombie.name, zombie.is_dead
    );
    
    println!("--------------------");
    stat::print_stats(&zombie.stats);
    println!("--------------------");


}
