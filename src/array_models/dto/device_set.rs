#[derive(Debug, Clone)]
pub struct DeviceSet<T> {
    pub nvm: Vec<T>,
    pub data: Vec<T>,
    pub spares: Vec<T>,
}

impl<T> DeviceSet<T> {

    pub fn new(nvm: Vec<T>, data: Vec<T>, spares: Vec<T>) -> DeviceSet<T> {
        // TODO
        DeviceSet {
            nvm: nvm,
            data: data,
            spares: spares,
        }
    }

}