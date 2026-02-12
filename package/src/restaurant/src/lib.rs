mod front_house{
    pub mod order{
    
          
            pub struct greeting{
                pub sweet:String,
                cooldrink:String,
                coffee:String
            }
            impl greeting{
                pub fn orders(order:&str)->Self{
                    Self{
                        sweet:String::from(order),
                        cooldrink:String::from("coak"),
                        coffee:String::from("mud")
                    }
                }
                fn quotes(){
                    println!("Have a gifted day");
                }
               
            
        }
        fn catalogue(){}
        pub fn take_order(){
            pub enum orderedFood{
                orderlist1(String,String),
                orderlist2(String,String)
            }
        }
    }
    mod serve{
        fn prepare(){}
        fn arrange(){}
    }
}
use crate::front_house::order::greeting;
fn accessingModule(){
    //absolute path
    crate::front_house::order::take_order();

    //relative path
    front_house::order::take_order();

    //accessing coffee from the submodule order
    let placedOrder=front_house::order::greeting::orders("kaju katli");

    //simply with 'use' keyword
    let order2=greeting::orders("rasagulla");


    println!("ordered sweet {}",placedOrder.sweet);

}
