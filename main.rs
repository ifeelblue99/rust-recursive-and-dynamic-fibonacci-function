fn main() {
  println!("fib recursive. {}", fib_recursive(7));
  println!("fib dynamic. {}",fib_dynamic(7));
}


fn fib_recursive(step: i8) -> i8 {
  if step <= 2 {
    return 1
  }
  fib_recursive(step-1)+fib_recursive(step-2)
}


fn fib_dynamic(step: i8) -> i8 {
  let step_usize = step as usize;
  let mut arr = vec![1;step_usize];

  if step > 2 {
    let mut indx:usize = 2;
    while indx < step_usize {
      arr[indx] = arr[indx-1]+arr[indx-2];
      indx = indx+1;
    }
  }
  arr[step_usize-1]
}
