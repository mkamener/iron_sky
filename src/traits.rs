use game::Collider;

pub trait Collides {
    fn collides_with<C: Collides>(&self, other: &C) -> bool;

    fn get_collider(&self) -> &Collider;
}
