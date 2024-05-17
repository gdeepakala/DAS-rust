// Program to reverse bits in non-negative integer
use std::io;


fn reverse_bits(mut n:i32)->(bool,i32){
    let mut ret:i32=0;
    if n<0{
        return (false,-1);
    }
    while n!=0 {
        ret <<= 1;
        if  (n & 1) == 1{
            ret = ret | 1;
        }
        println!("num={:#b},{}; ret={:#b},{}",n,n,ret,ret);
        n=n>>1;
    }
    (true,ret)
}

fn main() {
    println!("Hello, world!");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read a line");
    let num:i32=line.trim()
                    .parse()
                    .expect("Not a number");
    println!("num = {}",num);
    match reverse_bits(num){
        (true,ret)=>
            println!("Input={:#b},{}; Reversed num={:#b},{}",num,num,ret,ret),
        (false,_)=>
            println!("Input = {} is a negative number",num),
    }
}

// unit test cases for reversal
#[cfg(test)]
mod test{
    use crate::reverse_bits;

    #[test]
    fn test_reverse_negative_num(){
        assert_eq!(reverse_bits(-213),(false,-1));
    }
    #[test]
    fn test_reverse_pos_num(){
        assert_eq!(reverse_bits(213),(true,171));
        assert_eq!(reverse_bits(1),(true,1));
        assert_eq!(reverse_bits(202),(true,83));
    }
}