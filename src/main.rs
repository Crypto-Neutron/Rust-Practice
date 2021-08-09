mod elementary;
mod lists_strings;

fn main() {
    
    let v = vec![1,2,3,3,2,1];
    let w = &mut vec![100,200,300];

    let square_vec = lists_strings::lists::list_to_number(v);

    println!("vector: {:#?}", square_vec);

}

fn square(x: &f32) -> f32{
    let y = x * x;
    y 
}

fn quadratic(x: &f32) -> f32{
    let m = 1.254;
    let b = 12.;
    let y = m * x + b;
    y
}