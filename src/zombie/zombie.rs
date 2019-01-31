use super::stat::Stat;

pub struct Zombie {
  pub name: String,
  pub is_dead: bool,
  pub stats: [Stat; 2]
}

impl Default for Zombie {

  fn default() -> Self {

    return Zombie {
      name: "test".to_string(),
      is_dead: false,
      stats: super::stat::get_default_stats()
     }
  }

}