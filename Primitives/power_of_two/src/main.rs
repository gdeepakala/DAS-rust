use std:: io;

fn is_power_of_two(mut n:i32)->bool{
    if n<0 {
        println!("{} is a negative number",n);
        return false;
    }
    while n!=1 && n>0{
        if n%2 != 0 {// Not a multiple of 2
            return false; 
        }else{
            n/=2;
        }
    }
    return true;
}

fn is_power_of_two_optimized(n: i32)->bool{
    if n<0 {
        println!("{} is a negative number",n);
        return false;
    }
    if (n &(n-1)) == 0 {
        return true;
    }else {
        return false;
    }
}
fn main() {
    println!("Hello, world!");
    let mut temp=String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Nothing to read");
    let n:i32=temp.trim().parse().expect("Not a number");
    match is_power_of_two(n){
        true=>{
            let f:f32 = n as f32;
            println!("{} is {} th power of 2",n,f.log2());
        },
        false=>println!("{} is not a power of 2",n),
    }
    match is_power_of_two_optimized(n){
        true=>{
            let f:f32 = n as f32;
            println!("{} is {} th power of 2",n,f.log2());
        },
        false=>println!("{} is not a power of 2",n),
    }
    
}

#[cfg(test)]
mod test{
    use crate::{is_power_of_two,  is_power_of_two_optimized};
    #[test]
    fn test_power_of_two(){
        assert_eq!(is_power_of_two(1),true);
        assert_eq!(is_power_of_two(4),true);
        assert_eq!(is_power_of_two(256),true);
        assert_eq!(is_power_of_two(1024),true);
    }
    #[test]
    fn test_not_power_of_two(){
        assert_eq!(is_power_of_two(3),false);
        assert_eq!(is_power_of_two(-4),false);
        assert_eq!(is_power_of_two(212),false);
        assert_eq!(is_power_of_two(999),false);
    }
   
    #[test]
    fn test_power_of_two_optimized(){
        assert_eq!(is_power_of_two_optimized(1),true);
        assert_eq!(is_power_of_two_optimized(4),true);
        assert_eq!(is_power_of_two_optimized(256),true);
        assert_eq!(is_power_of_two_optimized(1024),true);
    }
    #[test]
    fn test_not_power_of_two_optimized(){
        assert_eq!(is_power_of_two_optimized(3),false);
        assert_eq!(is_power_of_two_optimized(-4),false);
        assert_eq!(is_power_of_two_optimized(212),false);
        assert_eq!(is_power_of_two_optimized(999),false);
    }
}