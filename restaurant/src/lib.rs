// mod declaration and name of module
mod front_of_house {

// further modules inside parent module
// like this we can group related definitions together

    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}