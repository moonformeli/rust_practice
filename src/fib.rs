pub fn run() {
  let mut f = Vec::new();
  f.push(1);
  f.push(1);

  for i in 2..=10 {
    f.push(f[i - 2] + f[i - 1]);
  }

  for i in f.iter() {
    println!("{}", i);
  }
}
