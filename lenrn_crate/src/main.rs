mod factory {
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("Produce re!");
        }
    }

    pub mod produce_washing_machine {
        pub fn produce_w() {
            println!("produce washing machine!");
        }
    }
}

fn main() {
    factory::produce_refrigerator::produce_re();
}
