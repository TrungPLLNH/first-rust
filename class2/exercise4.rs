fn main(){
    let  a = vec![1,2,3,4,5];  
    let a = reverse(a);
    println!("{:?}", a);
}

pub fn reverse(mut a: Vec<u8>) -> Vec<u8> {
    let mut b:Vec<u8>  = Vec::new();
    loop {
        if a.len() == 0 { break; }
        b.push(a.pop().unwrap());
    }
    b
}