impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut num = x.abs();
    let mut reverse_num = 0;
    while num > 0 {
      let digit = num % 10;
      num /= 10;
      if reverse_num > i32::MAX / 10 {
        return 0;
      }
      reverse_num = reverse_num * 10 + digit;
    }
    return match x < 0 {
      true => -reverse_num,
      false => reverse_num,
    };
  }
}
