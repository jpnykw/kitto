#[derive(Debug)]
pub struct Blob (String);

pub fn encode(text: String) -> Option<Blob> {
  let length = text.chars().count();
  Some(Blob(String::from(format!("blob {}\0{}", length, text))))
}
