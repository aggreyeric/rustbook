mod c1;
mod c2;
mod code_challenge1;
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
    use crate::{c1, c2, code_challenge1};

    #[test]
    fn it_works() {
        assert_eq!(c1::func_c1() ,3);
    }

    #[test]
    fn empty_list(){
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

fn sorted_list(){
    let input: Vec<f32> = vec![1.0, 2.0,3.0,4.0,5.0];

    let expected_output:Option<f32> = Some(3.0);
    let actual_output:Option<f32> = code_challenge1::median(input);

    assert_eq!(expected_output, actual_output);
}


#[test]
fn unsorted_list() {
    let input = vec![3.0, 5.0, 2.0];
    let expected_output = Some(3.0);
    let actual_output = code_challenge1::median(input);
    assert_eq!(actual_output, expected_output);
}
    // #[test]
    // fn it_works2() {
    //     assert_eq!(c2::chapter2() ,3);
    // }

}