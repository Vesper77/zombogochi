mod status;
mod exit;
mod eat;

pub fn exec_cmd(cmd: &String, zombie: &crate::zombie::zombie::Zombie) {

  if status::check_cmd(&cmd) {
    status::exec_cmd(zombie);
  } else if exit::check_cmd(&cmd) {
    exit::exec_cmd();
  } else if eat::check_cmd(&cmd) {
    eat::exec_cmd(&zombie.stats);
  }

}