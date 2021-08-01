use std::ops::Add;

pub fn largest_elem(full_list: Vec<i32>)->i32{
    let mut largest = full_list[0];

    for i in full_list{
        if i > largest{
            largest = i;
        }
    }
    largest
}

pub fn reverse_list<T: Copy>(list: &mut Vec<T>){

    let len = list.len();

    for i in 0..len/2 -1 {
        let place_holder = list[i];

        list[i] = list[len-i-1];
        list[len-i-1] = place_holder
    }
}

pub fn occur_in_list<T: PartialEq>(value: &T ,list: &Vec<T>) -> bool{
    

    if list.iter().any(|x| x == value){
        return true;
    } else {
        return false;
    }
    
}

pub fn return_odds<T>(list: &[T]) -> Vec<&T>{

    let mut i = 0;
    let mut odd_vec = Vec::new();

    while i < list.len() {
        odd_vec.push(&list[i]);
        i += 2;
    }

    odd_vec
}

pub fn vec_sum(list: &Vec<f32>) -> f32{

    let mut sum = 0.;
    let mut i = 0;

    while i <= list.len()-1{
        sum += list[i];
        i += 1;
    }

    sum

}

pub fn vec_sum_recursive<T: Add<Output = T> + Copy + Default>(list: &[T]) -> T{

    if list.len() == 1{
        list[0]
    } else {
        list[0] + vec_sum_recursive(&list[1..])
    }

}

pub fn to_all<T>(f: fn(x: &T)->T, list: &Vec<T>) -> Vec<T>{

    let mut i = 0;
    let mut final_vec = Vec::new();

    while i <= list.len()-1{
        final_vec.push(f(&list[i]));
        i +=1;

    }
    final_vec
}


pub fn concatinate_lists<T>(list1: &mut Vec<T>, list2: &mut Vec<T>) -> Vec<T>{

    let mut final_vec = Vec::new();

    final_vec.append(list1);
    final_vec.append(list2);

    final_vec
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