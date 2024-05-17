/* 
    Program: Given an nxn two-dimensional matrix of numbers, 
                -rotate the matrix 90 degrees to the right (clockwise).

Input-Output

Example:

In

[
  [ 1,2,3],
  [ 4,5,6],
  [7,8,9]
]
Out

[
 [7,4,1],
 [8,5,2],
 [9,6,3]
]
 */ 
mod my_array;
use std::io;

use crate::my_array::MyArray;

fn main() -> Result<(),String>{
    let mut line = String::new();
    let choice =2; // user input array elements
    println!("Enter the size of square matrix");   
    io::stdin()
        .read_line(&mut line)
        .expect("Nothing to read here");
    let size=line.trim().parse::<usize>().expect("Not a number");
    let mut input= match MyArray::new(choice,size){
        Ok(res)=>res,
        Err(e)=>return Err(e),
    };
    println!("\n Input Array:");
    input.print_2d_array();
    let res=input.rotate_2d_array();
    println!("\nRotated matrix:(new result array)");
    res.print_2d_array(); 
    input.rotate_2d_array_in_place();
    println!("\nRotated matrix (in-place):");
    input.print_2d_array(); 
    Ok(())
    
}
