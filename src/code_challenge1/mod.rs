pub fn median(mut a: Vec<f32>) -> Option<f32>{
//   median()  is a function, that takes a vector of floating point numbers (Vec<f32>) and returns the median as a floating point number.

// median() returns a value  wrapped in an Option type to account for cases where the input list is empty. When that occurs, there is no meaningful median.
    
    // - empty
    // - odd number of elements in the list
    // - even number of elements in the list

if a.is_empty() {
    return None;
}


a.sort_by(|x: &f32, y: &f32| x.partial_cmp(y).unwrap());
// TODO: sort

let n_elements: usize = a.len();
let middle: usize = n_elements /2;

let med: f32 = if n_elements % 2 == 0{
 //even
    (a[middle - 1] + a[middle]) / 2.0
}else{
    //odd
    a[middle] 
};

Some(med)

}