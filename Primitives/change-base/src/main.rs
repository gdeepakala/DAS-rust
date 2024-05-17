use std:: io;
fn print_num_value(num: String, b: u32){
    let mut i:u32 = (num.len() as u32).try_into().unwrap();
    let mut val=0;
    for c in num.trim().chars(){
        
        val+= (c.to_digit(10).unwrap()) * b.pow(i-1);
        if i>=1{
            i-=1;
        }
    }
    println!("val = {}",val);
}
fn convert_decimal_to_binary(num: String)->String{
    let mut n=num.trim().parse::<u32>().unwrap();
    let mut result=String::new();
    while n>0 {
        let t1=n%2;
        let t2 = t1.to_string();
        result.push_str(&t2);
        n=n/2;
    }
    result
}
fn main() {
    //println!("Hello, world!"); 
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nothing to read");
    println!("String entered {}",input);
    let num=input.trim().parse::<i32>().unwrap();
    println!("value = {}",num);
    print_num_value(input.clone(),10);
    print_num_value(input.clone(),2);
    print_num_value("2115".to_string(),8);
    println!("Binary value of 15 is {}",convert_decimal_to_binary("15".to_string()));

}
