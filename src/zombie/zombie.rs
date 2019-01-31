pub struct Zombie {
  pub name: String,
  pub is_dead: bool,
}

impl Default for Zombie {
  fn default() -> Zombie {
    Zombie {
      name: "Unknown".to_string(),
      is_dead: false,
    }
  }
}
