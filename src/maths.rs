
use std::io;
use std::time;
use chrono::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

pub fn sequence_addition(max_num: i32) -> i32 {
    let now = time::Instant::now();
    let mut sum = 0;
    let mut n = 0;

    while n <= max_num{
        sum += n;
        n+=1;
    }
    let runtime = time::Instant::now() - now;
    println!("Runtime = {:?}", runtime);
    sum
}

pub fn sequence_product(max_num: i32) -> i32{
    let now = time::Instant::now();
    let mut sum = 1;
    let mut n = 1;

    while n <= max_num{
        sum *= n;
        n+=1;
    }
    let runtime = time::Instant::now() - now;
    println!("Runtime = {:?}", runtime);
    sum
}

pub fn odds_and_fives(max_num: i32) -> i32 {
    let now = time::Instant::now();
    let mut sum = 0;
    let mut n = 0;

    while n <= max_num{
        if n % 3 == 0{
            sum += n;
            n+=1;
        } else if n % 5 == 0{
            sum += n;
            n+=1;
        } else{
            n+=1;
        }
    }
    let runtime = time::Instant::now() - now;
    println!("Runtime = {:?}", runtime);
    sum
}

pub fn sum_or_product() -> i32 {
    let mut total = 0;
    let mut n = String::new();
    let mut n_num: i32 =  0;
    let mut p_or_s = String::new();
    
    println!("Give me a positive number");

     io::stdin()
        .read_line(&mut n)
        .expect("Failed to read name");
    let trimmed = n.trim();
    match trimmed.parse::<i32>(){
        Ok(i) => n_num = i,
        Err(..) => println!("This is not an integer"),
    }

    println!("Do you want the sequential sum or product?");
     io::stdin()
        .read_line(&mut p_or_s)
        .expect("Failed to read name");
    let now = time::Instant::now();
    match p_or_s.to_lowercase().trim(){
        "sum" => total = sequence_addition(n_num),
        "product" => total = sequence_product(n_num),
        _ => println!("Not a product or sum"),
    }
    let runtime = time::Instant::now() - now;
    println!("Runtime = {:?}", runtime);
    total
}

pub fn multiplication_table(max_num: u32){

    //let mut mult_table = vec![vec![0;max_num+1];max_num+1];
    let now = time::Instant::now();

    //let len = mult_table.len();

    for i in 0..max_num+1 {
        for j in 0..max_num+1 {
            print!("{} ",i*j);
        }
        println!("");
    }
    let runtime = time::Instant::now() - now;
    println!("Runtime = {:?}", runtime);
    //println!("{:?}", mult_table);

}

pub fn print_primes(max_num: i128){
    let mut i=1;
    let now = time::Instant::now();


    while i < max_num{
        if is_prime(i) == true {
             println!("{}", i);
             i+=1;
        }else{
             i+=1;
        }
    }

    let runtime = time::Instant::now() - now;
    println!("Runtime = {:?}", runtime);
}

fn is_prime(val: i128) -> bool{

    let mut i = 2;
    let mut return_bool = true;
     if val == 1 || val == 2 || val == 3 {
        return_bool = true;
     } else if val % 2 == 0 {
          return_bool = false;
     } else {
         while i < val {
             if val % i == 0 {
                 return_bool = false;
                 break;
             } else {
                 i += 1;
             }
         }
     }
    return_bool
}

pub fn print_leap_years(){
    let mut year = Local::now().year();
    
    let mut printed_years = 0;

    while printed_years < 20{
        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0{
                     println!("{}", year);
                    year += 1;
                    printed_years += 1;
                } else {
                    year +=1 ;
                }
            } else {
                println!("{}", year);
                year += 1;
                printed_years += 1;
            }
        } else {
            year += 1;
        }
    }
}

pub fn guessing_game(){
    let correct_value = rand::thread_rng().gen_range(1..1001);

    //println!("Correct value {}", correct_value);

    loop {
        println!("Please input guess for the random number. Type quit to end.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        if guess.trim() == "quit"{
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&correct_value){
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("Right on!");
                break;
            }
        }
    }
}