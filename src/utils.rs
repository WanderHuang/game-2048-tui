/// predict if two matrices contains same value
pub fn equal_slice<T>(a: &[T], b: &[T]) -> bool where T: Eq {
  if a.len() != b.len() {
    return false;
  } else {
    for (i, x) in a.iter().enumerate() {
      if *x != b[i] {
        return false;
      }
    }
  }

  true
  
}