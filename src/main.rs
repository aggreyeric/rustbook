mod c1;
mod c2;
mod code_challenge1;
mod code_challenge2;
// mod c3;


fn main() {


let mut  x = 5;



{
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
}

println!("The value of x is: {x}");


}


#[cfg(test)]
mod tests {
    use crate::{c1, c2, code_challenge1,code_challenge2};

    #[test]
    fn it_works() {
        assert_eq!(c1::func_c1() ,3);
    }

    #[test]
    fn empty_list1(){
        let input: Vec<f32> = vec![];
    
        let expected_output:Option<f32> = None;
        let actual_output:Option<f32> = code_challenge1::median(input);
    
        assert_eq!(expected_output, actual_output);
    }

    #[test]
fn even_lenght(){
    let input: Vec<f32> = vec![1.0, 2.0,2.0, 3.0];

    let expected_output:Option<f32> = Some(2.0);
    let actual_output:Option<f32> = code_challenge1::median(input);

    assert_eq!(expected_output, actual_output);
}

#[test]

fn sorted_list1(){
    let input: Vec<f32> = vec![1.0, 2.0,3.0,4.0,5.0];

    let expected_output:Option<f32> = Some(3.0);
    let actual_output:Option<f32> = code_challenge1::median(input);

    assert_eq!(expected_output, actual_output);
}


#[test]
fn unsorted_list1() {
    let input = vec![3.0, 5.0, 2.0];
    let expected_output = Some(3.0);
    let actual_output = code_challenge1::median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn empty_list2() {
  //  let input: Vec<i64> = vec![];
    let input = vec![];
    let expected_output = vec![];
    let actual_output = code_challenge2::unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list2() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = code_challenge2::unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list2() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = code_challenge2::unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = code_challenge2::unique(input);
    assert_eq!(actual_output, expected_output);
}

    // #[test]
    // fn it_works2() {
    //     assert_eq!(c2::chapter2() ,3);
    // }

}