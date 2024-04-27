/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd+Copy>(array: &mut [T]){
    let len = array.len();
    if array.len() < 2 {
        return;
    }

    let (array1, array2) = array.split_at_mut(len/2);
    sort(array1);
    sort(array2);

    let v = merge(array1, array2);

    array.copy_from_slice(&v);
}

fn merge<T: PartialOrd+Copy>(array_a: &mut [T], array_b: &mut[T]) -> Vec<T> {
    let mut v = Vec::new();
    let (mut ai, mut bi) = (0, 0);
    while ai < array_a.len() || bi < array_b.len() {
        if ai < array_a.len() && bi < array_b.len() && array_a[ai] < array_b[bi] || bi == array_b.len() {
            v.push(array_a[ai]);
            ai += 1;
        }else {
            v.push(array_b[bi]);
            bi += 1;
        }
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}