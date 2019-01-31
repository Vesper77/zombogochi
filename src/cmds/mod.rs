mod status;
mod exit;

pub fn exec_cmd(cmd: &String, zombie: &crate::zombie::zombie::Zombie) {

  if status::check_cmd(&cmd) {
    status::exec_cmd(zombie);
  } else if exit::check_cmd(&cmd) {
    exit::exec_cmd();
  }

}