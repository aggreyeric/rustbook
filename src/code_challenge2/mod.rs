pub fn unique(mut a: Vec<i32>) -> Vec<i32> {
    //unique() is a function that takes a vector of floating point numbers (Vec<f32>) and returns a list that only contains unique values.
   
    a.sort();
    a.dedup();
    a
  }
  