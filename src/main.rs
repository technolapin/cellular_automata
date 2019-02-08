mod grid;
use self::grid::NDimSpace;

#[derive(Clone, PartialEq)]
enum State
{
    On, Off, Bite
}

impl State
{
    fn default()-> State
    {
        State::Off
    }

    fn to_string(&self) -> &str
    {
        match self
        {
            State::On  => "O",
            State::Off => " ",
            State::Bite => "·"
        }
    }
    
}


fn transition_locale(neighbors_state: Vec<State>,
                     current_state: &State) -> State
{
    let mut s = 0;
    for cell in neighbors_state.iter()
    {
        match cell
        {
            State::On  => s=s+1,
            _ => ()
        }
    }
    if s == 3
    {
        State::On
    }
    else if s == 2
    {
        current_state.clone()
    }
    else if *current_state != State::Off
    {
        State::Bite
    }
    else
    {
        State::Off
    }
}











struct Automata
{
    width: usize,
    height: usize,
    grid: [multiarray::Array2D<State>; 2],
    flag: usize,
    neighbor: Vec<[i8; 2]>,
    default_state: State,
    transition: fn(Vec<State>, &State) -> State
}






/**
* Implementation of all the universal functions
* (isn't to be changed)
*/
impl Automata
{
    fn new(width: usize,
           height: usize,
           neighborhood: Vec<[i8; 2]>,
           transition: fn(Vec<State>, &State) -> State) -> Automata
    {
        Automata
        {
            width: width,
            height: height,
            grid: [
                multiarray::Array2D::new([width, height], State::default()),
                multiarray::Array2D::new([width, height], State::default())
            ],
            flag: 0,
            neighbor: neighborhood,
            default_state: State::default(),
            transition: transition
        }
    }

    fn look_actual(&self,
                  x: isize,
                  y: isize) -> &State
    {
        if 0 <= x && x < self.width as isize &&
            0 <= y && y < self.height as isize
        {
            &self.grid[self.flag][[x as usize, y as usize]]
        }
        else
        {
            &self.default_state
        }
    }
    
    fn look_next(&self,
                 x: isize,
                 y: isize) -> &State
    {
        if 0 <= x && x < self.width as isize &&
            0 <= y && y < self.height as isize
        {
            &self.grid[1 - self.flag][[x as usize, y as usize]]
        }
        else
        {
            &self.default_state
        }
    }
    
    
    fn set_next(&mut self,
                x: usize,
                y: usize,
                s: State)
    {
        self.grid[1 - self.flag][[x, y]] = s;
    }

    
    fn set_actual(&mut self,
                x: usize,
                y: usize,
                s: State)
    {
        self.grid[self.flag][[x, y]] = s;
    }
    

