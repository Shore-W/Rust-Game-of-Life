// my function to compute the next generation
fn next_generation(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> { //this function takes a reference to a a matrix and returns a new matrix 

    let n = grid.len(); // # of rows
    let m = grid[0].len(); // # of columns
    let mut future: Vec<Vec<i8>> = vec![vec![0; n]; m]; //this constructs a new MxN matrix full of 0s

    for i in 0..n {
        for j in 0..m {
            
            let cell_state = grid[i][j];
            //stores the current cell in a variable
            
            let mut live_neighbors = 0;
            //resets the live_neighbor to be 0 with each new loop so that its value is local to the loop

            for x in -1i8..=1 {
                for y in -1i8..=1 {

                    //takes the possible positions of the neighbors for each point
                    // (1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1), (1,-1), (0,0)

                    let new_x = (i as i8) + x;
                        //calculates the neighbors of x

                    let new_y = (j as i8) + y;
                        //calculates the neighbors of y

                    if new_x >= 0 && new_y >= 0 && new_x < n as i8 && new_y < m as i8 {
                        //by checking if the points are greater than/equal to 0, it makes sure the grid itself isn't compromised with new values
                        // and, by seeing if new_x and new_y are less than the dimensions of the grid, it saves itself from an indexing error

                        live_neighbors += grid[new_x as usize][new_y as usize];
                        //this saves the neighbor locations in the grid
                    }
                }
            }

            live_neighbors -= cell_state;
            //this adds the value of neighbors to the cell

            if cell_state == 1 && live_neighbors < 2 {
                //if the grid[i][j] == 1, and its live neighbors relative is less than 2, this cell will die 
                    future[i][j] = 0;

            } else if cell_state == 1 && live_neighbors > 3 {
                    future[i][j] = 0;
                //if the grid[i][j] == 1, and its live neighbors relative is greater than 3, this cell will die

            } else if cell_state == 0 && live_neighbors == 3 {
                    future[i][j] = 1;
                //if the grid[i][j] == 0, and its live neighbors relative is 3, this cell will come to life
            }
            else {
                future[i][j] = cell_state;
            }
        }
    }

    future
    //what is returned by the next_generation function is a new matrix called future that has the updated changes to the cells
}

// main function
fn main() {

    // set the dimensions of our 16x16 matrix
    let (rows, cols) = (16, 16);

    // create the grid
    let mut matrix: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    // nw that we have a 16x16 grid full of 0s, we need to fill our initial coordinates with 1's 
    matrix[0][1] = 1;
    matrix[1][2] = 1;
    matrix[2][0] = 1;
    matrix[2][1] = 1;
    matrix[2][2] = 1;

    // To see our starting point, we now need to print the initial state of the grid

    println!("Initial grid:");

    for row in &matrix {

        //this for loop will run 16 times and read the line of values in the row (which is 1s and 0s) 
        //and will concatenate a string full of X or - depending on the value being read

        let mut row_string = String::new();
        //row_str will store the line of characters

            for &cell in row {
                //if the cell in the row is equal to 1, an X will be pushed into row_string, and if the cell is equal to 0, it will push a -. 

                if cell == 1 {
                    row_string.push('X');
                    
                } else {
                    row_string.push('-');
                }
            }
            println!("{}", row_string);
            //this then prints the string
        }


    // compute and print the next generation
    for i in 1..=10 {
        matrix = next_generation(&matrix);
        //this self refernce to the matrix allows us to continue building upon the new iterations of the grid

        println!("Iteration {}:", i);
        for row in &matrix {
            //for each row in the matrix this will create a new string that reads the values of 1s and 0s and writes it as a combination of
            //X's and -'s

            let mut new_row_string = String::new();
            // we will store our row strings into the new variable row_string

            for &cell in row {
                if cell == 1 {
                    new_row_string.push_str("X");
                    //this adds "X" to the string
                } else {
                    new_row_string.push_str("-");
                    //this adds "-" to the string
                }
            }
            println!("{}", new_row_string);
            //this then prints the new string
        }
        println!("");
        //this prints the next line below the current one so that the layers don't overlap
    }
}
