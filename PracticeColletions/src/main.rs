use std::collections::HashMap;
use std::io;

fn main() {
    //find vector median an mode

    let mut Vector=vec![15,4,3,26,7,2,5,6,8,18];
    median(Vector);

    let mut Vector2=vec![1,3,2,4,5,6,7,8,9,5,4,6,3,2,1,7,9,3,3];
    mode(Vector2);
   
    //Pig Latin

    let Original_String=String::from("first");
    PigLatin(Original_String);
    text_interface();

}


fn median(mut Vector:Vec<isize>){   
    Vector.sort();
    println!("{Vector:?}");
    let n=Vector.len();


    let division2=n/2;

    if n%2==0{
        let median=(Vector[division2-1]+Vector[division2]) as f64/2.0;
        println!("The median of the given vector is: {median}")}
    else{
        let med=Vector[division2];
        println!("The median of the given vector is: {med}")
    };

}
fn mode(mut Vector:Vec<isize>){
     let mut hashmap=HashMap::new();
     for i in Vector{
        let count=hashmap.entry(i).or_insert(0);
        *count+=1;
     }
     let max_value=hashmap.values().max().cloned().unwrap_or(0);
     let mut Resultant_Vector=vec![];
     for (K,V) in &mut hashmap{
        if *V==max_value{
            Resultant_Vector.push(K);
        }
     }
     println!("The mode of given vector is :");
     println!("{Resultant_Vector:?}");
}

fn PigLatin(Text:String){
    let mut count=0;
    let Vowel:Vec<char>=vec!['a','e','i','o','u'];
  
    let len_string=Text.chars().count();
    println!("{len_string}");
    let mut resultant_string=String::new();
    let mut Special_String=String::new();
    
    for (i,character) in Text.chars().enumerate(){
        if i==0{
            if Vowel.contains(&character){
                  Special_String.push_str(&(String::from("-")+&character.to_string()+&"ay"));
                  count+=1;
                  continue;
            }
            else{
               
                 Special_String.push_str(&(String::from("-")+&"hay"));

            }
        }
        resultant_string.push_str(&character.to_string());
    }

    resultant_string.push_str(&Special_String);
    

    println!("The piglatin version of given input string is {resultant_string}");


}

fn text_interface(){
    let mut department_details:HashMap<String, Vec<String>>=HashMap::new();
    for i in 0..4{
         let mut user_input=String::new();
         io::stdin().read_line(&mut user_input).expect("not able to read the line");
        //  for word in user_input.split_whitespace(){
        //     user_input
        //  }
        let mut Vector:Vec<String>=Vec::new();
        for word in user_input.split_whitespace(){
            Vector.push(word.to_string());
        }
        let count:&mut Vec<String>=department_details.entry(Vector[3].clone()).or_insert(Vec::new());
        count.push(Vector[1].clone());
        
    }
    println!("{department_details:?}");
}
