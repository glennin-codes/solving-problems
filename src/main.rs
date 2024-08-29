use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        map.shrink_to_fit();
      
        // Initialize the solution Vec with a capacity of 2, as it will store exactly two elements
        // let mut soln:Vec<i32> = Vec::with_capacity(2);
             
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = map.get(&complement) {
                // soln.push(j);
                // soln.push(i as i32);
            
                // return soln;
            
                //to reduce memory usage
                //no need to to store intillize a vec
                println!("capacity : {}",map.capacity());
                return vec![j ,i as i32];
            }  
                
            map.insert(num, i as i32);
        }
     
        // soln
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);  // Output should be [0, 1]
}