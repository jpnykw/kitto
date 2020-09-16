mod blob;
mod zlib;
mod ext;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use sha1::{Sha1, Digest};
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "jpnykw <jpnykw.com>")]
struct Opts {
  #[clap(subcommand)]
  subcmd: Option<SubCommand>,
}

#[derive(Clap, Debug)]
enum SubCommand {
  #[clap(version = "0.0.1")]
  Add(Add),
  #[clap(version = "0.0.1")]
  Cat_File(Cat_File)
}

#[derive(Clap, Debug)]
struct Add {
  #[clap(required = true)]
  path: String,
}

#[derive(Clap, Debug)]
struct Cat_File {
  #[clap(long)]
  p: Option<bool>, // -p 調べる
  #[clap(required = true)]
  path: String,
}

fn sha1(content: &String) -> String {
  let mut sha1 = Sha1::new();
  sha1.update(content.as_bytes());
  ext::join_iter(sha1.finalize().iter())
}

fn main() {
  let opts: Opts = Opts::parse();

  match opts.subcmd {
    Some(command) => {
      // 指定されたから分岐するよ
      match command {
        SubCommand::Add(args) => {
          // 指定したパスのファイルを読むよ
          let mut file = File::open(args.path).expect("Invalid path");
          let mut contents = String::new();
          file.read_to_string(&mut contents)
          // 読み込みに失敗したよ
          .expect("Something went wrong reading the file");

          // 中身をblob objectに変換
          let blob_object = blob::encode(String::from(contents));
          println!("object {:?}", blob_object);

          match blob_object.expect("Failed to unwrap blob object") {
            blob::Blob(object) => {
              let bytes = object.as_bytes();

              // SHA-1を通してIDを生成
              let id = sha1(&object);
              println!("blob {}", id);

              // objectを圧縮して
              let object = zlib::compress(object);
              println!("object {}", object);

              // (確認のため) 展開してみる
              zlib::decompress(object);
            }
          }
        },
        SubCommand::Cat_File(args) => {
          println!("cat args -> {:?}", args);
        },
        _ => {
          // 存在しないよ
          println!("Unknown subcommand.");
        }
      };
    },
    None => {
      // 何も指定されなかったよ
      println!("Select subcommand.");
    }
  };

  /*

  // TODO: 引数の処理をclapで書き換える
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 { panic!("Select mode"); }
  // println!("{:?}", args);

  let mode: &str = &args[1];
  println!("git {}", mode);

  match mode {
    "cat-file" => {
      if args.len() < 3 { panic!("Add object"); }
      // オブジェクトを受け取って
      let object = &args[2];
      println!("object {}", object);

      // 展開する
      let unzip_object = zlib::decompress(object.to_string());
      println!("object {}", unzip_object);
    }

    _ => {
      println!("you can learn about how to use with help")
    }
  }

  */
}

#[test]
fn sha1_case_a() {
  assert_eq!(
    sha1(&String::from("a")),
    String::from("86f7e437faa5a7fce15d1ddcb9eaeaea377667b8")
  );
}

#[test]
fn sha1_case_b() {
  assert_eq!(
    sha1(&String::from("b")),
    String::from("e9d71f5ee7c92d6dc9e92ffdad17b8bd49418f98")
  );
}

#[test]
fn sha1_case_c() {
  assert_eq!(
    sha1(&String::from("c")),
    String::from("84a516841ba77a5b4648de2cd0dfcb30ea46dbb4")
  );
}

#[test]
fn sha1_case_git() {
  assert_eq!(
    sha1(&String::from("git")),
    String::from("46f1a0bd5592a2f9244ca321b129902a06b53e03")
  );
}

