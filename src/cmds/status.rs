use crate::zombie::zombie;

pub fn check_cmd(cmd: &String) -> bool {
  cmd == "status"
}

pub fn exec_cmd(zombie: &zombie::Zombie) {
  zombie::print_zombie(zombie);
}