    fn neighborhood_states(&self,
                     x: usize,
                     y: usize) -> Vec<State>
    {
        let mut ngbr = Vec::new();
        for i in 0..self.neighbor.len()
        {
            ngbr.push(self.look_actual(
                self.neighbor[i][0] as isize + x as isize,
                self.neighbor[i][1] as isize + y as isize
            ).clone());
        }
        ngbr
    }
    
    
    fn transition_globale(&mut self)
    {
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                self.transition_locale(x, y);
            }
        }
        self.flag = 1-self.flag;
    }
    
    fn print(&self)
    {
        for line in 0..self.height
        {
            for col in 0..self.width
            {
                print!("{}", self.look_actual(col as isize, line as isize).to_string());
            }
            println!();
        }
    }

    
    fn evolve(&mut self)
    {
        self.transition_globale();
        self.print();
        for _ in 0..self.width
        {
            print!("{}", "#");
        }
        println!();
    }

    fn transition_locale(&mut self,
                         x: usize,
                         y: usize)
    {

        
        self.set_next(x, y, (self.transition)(
            self.neighborhood_states(x, y),
            self.look_actual(x as isize, y as isize) ));
    }

    //NOT_NECESSARY
    fn is_stationary(&self) -> bool
    {
        for x in 0..self.height
        {
            for y in 0..self.width
            {
                if self.look_actual(x as isize,
                                    y as isize) != self.look_next(x as isize,
                                                                  y as isize)
                {
                    return false;
                }
            }
        }
        return true;
    }

    fn clone_actual(&self) -> multiarray::Array2D<State>
    {
        let mut copy =  multiarray::Array2D::new([self.width, self.height], self.default_state.clone());
        println!("{}", "avant les itérations");
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                println!("{} {} | {} {}", x, y, self.width, self.height);

                copy[[x,y]] = self.look_actual(x as isize, y as isize).clone();
            }
        }
        println!("{}", "end");
        copy
    }

    fn actual_equals(&self, array: &multiarray::Array2D<State>) -> bool
    {
        for x in 0..self.height
        {
            for y in 0..self.width
            {
                if *self.look_actual(x as isize,
                                    y as isize) != array[[x,y]]
                {
                    return false;
                }
            }
        }
        true
    }
    
    fn evolve_until_periodical(&mut self)
    {
        let mut etape: u64 = 0;
        let mut periode: u64 = 1;
        let mut image = self.clone_actual();
        loop
        {
            periode = periode + 1;
            for i in etape..(etape+periode)
            {
                if self.actual_equals(&image)
                {
                    return;
                }
                self.evolve();
            }
            image = self.clone_actual();
            etape = etape + periode;
        }
    }
}




fn main()
{
    let mut toto = Automata::new(42,32, vec![[-1, -1],[0, -1],[1, -1],
                                             [-1,  0],        [1 , 0],
                                             [-1,  1],[0,  1],[1 , 1]],
                                 transition_locale);
    toto.set_actual(5, 5, State::On);
    toto.set_actual(6, 5, State::On);
    toto.set_actual(5, 6, State::On);

    toto.set_actual(17, 3, State::On);
    toto.set_actual(18, 3, State::On);
    toto.set_actual(16, 4, State::On);
    toto.set_actual(15, 5, State::On);
    toto.set_actual(15, 6, State::On);
    toto.set_actual(15, 7, State::On);
    toto.set_actual(16, 8, State::On);
    toto.set_actual(17, 9, State::On);
    toto.set_actual(18, 9, State::On);

    toto.set_actual(19, 6, State::On);
    toto.set_actual(20, 4, State::On);
    toto.set_actual(20, 8, State::On);
    toto.set_actual(21, 5, State::On);
    toto.set_actual(21, 6, State::On);
    toto.set_actual(21, 7, State::On);
    toto.set_actual(22, 6, State::On);

    toto.set_actual(25, 5, State::On);
    toto.set_actual(26, 5, State::On);
    toto.set_actual(25, 4, State::On);
    toto.set_actual(26, 4, State::On);
    toto.set_actual(25, 3, State::On);
    toto.set_actual(26, 3, State::On);
    
    toto.set_actual(27, 2, State::On);
    toto.set_actual(27, 6, State::On);

    toto.set_actual(29, 1, State::On);
    toto.set_actual(29, 2, State::On);

    toto.set_actual(29, 6, State::On);
    toto.set_actual(29, 7, State::On);

    toto.set_actual(39, 3, State::On);
    toto.set_actual(39, 4, State::On);
    toto.set_actual(40, 3, State::On);

    
    toto.print();
   /*
    while !toto.is_stationary()
    {
        toto.evolve();
}*/

    //toto.evolve_until_periodical();

    let mut spaace = NDimSpace::new(2); 

    spaace.set(&[-1, -1], 1);
    spaace.set(&[0, -1], 2);
    spaace.set(&[1, -1], 3);

    spaace.set(&[-1, 0], 4);
    spaace.set(&[0, 0], 5);
    spaace.set(&[1, 0], 6);

    spaace.set(&[-1, 1], 7);
    spaace.set(&[0, 1], 8);
    spaace.set(&[1, 1], 9);

    for line in -8..8 {
        for column in -8..8 {
            print!("{}", spaace.get(&[column, line]));
        }
        println!();
    }
}
