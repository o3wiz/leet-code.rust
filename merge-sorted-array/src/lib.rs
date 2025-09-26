// https://leetcode.com/problems/merge-sorted-array

pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = m + n - 1;

    while i >= 0 && j >= 0 {
        if a[i as usize] > b[j as usize] {
            a[k as usize] = a[i as usize];
            i -= 1;
        } else {
            a[k as usize] = b[j as usize];
            j -= 1;
        }
        k -= 1;
    }

    while j >= 0 {
        a[k as usize] = b[j as usize];
        j -= 1;
        k -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
