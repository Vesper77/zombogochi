use crate::zombie::stat as stat_mod;


pub fn check_cmd(cmd: &String) -> bool {
  cmd == "eat"
}

pub fn exec_cmd(stats_get: &stat_mod::StatGet) {
  
  // let stat_result = stats_get.get_stat_by_type(stat_mod::TYPE_HUNGER.to_string());

  println!("EAT!");

  //TODO: Merge into one condition, uncomment
  //Some(Stat)
  // if let Some(stat) = stat_result {

  //   if let stat_mod::Stat::Int(mut stat_val) = stat {

  //     stat_val.val += 5;

  //   }

  // }



}