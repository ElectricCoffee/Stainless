pub mod taint;

#[cfg(test)]
mod tests {
    use taint::{Clean, Dirty};

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
    fn test_from() {
        let a: Clean<i32> = 1.into();
        let b: Dirty<i32> = 2.into();

        assert_eq!(a, Clean(1));
        assert_eq!(b, Dirty(2));
    }

    #[test]
    fn test_map() {
        let a = Clean(1).map(|x| x + 3);
        let b = Dirty(3).map(|x| x * 3);
        assert_eq!(a, Clean(4));
        assert_eq!(b, Dirty(9));
    }

    fn double_clean(x: i32) -> Clean<i32> {
        Clean(x * 2)
    }

    fn triple_dirty(x: i32) -> Dirty<i32> {
        Dirty(x * 3)
    }

    #[test]
    fn test_and_then() {
        let a = Clean(8).and_then(double_clean).and_then(double_clean);
        let b = Dirty(2).and_then(triple_dirty).and_then(triple_dirty);
        assert_eq!(a, Clean(32));
        assert_eq!(b, Dirty(18));
    }
}
