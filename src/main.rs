#[derive(Clone, PartialEq)]
enum State
{
    On, Off, Bite
}

struct Automata
{
    width: usize,
    height: usize,
    grid: [multiarray::Array2D<State>; 2],
    flag: usize,
    neighbor: Vec<[i8; 2]>,
    default_state: State
    
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
            State::Bite=> "·"
        }
    }
}


/**
* Implementation of all the non-universal functions for 2D
* (is to be changed if the automata's rules are changed)
*/
impl Automata
{
    fn new(width: usize,
           height: usize) -> Automata
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
            neighbor: vec![[-1, -1],[0, -1],[1, -1],
                           [-1,  0],        [1 , 0],
                           [-1,  1],[0,  1],[1 , 1]],
            default_state: State::default()
        }
    }

    
    fn transition_locale(&mut self,
                         x: usize,
                         y: usize)
    {
        let mut s = 0;
        for cell in self.neighborhood_states(x, y).iter()
        {
            match cell
            {
                State::On  => s=s+1,
                _ => ()
            }
        }
        self.set_next(x, y,
                      if s == 3
                      {
                          State::On
                      }
                      else if s == 2
                      {
                          self.look_actual(x as isize, y as isize).clone()
                      }
                      else if *self.look_actual(x as isize, y as isize) != State::Off
                      {
                          State::Bite
                      }
                      else
                      {
                          State::Off
                      }
        );
    }
}


/**
* Implementation of all the universal functions
* (isn't to be changed)
*/
impl Automata
{
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
        for i in 0..self.width
        {
            print!("{}", "#");
        }
        println!();
    }
}



fn main() {
    let mut toto = Automata::new(20,10);
    toto.set_actual(0, 0, State::On);
    toto.set_actual(1, 1, State::On);
    toto.set_actual(1, 2, State::On);
    toto.set_actual(2, 1, State::On);
    toto.set_actual(2, 0, State::On);
    toto.print();
    
    while !toto.is_stationary()
    {
        toto.evolve();
    }
}
