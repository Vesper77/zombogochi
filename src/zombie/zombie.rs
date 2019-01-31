use super::stat;

pub struct Zombie {
  pub name: String,
  pub is_dead: bool,
  pub stats: Vec<stat::Stat>
}
//'static - https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.htmls

impl Default for Zombie {

  fn default() -> Self {

    return Zombie {
      name: "test".to_string(),
      is_dead: false,
      stats: stat::get_default_stats()
     }
  }

}

pub fn print_zombie(zombie: &Zombie) {

  println!("-----------------");
  println!(
    "Here is the new zombie: {}, and does it dead? {}",
    zombie.name, zombie.is_dead
  );
  println!("-----------------");

  stat::print_stats(&zombie.stats);

}