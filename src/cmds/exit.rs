pub fn check_cmd(cmd: &String) -> bool {
  cmd == "exit"
}

pub fn exec_cmd() {
  std::process::exit(0)
}