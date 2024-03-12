use std::ops::{Div, Rem};

use super::constant;


pub fn get_total_pages(total_items: i32)-> i32{
    let limit = constant::FETCH_LIMIT;

    let rem = total_items.rem(limit);

    // if is even
    if rem.eq(&(constant::NIL as i32)){
        return total_items.div(limit) as i32
    }else{
        // else if odd
        return ((total_items - rem).div(limit) + 1)  as i32
    }
}