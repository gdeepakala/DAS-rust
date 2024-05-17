use std::io;

fn main() -> Result<(),String>{
    
    let mut size=0;
    let mut input = String ::new();
    println!("Enter the size of the array: ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("nothing to read");
    size = match input.trim().parse::<usize>(){
        Ok(s)=>s,
        Err(e)=>return Err(e.to_string()),
    };
    assert_ne!(size,0);
    let oned_array:Vec<i32>=(0..size as i32).collect();
    
    for i in 0..size{
        println!(" {:?} ",oned_array[i]);
    }
    Ok(())
}
