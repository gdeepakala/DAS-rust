use std:: io;

fn is_power_of_four(mut n:i32)->bool{
    if n<0 {
        println!("{} is a negative number",n);
        return false;
    }
    while n!=1 && n>0{
        if n%4 != 0 {// Not a multiple of 4
    
            return false; 
        }else{
            n/=4;
        }
    }
    return true;
}

fn is_power_of_four_optimized(n: i32)->bool{
    if n<0 {
        println!("{} is a negative number",n);
        return false;
    }
    if (n &(n-1)) == 0 {// This proves number is power of 2
        /*let ftemp: f32 = n as f32;
        if (ftemp.log2() as i32 %2) == 0 {
            // log2() Gives the position of '1' bit in the number
            // If the position is even, then the number is a power of 4
            return true;
        } else{
            // '1' bit in odd position - power of 2
            return false;
        }*/
        // use 32-bit hex mask
        let mask:i32=0x55555555;
        if n&mask != 0{
            return true; // the '1' bit is even indexed - power of 4
        }else{
            return false; // power of 2
        }
        
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
    match is_power_of_four(n){
        true=>{
            let f:f32 = n as f32;
            println!("{} is {} th power of 4",n,(f.log2() as i32)/2);
        },
        false=>println!("{} is not a power of 4",n),
    }
    match is_power_of_four_optimized(n){
        true=>{
            let f:f32 = n as f32;
            println!("{} is {} th power of 4",n,(f.log2() as i32)/2);
        },
        false=>println!("{} is not a power of 4",n),
    }
    
}

#[cfg(test)]
mod test{
    use crate::{is_power_of_four, is_power_of_four_optimized};
    #[test]
    fn test_power_of_four(){
        assert_eq!(is_power_of_four(1),true);
        assert_eq!(is_power_of_four(4),true);
        assert_eq!(is_power_of_four(256),true);
        assert_eq!(is_power_of_four(1024),true);
    }
    #[test]
    fn test_not_power_of_four(){
        assert_eq!(is_power_of_four(3),false);
        assert_eq!(is_power_of_four(-4),false);
        assert_eq!(is_power_of_four(32),false);
        assert_eq!(is_power_of_four(2048),false);
    }
   
    #[test]
    fn test_power_of_four_optimized(){
        assert_eq!(is_power_of_four_optimized(1),true);
        assert_eq!(is_power_of_four_optimized(4),true);
        assert_eq!(is_power_of_four_optimized(256),true);
        assert_eq!(is_power_of_four_optimized(1024),true);
    }
    #[test]
    fn test_not_power_of_four_optimized(){
        assert_eq!(is_power_of_four_optimized(3),false);
        assert_eq!(is_power_of_four_optimized(-4),false);
        assert_eq!(is_power_of_four_optimized(200),false);
        assert_eq!(is_power_of_four_optimized(2048),false);
    }
}