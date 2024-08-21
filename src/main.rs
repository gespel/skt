mod deployment;
mod service;
mod models;

use serde::{Serialize, Deserialize};
use deployment::Deployment;
use service::Service;

fn main() {
    let d = Deployment::new("test".to_string(), "test:1.0.0".to_string());
    let s = Service::new("testsvc".to_string(), "testdeployment".to_string());

    let yaml = serde_yaml::to_string(&d).unwrap();
    let yaml2 = serde_yaml::to_string(&s).unwrap();
    println!("{}", yaml);
    println!("{}", yaml2);
}
