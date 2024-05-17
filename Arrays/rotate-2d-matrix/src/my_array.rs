use std::io::{self};
#[derive(Debug, PartialEq)]
pub struct MyArray{
    arr: Vec<Vec<i32>>,
    size:usize,
}

impl MyArray{
    pub fn new(choice:i8,size:usize)->Result<Self,String>{
        let mut my_array:Vec<Vec<i32>>=vec![vec![0;size];size];
        if choice==1{//hardcode array elements - for testing purpose
            let mut count = 1;
            let iter=size as i32;
            for i in 0..size{
                my_array[i]=(count..count+iter).collect();
                count+=iter;
            }
        }else if choice == 2{//user input array elements
            let mut line = String::new();            
            
            println!("Enter the array elements one by one {}*{}: ",size,size);  
            for i in 0..size{
                for j in 0..size{
                    line.clear();
                    io::stdin()
                        .read_line(&mut line)
                        .expect("Nothing to read here");
                    println!("Read element = {}",line);
                    
                    match line.trim().parse::<i32>(){
                        Ok(n)=>my_array[i][j]=n,
                        Err(e)=>{
                            println!("Input error : {}",e);
                            return Err(e.to_string());
                        },
                    }
                    
                }
            }
        }
        let result= MyArray{
            arr:my_array,
            size:size,
        };
        Ok(result)
    }
    pub fn rotate_2d_array(&mut self)->MyArray{
        let mut my_result=vec![vec![0;self.size];self.size];
        for i in 0..self.arr.len(){
            for j in 0..self.arr.len(){
                println!("i={};j={}",i,j);
                my_result[j][self.size-1-i]=self.arr[i][j];
            }
        }
        MyArray { arr: my_result, size:self.size }
    }

    pub fn print_2d_array(&self){
        
        for i in 0..self.arr.len(){
            println!("");
            for j in 0..self.arr[i].len(){
                print!("\t {}",self.arr[i][j]);
            }
        }
        println!("");
    }

    pub fn rotate_2d_array_in_place(&mut self){
        let last_index= self.size-1;
        for layer in 0..self.arr.len()/2{
            for j in layer..last_index{
                let top = self.arr[layer][j];
                let right=self.arr[j][last_index-layer];
                let bottom=self.arr[last_index-layer][last_index-j];
                let left=self.arr[last_index-j][layer];
                let temp=top;
                self.arr[layer][j]=left;
                self.arr[self.size-1-j][layer]=bottom;
                self.arr[self.size-1-layer][last_index-j]=right;
                self.arr[j][self.size-1-layer]=temp;
            }
        }
    }
}

#[cfg(test)]
mod test{
    use crate::my_array::MyArray;

    #[test]
    fn test_my_array_creation(){
        let exp_res=MyArray{
            arr: vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]],
            size:3,
        };
        match MyArray::new(1,3){ // 1- choice to hard code array elements for testing purposes
            Ok(a)=>assert_eq!(a,exp_res),
            Err(e)=>panic!("Error {}",e),
        }
    }

    #[test]
    fn test_my_array_rotation(){
        let exp_res=MyArray{
            arr: vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]],
            size:3,
        };
        let mut test_array=match MyArray::new(1,3){ // 1- choice to hard code array elements for testing purposes
            Ok(a)=>a,
            Err(e)=>panic!("Error {}",e),
        };
        test_array.rotate_2d_array_in_place();
        assert_eq!(test_array,exp_res);
    }
}
