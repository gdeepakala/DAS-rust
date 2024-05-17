use std::io;



fn number_palindrome(mut n: i32)->bool{
    if n<0 { // return false if negative number
        return false;
    }
    //let mut n1 = n;
    let temp = n as f32; // Total Digits = (log n to base 10) + 1 
    let len=temp.log10() as i32; // log10 function only available for float, so temp variable
    let mut len:u32=len as u32;
    println!("len = {}",len);
    let mut msb=0; // Most Significant Byte
    let mut lsb=0; // Least Significant Byte
    let base:i32 = 10;
    while n>0 && len >0{
        let mask = base.pow(len); // Mask for extracting MSB
        msb = n / mask;
        lsb = n % 10;
        println!(" mask={};msb={};lsb={};len={}",mask,msb,lsb,len);
        if msb == lsb {
            n %= mask;
            n /= 10;
            if(len >=2){
                len=len-2;
            }
            continue;
        }else{
            return false;
        }
        
    }

    return true;
}
fn main() {
    println!("Hello, world!");
    let mut  line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n = line.trim()
                    .parse()
                    .expect("Not a number");
    let result = number_palindrome(n);
    match result{
        true=> println!(" {} is a palindrome",n),
        false=> println!(" {} is not a palindrome",n),
    }
}

//unit tests - is a palindrome, not a palindrome, negative number

#[cfg(test)]
mod test{
    use crate::number_palindrome;

    #[test]
    fn test_palindrome(){
        assert_eq!(number_palindrome(43234),true);
    }
    #[test]
    fn test_not_a_palindrome(){
        assert_ne!(number_palindrome(1234),true);
    }
    #[test]
    fn test_negative_number(){
        assert_eq!(number_palindrome(-1234),false);
    }
}