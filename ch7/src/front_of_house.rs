pub mod hosting {
    pub fn add_to_waitlist(customer: &Customer) {
        println!("Hello customer");
        println!("Customer Details : {}", customer.name);
    }

    pub struct Customer {
        pub name: String,
    }
}
