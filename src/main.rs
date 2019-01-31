mod zombie;
mod cmds;

use zombie::zombie as zombie_mod;

use std::io;

fn main() {

    let zombie = zombie_mod::Zombie {
        name: "Serega".to_string(),
        ..Default::default()
    };

    zombie_mod::print_zombie(&zombie);

    loop {
        
        let mut input = String::new();

        let result = io::stdin().read_line(&mut input);

        let trim_str = input.trim().to_string();

        match result {
            Ok(_) => cmds::exec_cmd(&trim_str, &zombie),
            Err(err) => println!("ERROROROROROROROOR - {}", err)
        }

    }

}
