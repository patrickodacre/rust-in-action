pub fn run()
{
    // lifetimes, generics and traits::
    let mut k = Knight::new("Daniel".to_string(), 20, 100);
    let mut m = Mob::new(40);

    let attacked = k.attack(&mut m);

    let attacked = m.attack(&mut k);
}

// I wasn't able to get this to work as an associated type to the HasHealth trait
type Hp = u32;

struct Knight
{
    name: String,
    atk: u32,
    hp: Hp,
}

struct Mob
{
    atk: u32,
    hp: Hp,
}

impl Knight
{
    fn new(name: String, atk: u32, hp: u32) -> Self
    {
        Knight { name, atk, hp }
    }
}

impl Mob
{
    fn new(hp: Hp) -> Self
    {
        Mob { atk: 20, hp }
    }
}

trait HasHealth
{
    type HealthPoints: std::fmt::Display + std::fmt::Debug;

    fn decrease_health(&mut self, amount: Hp) -> Result<(), &'static str>;
    fn health_points(&self) -> Self::HealthPoints;
}

impl HasHealth for Knight
{
    type HealthPoints = u32;
    fn decrease_health(&mut self, amount: Hp) -> Result<(), &'static str>
    {
        // instant death
        if amount >= self.hp {
            self.hp = 0;

            return Ok(());
        }

        let new_hp = self
            .hp
            .checked_sub(amount)
            .ok_or("Cannot decrease health")?;

        self.hp = new_hp;

        Ok(())
    }

    fn health_points(&self) -> Self::HealthPoints
    {
        self.hp
    }
}

impl HasHealth for Mob
{
    type HealthPoints = u32;
    fn decrease_health(&mut self, amount: Hp) -> Result<(), &'static str>
    {
        // instant death
        if amount >= self.hp {
            self.hp = 0;

            return Ok(());
        }

        let new_hp = self
            .hp
            .checked_sub(amount)
            .ok_or("Cannot decrease health")?;

        self.hp = new_hp;

        Ok(())
    }

    fn health_points(&self) -> Self::HealthPoints
    {
        self.hp
    }
}

trait CanAttack
{
    // by bounding this to the trait HasHealth I'm saying that this function
    // needs to operate on a type the impl that trait.
    // if my intention is to access Mob / Knight struct properties on attackee
    // but still use generics, then the new trait will have to expose
    // getters and setters for those properties. The impl will then
    // point caller to those properties on the implementing struct
    fn attack<T: HasHealth>(&self, attackee: &mut T) -> Result<(), &'static str>;
}

impl CanAttack for Knight
{
    fn attack<T: HasHealth>(&self, mob: &mut T) -> Result<(), &'static str>
    {
        println!("Knight is attacking!!");

        if !mob.decrease_health(self.atk).is_ok() {
            return Err("Attack failed.");
        }

        // I've extended the Display trait
        println!("Remaining health points {}\n", mob.health_points());

        Ok(())
    }
}

impl CanAttack for Mob
{
    fn attack<T: HasHealth>(&self, knight: &mut T) -> Result<(), &'static str>
    {
        println!("Mob is attacking!!");

        if !knight.decrease_health(self.atk).is_ok() {
            return Err("Attack failed.");
        }

        // I've also extended the Debug trait
        println!("Remaining health points {:?}\n", knight.health_points());

        Ok(())
    }
}
