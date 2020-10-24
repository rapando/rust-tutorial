mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // relative path
    let customer = hosting::Customer {
        name: String::from("Rapando Chitechi"),
    };
    hosting::add_to_waitlist(&customer);
}
