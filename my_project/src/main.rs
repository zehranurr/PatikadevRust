fn main() {


    let sum = add(3,5);
    println!(" {}",sum);
    let start_string=String:: from("This project is demo for Patika Dev Polkadot Rust ");
    let slice =&start_string[24..50];
    println!("{}",slice);

    let my_string=String::from("Hello ,Patika Dev!");
    print_string(my_string);
   // println!("{}",my_string); if we use this one again it will give error


    let mut newmutvar_ = String::from(" I m studying computer");


    change_string(&mut newmutvar_);




    let mut explanation= String::from("here we define mut str  and references to immut str ");
    let r1=&explanation;
    let r2=&explanation;
    println!("{}, {}", r1, r2);
    let myage=22 ; //  integer value stored on the stack 
    let whoamI = String::from("This is Zehra Nur :).I m "); // stored on the heap 
    let Im = whoamI; // ownership of whoamI is moved to Im 
    println!("Im = {}, myage = {}", Im,myage ); 






    
}
fn change_string(s: &mut String) {
    s.push_str(" engineer");
    println!("this messge is modified {}",s);
}





fn add(x: i32, y: i32)-> i32{

    let result = x + y;
    return result;
}



fn no_prm()-> i32 {
    println!("this just works");
    1
}

fn print_string(s : String){

    println!("{}",s);
}



