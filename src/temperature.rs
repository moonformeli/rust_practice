use std::io;

pub fn run() {
  loop {
    println!("섭씨를 입력하세요");

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("다시 입력하세요");

    let c: u8 = match c.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("오류가 발생했습니다");
        continue;
      }
    };
    let f: u8 = (9 / 5) * c + 32;

    println!("섭씨 {}도 는 화씨 {}도 입니다", c, f);
  }
}
