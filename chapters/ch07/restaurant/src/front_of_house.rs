pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

    fn do_something() {}

    fn super_import() {
        // hosting::add_to_waitlist(); // Wont work
        super::hosting::add_to_waitlist(); // Goes to same level as hosting module
        // super::eat_at_restaurant(); // Wont work
    }
}
