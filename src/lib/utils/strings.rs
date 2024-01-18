use itertools::Itertools;

pub fn vec_to_lowercase(arr: &mut [String]) -> Vec<String> {
    arr.iter_mut().map(|str| str.to_lowercase()).collect_vec()
}

pub fn vec_to_uppercase(arr: &mut [String]) -> Vec<String> {
    arr.iter_mut().map(|str| str.to_uppercase()).collect_vec()
}
