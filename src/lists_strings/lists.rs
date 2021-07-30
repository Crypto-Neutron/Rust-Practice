pub fn largest_elem(full_list: Vec<i32>)->i32{
    let mut largest = full_list[0];

    for i in full_list{
        if i > largest{
            largest = i;
        }
    }
    largest
}

pub fn reverse_list(list: &mut Vec<i32>){

    let mut place_holder: i32;
    let len = list.len();

    for i in 0..len/2 -1 {
        place_holder = list[i];

        list[i] = list[len-i-1];
        list[len-i-1] = place_holder
    }
     println!("Reverse vector: {:?}", list);
}