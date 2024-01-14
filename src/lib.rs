pub mod platform;

#[cfg(test)]
mod tests {
    use crate::platform::{
        agent::{behavior::Behavior, Agent},
        Platform,
    };

    #[test]
    fn platform_boot() {
        struct A;
        let data_a = A;
        impl Behavior for Agent<A> {
            fn action(&mut self) {
                println!("HOLAAAAA AGENTE A")
            }
        }
        let mut ap = Platform::new("crate_test".to_string());
        //let mut ag_a = Agent::new("Agente-A".to_string(), 1, 4,"crate_test", data_a).unwrap();
        let boot = ap.boot();
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert!(boot.is_ok());
        let ag_a = ap.add::<A>("Agente-A".to_string(), 1, 4, data_a).unwrap();
        let start = ap.start(ag_a);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert!(start.is_ok());
    }

    #[test]
    fn instantiating() {
        struct B;
        struct C;
        let data_b = B;
        let data_c = C;

        impl Behavior for Agent<C> {
            fn action(&mut self) {}
        }
        impl Behavior for Agent<B> {
            fn action(&mut self) {}
        }
        /*let ag_b = Agent::new("Agente-B".to_string(), 98, 4, "RPi", data_b);
        let ag_c = Agent::new("Agente-C".to_string(), 99, 4, "RPi", data_c);

        std::thread::sleep(std::time::Duration::from_millis(2000));

        assert!(ag_b.is_ok());
        assert!(ag_c.is_err());*/
    }
}
