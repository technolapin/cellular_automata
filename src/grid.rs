
pub struct NDimSpace<T>
{
    dim: u8,
    datas: rudy::rudymap::RudyMap<u64, T>,
    default_value: T
}


impl<T> NDimSpace<T>
where T: Default + Copy
{
    pub fn new(dim: u8) -> NDimSpace<T>
    {
        NDimSpace
        {
            dim: dim,
            datas: rudy::rudymap::RudyMap::new(),
            default_value: T::default()
        }
    }

    // ** on veut pouvoir:
    // accéder à un élément depuis ses coordonées

    fn cantor(a: u64, b: u64) -> u64
    {
        ((a+b)*(a+b)+3*a+b)/2
    }

    fn Z_to_N(a: i64) -> u64
    {
        if (a < 0)
        {
            (-a*2+1) as u64
        }
        else
        {
            (a*2) as u64
        }
    }
  
    fn coord_to_index(coords: &[i64]) -> u64
    {
        match coords.split_first()
        {
            Some((tete, &[])) => Self::Z_to_N(*tete),
            Some((tete, queue)) => Self::cantor(Self::Z_to_N(*tete), Self::coord_to_index(queue)),
            None => panic!("Coordonée vide lors de la conversion vers index")
        }
    }
    
    fn access_mut(&mut self, coords: &[i64]) -> &mut T
    {
        let index = Self::coord_to_index(coords);
        if !self.datas.contains_key(index)
        {
            self.datas.insert(index, T::default());
        }

        match self.datas.get_mut(index)
        {
            Some(pointer) => pointer,
            None => unreachable!()
        }
    }

    fn access(&self, coords: &[i64]) -> &T
    {
        let index = Self::coord_to_index(coords);
        match self.datas.get(index)
        {
            Some(pointer) => pointer,
            None => &self.default_value
        }
    }

    pub fn set(&mut self, coords: &[i64], value: T)
    {
        *self.access_mut(coords) = value;
    }

    pub fn get(&self, coords: &[i64]) -> T
    {
        *self.access(coords)
    }

    
}
