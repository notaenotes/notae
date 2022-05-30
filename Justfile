configuration_dir_path := if os() == "macos" {
  "~/Library/Application\\ Support/notae"
} else {
"oi"
}

default:
  @just --list
init:
  rm -rf {{configuration_dir_path}} 
