use slug::slugify;

pub trait Named {
    fn name(&self) -> String;

    fn slug(&self) -> String {
        slugify(self.name())
    }
}
