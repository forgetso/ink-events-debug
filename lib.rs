#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod flipper {
    #[ink(event)]
    pub struct Flip {
        value: bool,
    }

    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            // Note: We added this event
            Self::env().emit_event(Flip { value: self.value });
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert!(!flipper.get());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert!(!flipper.get());
            flipper.flip();
            assert!(flipper.get());

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();

            // Both of these print out okay
            dbg!(&emitted_events);
            ink_env::debug_println!("{:?}", emitted_events);

            // Panicking so we can print the debug messages
            panic!()
        }
    }

    #[cfg(feature = "ink-experimental-engine")]
    #[cfg(test)]
    mod tests_experimental_engine {
        use ink_lang as ink;
        use super::*;

        /// Test add operator
        #[ink::test]
        fn it_breaks() {
            let mut flipper = Flipper::new(false);
            assert!(!flipper.get());
            flipper.flip();
            assert!(flipper.get());

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();

            // `EmittedEvent` doesn't implement `Debug`"
            dbg!(&emitted_events);
            ink_env::debug_println!("{:?}", emitted_events);

            // Panicking so we can print the debug messages
            panic!()
        }
    }
}