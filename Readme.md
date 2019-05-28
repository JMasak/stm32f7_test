rustup target add thumbv7em-none-eabihf


when getting "can't find crate for 'std'" error you forgot to add #![no_std] to source
