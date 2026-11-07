//! lxd model — system container and VM manager.

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub type_: String,
    pub state: String,
    pub image: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub instances: Vec<Instance>,
}

impl Default for Model {
    fn default() -> Self { Self { instances: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn launch(&mut self, name: &str, image: &str, type_: &str) -> bool {
        self.instances.push(Instance { name: name.into(), type_: type_.into(), state: "Running".into(), image: image.into() });
        true
    }

    pub fn list(&self) -> &[Instance] { &self.instances }

    pub fn stop(&mut self, name: &str) -> bool {
        if let Some(i) = self.instances.iter_mut().find(|i| i.name == name) { i.state = "Stopped".into(); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn launch_stop() {
        let mut l = Model::new();
        assert!(l.launch("c1", "ubuntu", "container"));
        assert!(l.stop("c1"));
        assert_eq!(l.list()[0].state, "Stopped");
    }
}
