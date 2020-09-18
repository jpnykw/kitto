#[derive(Debug, PartialEq)]
pub struct Blob(pub String);

pub fn encode(text: &String) -> Option<Blob> {
  let length = text.chars().count();
  Some(Blob(String::from(format!("blob {}\0{}", length, text))))
}

pub fn decode(object: Blob) -> Option<String> {
  match object {
    Blob(text) => {
      // ヌル文字で区切って
      let mut null_split = text.split("\0").collect::<Vec<&str>>();
      // ヘッダー部分を除去
      null_split.remove(0);
      // コンテンツにヌル文字が含まれていることを考慮して
      // 後ろの配列を結合して文字列に変換
      Some(null_split.join("\0"))
    }
  }
}

