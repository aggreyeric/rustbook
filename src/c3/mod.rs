
pub const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;



pub fn var_mut_notwork() {
    let x = 5;
    x = 6;
    println!("x = {}", x);
}

pub fn var_mut_work() {
    let mut x = 5;
    x = 6;
    println!("x = {}", x);
}


pub fn shadoow() {
    let  x = 5;

    //  x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}