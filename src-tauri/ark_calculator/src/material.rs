pub(crate) struct Material{
    pub(crate) name:String
}

impl Material {
    pub(crate) fn new(name:String) -> Self{
        Self{
            name
        }
    }
}