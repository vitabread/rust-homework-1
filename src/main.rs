mod mod1;

mod submodule {
    pub mod mod2;
}

use submodule::mod2;

fn main() {
    mod1::print_characters();
    mod2::print_characters();
}
