// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.


// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}


// fn mult_box() {
//     let mut num: Box<u32> = Box::new(3);
//     num_sq(&mut num);
//     assert_eq!(*num, 9);
// }
// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq1<T,U>(arg: &mut T)
where T:AsMut<U>,U:std::ops::Mul<Output=U>+Copy
{
    let val = arg.as_mut();
    *val = *val * *val;    
}

// 更加灵活
/**
 * struct NonCopy {
    value: i32,
}

impl std::ops::Mul for &NonCopy {
    type Output = NonCopy;

    fn mul(self, rhs: Self) -> NonCopy {
        NonCopy {
            value: self.value * rhs.value,
        }
    }
}

fn main() {
    let mut x = NonCopy { value: 3 };
    let mut boxed_x: Box<NonCopy> = Box::new(x);
    num_sq(&mut boxed_x);
    println!("{}", boxed_x.value); // 输出 9
}
 */
fn num_sq<T,U>(arg: &mut T)
where T:AsMut<U>,for<'a> &'a U:std::ops::Mul<Output=U>
{
    let val = arg.as_mut();
    *val = &*val * &*val;    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
