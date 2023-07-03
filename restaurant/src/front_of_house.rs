// by default, modules are private to any thing except the module and its children, so we must mark this as public so it can be used in eat_at_restaurant     
pub mod hosting; // this is implemented in src\front_of_house\hosting.rs
                    // we could have also put it at src\front_of_house\mod.rs, but we can only use one style of module organization per crate
                    // the only downside to this method is that it can be confusing to have mutliple files named mod.rs in the same project                

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
