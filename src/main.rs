use rand::Rng;
use std::io;

fn main() {
  println!("ben picks up da fone\ntell him something");
  loop {
    read_user();
    handle_response(gen_response());
  }
}

fn read_user() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("read line fail");
}

fn handle_response(res: Response) {
    match res {
      Response::Yes => println!("Yes"),
      Response::No => println!("No"),
      Response::Laugh => println!("* ben laughs * "),
      Response::Bleurgh => println!("* ben throws up *"),
      Response::Slam => {
        println!("* ben slams the phone down *");
        println!("* you pick the phone back up *")
      }
    };
}

fn gen_response() -> Response {
  let mut rng = rand::thread_rng();
  match rng.gen_range(0..5) {
    0=> Response::Yes,
    1=> Response::No,
    2=> Response::Laugh,
    3=> Response::Bleurgh,
    4=> Response::Slam,
    _ => panic!("Not supposed to happen"),
  }
}

enum Response {
  Yes,
  No,
  Laugh,
  Bleurgh,
  Slam,
}