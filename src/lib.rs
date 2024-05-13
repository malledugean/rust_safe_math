pub mod calc_basic {
    /// # Safe sum two u8 numbers 
    /// Returns a u8 number, when overflow == 255
    /// 
    /// # sample
    /// 
    /// ``` rust
    /// use dicoco_safe_math::calc_basic;
    /// 
    /// assert_eq!( 15, calc_basic::sum_x_y(13,2));
    /// assert_eq!( 0, calc_basic::sum_x_y(0,0));
    /// assert_eq!( 255, calc_basic::sum_x_y(123,243));
    /// ```
    pub fn sum_x_y (x: u8, y:u8) -> u8 {
        match x.checked_add(y) {
            Some(v) => {
                return v;
            }
            None => {
                return 255;
            }
        };
    }

    /// # Safe subtracts two u8 numbers
    /// Returns a u8 number, when underflow == 0
    /// 
    /// # sample
    /// 
    /// ``` rust
    /// use dicoco_safe_math::calc_basic;
    /// 
    /// assert_eq!( 15, calc_basic::sub_x_y(17,2));
    /// assert_eq!( 0, calc_basic::sub_x_y(2,18));
    /// assert_eq!( 240, calc_basic::sub_x_y(245,5));
    /// ```
    pub fn sub_x_y (x: u8, y:u8) -> u8 {
        if x < y {
            return 0;
        }
        x - y 
    }
}


#[cfg(test)]
mod test {

    use super::calc_basic;

    #[test]
    fn test_sum_1(){
        let result = calc_basic::sum_x_y(7, 8);

        assert_eq!(result, 15);
    }

    
    #[test]
    fn test_sum_2(){
        let result = calc_basic::sum_x_y(11, 250);

        assert_eq!(result, 255);
    }

    
    #[test]
    fn test_sub_1(){
        let result = calc_basic::sub_x_y(7, 8);

        assert_eq!(result, 0);
    }
    
    #[test]
    fn test_sub_2(){
        let result = calc_basic::sub_x_y(17, 8);

        assert_eq!(result, 9);
    }

}