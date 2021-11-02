pub fn render_box<'a>(x: f64, y: f64, width: f64, height: f64) -> Vec<(f64, f64)> {
  let mut coords: Vec<(f64, f64)> = Vec::new();

  let mut x_point = x;
  

  loop {
    if x_point < x + width {
      let mut y_point = y;
      loop {
        if y_point < y + height {
          coords.push((x_point, y_point));
          y_point += 1.0 ;
        } else {
          break;
        }
      }
      x_point += 0.1 ;
    } else {
      break;
    }
    
  }
  
  coords
}