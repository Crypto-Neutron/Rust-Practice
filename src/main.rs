mod elementary;
mod lists_strings;

fn main() {
    
    let v = &mut vec![1,2,3];
    let w = &mut vec![100,200,300];

    let square_vec = lists_strings::lists::concatinate_lists(w, v);

    println!("vector: {:#?}", v);
    println!("vector concatinated: {:#?} ", square_vec);

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