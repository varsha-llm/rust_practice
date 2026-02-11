
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


}
