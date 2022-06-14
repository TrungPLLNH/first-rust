fn main() {
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8];
  
    println!("{}", is_sub_array(&org_arr, &sub_arr));
  }
  
  fn is_sub_array(a: &[i32], b: &[i32]) -> bool {
    let _len_a = a.len();
    let _len_b = b.len();
    let mut i = 0;
    let mut j = 0;
    while i < _len_a && j < _len_b {
      if a[i] == b[j] {
        i+=1;
        j+=1;
        if j == _len_b {
          return true;
        }     
      } else {
        i = i - j + 1;
        j = 0;
      }
    }
    return false;
  }