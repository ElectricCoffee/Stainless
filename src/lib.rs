pub mod taint;

#[cfg(test)]
mod tests {
    use taint::{Clean, Dirty, Result};

    #[test] 
    fn test_add() {
        let a = Clean(2) + Clean(3);
        let b = Clean(2) + Dirty(3);
        let c = Dirty(2) + Clean(3);
        let d = Dirty(2) + Dirty(3);

        assert_eq!(a, Clean(5));
        assert_eq!(b, Dirty(5));
        assert_eq!(c, Dirty(5));
        assert_eq!(d, Dirty(5));
    }

    #[test]
    fn test_result() {
        let a: Result<i32> = Ok(Clean(1));
        let b: Result<i32> = Err(Dirty(2));

        let r1 = a.map(|x| x + Clean(2)).map_err(|x: Dirty<i32>| x + Dirty(3));
        assert_eq!(r1, Ok(Clean(3)));

        let r2 = b.map(|x: Clean<i32>| x + Clean(2)).map_err(|x| x + Dirty(3));
        assert_eq!(r2, Err(Dirty(5)));
    }
}
