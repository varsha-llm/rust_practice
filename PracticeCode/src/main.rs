// #[derive(Debug)]
  //enumerator
  //The message processor
enum Message{
        Quit,
        Move{
            x:i32,
            y:i32
        },
        Write(String),
        ChangeColor(i32,i32,i32)}

fn process_message(n:Message){
    match n{
        Message::Quit=>println!("System Shutting down..."),
        Message::Move{x,y}=>println!("Moving to x:[{x}], y:[{y}]"),
        Message::Write(text)=>println!("Text received: {text}"),
        Message::ChangeColor(r,g,b)=>println!("Color changed to RGB([{r}],[{g}],[{b}])")

    }
        
    }
fn main() {
    //println!("Hello, world!");
    //Iterating over the array
    let iterator=[1,2,3,4,5,6,7];
    for i in iterator.iter(){
        println!("{i}");
    }

    let msg=Message::Move{x:1,y:2};
    process_message(msg);

    let msg2=Message::ChangeColor(201,234,2);
    process_message(msg2);

    let msg3=Message::Write(String::from("You have received one message"));
    process_message(msg3);

    //instance of Traffic controller

    let traf=TrafficLight::Red;
    println!("Duration of signal {}", duration(&traf));
    println!("Whether the vehicle need to stop {}",traf.is_stop());

    //preventing dangling reference

    let x:&str="camel";
    let y:&str="elephant";
    println!("The longest string is between {} and {} is {}",x,y,borrow_with_lifetime(x,y));

    let info=generic_type{
        School:String::from("messiah"),
        College:String::from("spkc"),
    };
    let school=info.School;
    println!("The student done his schooling in {school}");

    //genertic type with multiple input datatype

    let patient1=generic_Type{
        Patient_name:String::from("henry"),
        Location:String::from("Germany"),
        Disease:String::from("pneumonia"),
        Age:45,
    };

    let patientAge=patient1.Age;
    println!("The age of the patient 1 is {patientAge}");

    //perfect number

    let n=28f64;
    perfectNumber(n);
}

//Traffic Controller

enum TrafficLight{
    Red,
    Yellow,
    Green

}

fn duration(light: &TrafficLight)->u32{
    match light{
        TrafficLight::Red=>60,
        TrafficLight::Yellow=>5,
        TrafficLight::Green=>45
    }
}

impl TrafficLight{
    fn is_stop(&self)->bool{
        match &self{
            TrafficLight::Red=>true,
            TrafficLight::Yellow=>false,
            TrafficLight::Green=>false
        }
        
    }
}

//leveraging lifetime for references

fn borrow_with_lifetime<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len() {x} else {y}
}

//function accepts generic datatype

struct generic_type<T>{
    School:T,
    College:T,

}
//to make the function accept multiple generic data for different data types

struct generic_Type<T,U>{
    Patient_name:T,
    Location:T,
    Disease:T,
    Age:U,
}

//perfect number

fn perfectNumber(n:f64){
    let n_int=n as i32;
    //let square_root:i32 =n.sqrt() as i32;
    //println!("{square_root}");

    let mut divisors:Vec<i32>=Vec::new();
    let half=n_int/2;
    for i in 1..=half{
          if n_int%i==0{
            divisors.push(i);
          }
    }
    println!("The divisors are: {:?}", divisors);
    let total:i32=divisors.iter().sum();

    if n_int==total{
        println!("The given number {n} is an perfect number");
    }
    else{
        println!("The number {n} is not an perfect number");
    }
}
