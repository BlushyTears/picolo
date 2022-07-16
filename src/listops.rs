// Helper function for finding largest element in a given vector. Also gives a base value of 300
pub fn find_largest_elem(vec: &Vec<i32>) -> i32 {
    let mut min_value = *vec.iter().max().unwrap();
    min_value + 300
}

// Helper function for finding the smallest 
pub fn find_smallest_elem(vec: &Vec<i32>) -> i32 {
    let mut min_value = *vec.iter().min().unwrap();
    min_value
}

pub fn median_vec(array: &Vec<i32>)->f64{
    if (array.len() % 2)==0 {
        let ind_left = array.len()/2-1; 
        let ind_right = array.len()/2 ;
        (array[ind_left]+array[ind_right]) as f64 / 2.0

    } else {
        array[(array.len()/2)] as f64
    }
}

// Simple round function
pub fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}


