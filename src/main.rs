mod blob;
mod zlib;
mod ext;

use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
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
  Init(Init),
  #[clap(version = "0.0.1")]
  Add(Add),
  #[clap(version = "0.0.1")]
  Cat_File(Cat_File)
}

#[derive(Clap, Debug)]
struct Init {}

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
        SubCommand::Init(_) => {
          println!("init");
          // .git の代わりに .kitto を作成
          match fs::create_dir(".kitto") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
          };
          // 中身は objects, refs/heads, refs/tags
          match fs::create_dir(".kitto/objects") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
          };
          match fs::create_dir(".kitto/refs") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
          };
          match fs::create_dir(".kitto/refs/heads") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
          };
          match fs::create_dir(".kitto/refs/tags") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
          };
        },
        SubCommand::Add(args) => {
          println!("add");
          // 指定したパスのファイルを読むよ
          let mut file = File::open(args.path).expect("Invalid path");
          let mut contents = String::new();
          file.read_to_string(&mut contents)
          // 読み込みに失敗したよ
          .expect("Something went wrong reading the file");

          // 中身を blob object に変換
          let blob_object = blob::encode(&String::from(contents));
          // println!("object {:?}", blob_object);

          match blob_object.expect("Failed to unwrap blob object") {
            blob::Blob(object) => {
              let bytes = object.as_bytes();

              // SHA-1 を通して ID を生成
              let id = sha1(&object);
              println!("→ {:<8} {}", "ID", id);

              // object 圧縮して
              let object = zlib::compress(object);
              println!("→ {:<8} {}", "Content", object);

              // (確認のため) 展開してみる
              zlib::decompress(object);
            }
          }
        },
        SubCommand::Cat_File(args) => {
          // object 受け取って
          let object = args.path;
          println!("object {}", object);

          // 展開する
          let unzip_object = zlib::decompress(object.to_string());
          println!("object {}", unzip_object);
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
}

#[test]
fn sha1_case_1() {
  assert_eq!(
    sha1(&String::from("a")),
    String::from("86f7e437faa5a7fce15d1ddcb9eaeaea377667b8"),
  );
}

#[test]
fn sha1_case_2() {
  assert_eq!(
    sha1(&String::from("b")),
    String::from("e9d71f5ee7c92d6dc9e92ffdad17b8bd49418f98"),
  );
}

#[test]
fn sha1_case_3() {
  assert_eq!(
    sha1(&String::from("git")),
    String::from("46f1a0bd5592a2f9244ca321b129902a06b53e03"),
  );
}

#[test]
fn blob_encode_1() {
  assert_eq!(
    blob::encode(&String::from("Hello World")).unwrap(),
    blob::Blob(String::from("blob 11\0Hello World")),
  );
}

#[test]
fn blob_encode_2() {
  assert_eq!(
    blob::encode(&String::from("Hello\nWorld")).unwrap(),
    blob::Blob(String::from("blob 11\0Hello\nWorld")),
  );
}

#[test]
fn blob_encode_3() {
  assert_eq!(
    blob::encode(&String::from("Nyanko\0World")).unwrap(),
    blob::Blob(String::from("blob 12\0Nyanko\0World")),
  );
}

#[test]
fn blob_decode_1() {
  assert_eq!(
    blob::decode(blob::Blob(String::from("blob 11\0Hello World"))),
    Some(String::from("Hello World"))
  );
}

#[test]
fn blob_decode_2() {
  assert_eq!(
    blob::decode(blob::Blob(String::from("blob 11\0Hello\nWorld"))),
    Some(String::from("Hello\nWorld"))
  );
}

#[test]
fn blob_decode_3() {
  assert_eq!(
    blob::decode(blob::Blob(String::from("blob 11\0Hello\0World"))),
    Some(String::from("Hello\0World"))
  );
}

#[test]
fn blob_bridge_1() {
  let data = String::from("Nyanko Ippai");
  let encode = blob::encode(&data).unwrap();
  let decode = blob::decode(encode).unwrap();
  assert_eq!(&data, &decode);
}

#[test]
fn blob_bridge_2() {
  let data = String::from("Nyanko\nIppai");
  let encode = blob::encode(&data).unwrap();
  let decode = blob::decode(encode).unwrap();
  assert_eq!(&data, &decode);
}

#[test]
fn blob_bridge_3() {
  let data = String::from("Nyanko\0Ippai");
  let encode = blob::encode(&data).unwrap();
  let decode = blob::decode(encode).unwrap();
  assert_eq!(&data, &decode);
}

