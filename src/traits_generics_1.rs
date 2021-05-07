pub fn run()
{
    // lifetimes, generics and traits::
    let k = Knight::new("Daniel".to_string());
    let mut m = Mob::new(10);
    println!("{:?}", m.hp);

    let attacked = k.attack(&mut m);

    println!("Attacked {:?}", attacked);

    println!("{:?}", m.hp);
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
    fn new(name: String) -> Self
    {
        Knight {
            name,
            atk: 20,
            hp: 100,
        }
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
    fn decrease_health(&mut self, amount: Hp) -> Result<(), &'static str>;
}

impl HasHealth for Knight
{
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
}

impl HasHealth for Mob
{
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
}

trait CanAttack
{
    fn attack<T: HasHealth>(&self, attackee: &mut T) -> Result<(), &'static str>;
}

impl CanAttack for Knight
{
    fn attack<T: HasHealth>(&self, mob: &mut T) -> Result<(), &'static str>
    {
        if !mob.decrease_health(self.atk).is_ok() {
            return Err("Attack failed.");
        }

        Ok(())
    }
}

impl CanAttack for Mob
{
    fn attack<T: HasHealth>(&self, knight: &mut T) -> Result<(), &'static str>
    {
        if !knight.decrease_health(self.atk).is_ok() {
            return Err("Attack failed.");
        }

        Ok(())
    }
}
