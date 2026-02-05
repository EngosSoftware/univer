mod test_cli;
mod test_develop;
mod test_publish;

#[cfg(not(target_os = "windows"))]
fn normalize(s: &str) -> String {
  s.to_string()
}

#[cfg(target_os = "windows")]
fn normalize(s: &str) -> String {
  s.replace("\n", "\r\n")
}

#[cfg(not(target_os = "windows"))]
fn normalize_exe(s: &str) -> String {
  s.replace("||E||", "")
}

#[cfg(target_os = "windows")]
fn normalize_exe(s: &str) -> String {
  s.replace("||E||", ".exe")
}

#[cfg(not(target_os = "windows"))]
fn normalize_sep(s: &str) -> String {
  s.replace("||S||", "/")
}

#[cfg(target_os = "windows")]
fn normalize_sep(s: &str) -> String {
  s.replace("||S||", "\\")
}
