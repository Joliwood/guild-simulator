pub trait GainsExperience {
    fn gain_xp(&mut self, xp: u32);
    fn level_up(&mut self);
}
