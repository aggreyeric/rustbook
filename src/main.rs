mod c1;
mod c2;
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
    use crate::{c1, c2};

    #[test]
    fn it_works() {
        assert_eq!(c1::func_c1() ,3);
    }

    #[test]
    fn it_works2() {
        assert_eq!(c2::chapter2() ,3);
    }

}