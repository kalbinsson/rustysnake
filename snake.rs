use crate::random;

use std::collections::VecDeque;

use random::random_range;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
//enum to define all 4 directions snake will go
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    //struct that will hold all of game state
    pub width: usize,
    pub height: usize,
    //head is 1st item in vector
    //tail is last item in vector
    //updated vector to be a queue,
    //b/c movement of snake is implemented by
    // "popping" off tail and "adding" on a head in a direction
    //this simulates movement 
    //vecdeque is double ended queue where you can
    //append items and take out items on both ends
    pub snake: VecDeque<Position>,
    pub direction: Direction,
    next_direction: Direction,
    pub food: Position,
    //flag for when snake is "lost"
    //in case head "adds" onto the board in an illegal spot
    pub finished: bool,
}

impl SnakeGame {
    //width and height provides size of the board
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height,
            //place the snake on the board
            //starts on right side of board
            //.max(0) b/c if not then result could
            //be negative and then the snake would not
            //appear on the board
            snake: [((width - 3).max(0), height / 2)].into_iter().collect(),
            //hardcode direction to left
            direction: Direction::Left,
            //place food on the left of the board
            //.min(width-1) resolves similar problem as w/ snake
            next_direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            finished: false,
        }
    }
    //direction of snake
    pub fn change_direction(&mut self, direction: Direction) {
        if self.finished {
            return;
        }
        match (&self.direction, direction) {
            //specified directions that are "illegal"
            // => {} means that if a player tries then do nothing
            (Direction::Top, Direction::Top)
            | (Direction::Top, Direction::Bottom)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Bottom, Direction::Top)
            | (Direction::Bottom, Direction::Bottom)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {} 
            //specifies that for any legal direction move,
            //the direction then becomes equal to the "new" direction
            (_, direction) => self.next_direction = direction,
        }
    }

    //helper function for timer
    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    //"timer" for game
    //like "frames per second"
    //"moves" snake
    pub fn tick(&mut self) {
        //if the snake is lost
        //or if the snake's length is 0
        //then do not continue
        if self.finished && self.snake.len() == 0 {
            return;
        }

        self.direction = self.next_direction;
        
        //get the current head
        //(x,y) = head
        let (x,y) = self.snake[0];

        //constructing new item (new head)
        //depends on direction the snake is going
        //did "head.map" to map the head b/c this is dependent on
        //head not being empty
        let new_head= match self.direction {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        //losing conditions
        //if the new head placement isnt valid = lose game
        //if head runs into existing part of snake = lose game
        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            //if the snake is NOT eating anything,
            //then the tail should be popped to mimic movement
            //and keep snake length
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                //if the snake is eating something
                //then the tail is preserved to make the snake longer

                //if the position is randomly generated, there is a 
                    //chance the new position may be part of the snake's body
                    //which is an illegal state
                    //mapping all of the snake's possible positions and
                    //basing the randomly generated food item number
                    // off of these "taken" vs "free" positions
                    //helps prevent error

                //all positions NOT occupied by snake 
                //aka free tiles
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    //only filter out the spaces that are not occupied
                    //filter if snake does not contain position
                    .filter(|pos| !self.snake.contains(pos))
                    //collect into a vector
                    .collect::<Vec<_>>();
                
                //if no free positions, then
                //the game finishes
                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }

                //choosing random free space for food item
                self.food = free_positions[random_range(0, free_positions.len())];
                }
                    //else, move snake
                self.snake.push_front(new_head); //adding a new head
            }
    }
}


//writing a test to check logic
#[cfg(test)]
mod tests {
    use crate::SnakeGame;
    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10));
    }
}

//make sure when "wasm-pack  build --target web" is ran the terminal has sudo/admin