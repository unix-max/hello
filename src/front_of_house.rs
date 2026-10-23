 fn private_funct() {
        println!("Hello from private house");
        hosting::seat_at_table();
        crate::add_to_waitlist();
        self::hosting::add_to_waitlist();
    }
    

    pub mod hosting;

    pub(super) mod serving {
        pub(self) fn take_order() {}

        pub fn serve_order() {
            println!("serve_order");
        }

        fn take_payment() {}
    }
    fn grome() {
        serving::serve_order();
    }
		pub mod work {
			pub fn cook() {}
		}