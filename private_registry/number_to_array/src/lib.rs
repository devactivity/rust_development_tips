//! Ulangan Rust #3
//! Kamu harus buat sendiri fungsi konversi number menjadi array
//! Tidak tersedia fungsi ataupun method yg bisa langsung digunakan

/// # Conver number to array in Rust ()
/// konversi `u64` number menjadi array `u8`
///
/// ```
/// use number_to_array::convert_num;
///
/// let result = convert_num(2857);
/// assert_eq!(result, vec![2,5,7,8]);
/// ```
pub fn convert_num(num: u64) -> Vec<u8> {
    // no sorting and remove duplicate
    let mut res = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    // sorting the array
    res.sort();

    // remove duplicate (WAJIB di `sort` dulu)
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        // not sorted & contain duplicate elements
        // assert_eq!(convert_num(355231), vec![3,5,5,2,3,1]);
        // assert_eq!(convert_num(0), vec![0]);

        // sorted result
        // assert_eq!(convert_num(355231), vec![1,2,3,3,5,5]);

        // remove duplicate result
        assert_eq!(convert_num(355231), vec![1, 2, 3, 5,]);
    }
}
