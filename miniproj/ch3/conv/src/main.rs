fn to_f(t: f32) -> f32 {
  let temp: f32;
  temp = t * 1.8 + 32.0;
  temp
}

fn to_c(t: f32) -> f32 {
  let temp: f32;
  temp = (t - 32.0) / 1.8;
  temp
}

fn main() {
  println!("Omg hay");
  let f_temp = to_f(16.8);
  let c_temp = to_c(62.24);
  println!("The temperatures are: {:.2}C, {:.2}F", c_temp, f_temp);
}