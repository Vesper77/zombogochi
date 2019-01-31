pub struct StatDecription {
  name: String,
  descr: String,
}

pub struct StatType<T> {
  stat_descr: StatDecription,
  val: T,
}

pub trait StatToStringData {
  fn get_name(&self) -> String;
  fn get_description(&self) -> String;
  fn get_str_val(&self) -> String;
}

pub enum Stat {
  Int(StatType<i64>),
  Str(StatType<String>),
}

impl StatToStringData for StatType<i64> {
  fn get_name(&self) -> String {
    self.stat_descr.get_name()
  }
  fn get_description(&self) -> String {
    self.stat_descr.get_description()
  }
  fn get_str_val(&self) -> String {
    self.val.to_string()
  }
}

impl StatToStringData for StatType<String> {
  fn get_name(&self) -> String {
    self.stat_descr.get_name()
  }
  fn get_description(&self) -> String {
    self.stat_descr.get_description()
  }
  fn get_str_val(&self) -> String {
    self.val.clone()
  }
}

impl StatDecription {
  pub fn get_name(&self) -> String {
    return self.name.clone();
  }

  pub fn get_description(&self) -> String {
    return self.descr.clone();
  }
}

//'static - https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.htmls
pub fn get_default_stats() -> Vec<Stat> {
  let age_stat = StatType {
    val: 23,
    stat_descr: StatDecription {
      name: "Age".to_string(),
      descr: "Age of zombie".to_string(),
    }
  };

  let hunger_stat = StatType {
    val: 50,
    stat_descr: StatDecription {
      name: "Hunger".to_string(),
      descr: "Hunger of zombie".to_string()
    }
  };

  let str_stat = StatType {
    val: "Bolen".to_string(),
    stat_descr: StatDecription {
      name: "Status".to_string(),
      descr: "Status of zombie".to_string(),
    },
  };

  let mut stats_vec = Vec::new();

  stats_vec.push(Stat::Int(age_stat));
  stats_vec.push(Stat::Str(str_stat));
  stats_vec.push(Stat::Int(hunger_stat));

  return stats_vec;

}

pub fn get_stat_string_data(stat: &Stat) -> &StatToStringData {
  match stat {
    Stat::Int(val) => val,
    Stat::Str(val) => val,
  }
}

//&[Stat] - Slice (https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html)
pub fn print_stats(stats_slice: &[Stat]) {
  for stat in stats_slice {
    let stat_str_data = get_stat_string_data(stat);

    println!(
      "{} - {} - {}",
      stat_str_data.get_name(),
      stat_str_data.get_description(),
      stat_str_data.get_str_val()
    );
  }
}
