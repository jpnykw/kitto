use std::slice::Iter;

pub fn join_iter(iter: Iter<'_, u8>) -> String {
  iter.map(|&byte| format!("{:<02x}", byte)).collect::<String>()
}

