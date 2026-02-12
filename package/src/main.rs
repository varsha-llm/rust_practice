use std::collections::HashMap;
pub mod garden;

use crate::garden::vegetable::Asparagus;
fn main() {
    println!("Hello, world!");
    println!("{Asparagus}");

    //Collection datatype 
    let Vector:Vec<i32>=Vec::new();
    let Vector=vec![1,2,3,2,43];
    println!("{Vector:?}");

    //accessing the vector element
    let n:i32=12;
    let access_get:Option<&i32>=Vector.get(n as usize);
    match access_get{
        Some(access_get)=>println!("The element at index {} is {}",n,access_get),
        None=>println!("The index is out of range"),

    }

    //let access_index:&i32=&Vector[n as usize];

    //dereference to make changes on the existing list

    let mut Vector2=[3,4,5,4,5,3,6];

    for i in & mut Vector2{
        *i+=100;
    }
    println!("{Vector2:?}");

    //format!

    let s1=String::from("one");
    let s2=String::from("two");
    let s3=String::from("three");
    let formatted=format!("{s1}-{s2}-{s3}");
    println!("{formatted}");

    //Creating HashMap

    println!("HashMap");

    let mut a =HashMap::new();
    a.insert(String::from("one"),1);
    a.insert(String::from("two"),2);
    a.insert(String::from("three"),3);

    let b_three=a.get(&String::from("three")).copied().unwrap_or(0);
    println!("the hashmap value for the key three is {b_three}");

    //Overwriting a HashMap
    a.insert(String::from("three"),13);

    let b_three=a.get(&String::from("three")).copied().unwrap_or(0);
    println!("the hashmap value for the key three is {b_three}");
    //Iterate over HashMap

    for (key,value) in &a{
        println!("{key}, {value}");
    }


    //Ownership_hashmap();

}

    //Ownership in HashMap

    fn Ownership_hashmap(){
        let mut  c=HashMap::new();
        let first=String::from("rank1");
        c.insert(first,String::from("Gold"));
        c.insert(String::from("rank2"),String::from("Silver"));
        c.insert(String::from("rank3"),String::from("Bronze"));
       // println!("the value of variable first is {first}");
    }
