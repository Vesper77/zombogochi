pub const TYPE_HUNGER: &'static str = "HUNGER";
pub const TYPE_AGE: &'static str = "HUNGER";
pub const TYPE_STATUS: &'static str = "HUNGER";

pub trait StatToStringData {
  fn get_name(&self) -> String;
  fn get_description(&self) -> String;
  fn get_str_val(&self) -> String;
  fn get_type(&self) -> String;
}

pub trait StatGet {
  fn get_stat_by_type(&self, stat_type: String) -> Option<&Stat>;
}

pub trait StatHelp {
  fn get_stat_string_data(&self) -> &StatToStringData;
}

pub struct StatDecription {
  name: String,
  descr: String,
}

pub struct StatType<T> {
  stat_descr: StatDecription,
  stat_type: String,
  pub val: T // TODO: male private and add set_val, get_val
}

pub enum Stat {
  Int(StatType<i64>),
  Str(StatType<String>),
}

impl StatGet for Vec<Stat> {
  //Option - https://doc.rust-lang.org/std/option/index.html
  fn get_stat_by_type(&self, stat_type: String) -> Option<&Stat> {

    let stats_slice = self.as_slice();

    for stat in stats_slice {
      
      let stat_string_data = (stat as &StatHelp).get_stat_string_data();
      
      if stat_string_data.get_type() == stat_type {
        return Some(stat);
      }

    }

    return None;

  }

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
  fn get_type(&self) -> String {
    self.stat_type.clone()
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
  fn get_type(&self) -> String {
    self.stat_type.clone()
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

impl StatHelp for Stat {
  fn get_stat_string_data(&self) -> &StatToStringData {
    match (self) {
      Stat::Int(val) => val,
      Stat::Str(val) => val,
    }
  }
}

//TODO: Return value be reference? Set value by referene?
//TODO: Uncoment
// impl <T> StatType<T> {

//   pub fn get_val(&self) -> T {
//     return self.val;
//   }
  
//   pub fn set_val(mut self, val: T) -> T {
//     self.val = val;
//     return self.val;
//   }

// }

pub fn get_default_stats() -> Vec<Stat> {
  let age_stat = StatType {
    val: 23,
    stat_type: TYPE_AGE.to_string(),
    stat_descr: StatDecription {
      name: "Age".to_string(),
      descr: "Age of zombie".to_string(),
    },
  };

  let hunger_stat = StatType {
    val: 50,
    stat_type: TYPE_HUNGER.to_string(),
    stat_descr: StatDecription {
      name: "Hunger".to_string(),
      descr: "Hunger of zombie".to_string(),
    },
  };

  let str_stat = StatType {
    val: "Bolen".to_string(),
    stat_type: TYPE_STATUS.to_string(),
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

//&[Stat] - Slice (https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html)
pub fn print_stats(stats_slice: &[Stat]) {
  for stat in stats_slice {
    let stat_str_data = (stat as &StatHelp).get_stat_string_data();

    println!(
      "{} - {} - {}",
      stat_str_data.get_name(),
      stat_str_data.get_description(),
      stat_str_data.get_str_val()
    );
  }
}
