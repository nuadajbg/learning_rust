# Project Farthest Point Search

You can start the project at any point in the roadmap.
A git tag of the form `step{i}` where `i` is the step number is provided to easily start at a further step.
For example, to start at step 5, execute :

```bash
git checkout step5
```

To see the fully completed project, execute :

```git
git checkout step24
```

## Step 1 : Create the `Vertex` structure

A structure in Rust is written as :

```rust
struct MyType {
    username: String
}
```

Create a `Vertex` structure for a 3D vertex, with `x`, `y`, `z` coordinates, of type `f64`.

## Step 2 : Read args from command line

Read all the command line arguments into a vector.
Take a look at [std::env::args](https://doc.rust-lang.org/stable/std/env/fn.args.html).

## Step 3 : Load input file - begin

Write a `load_input` function that takes the name of a file as argument (of type `&str`).
This function may fail due to IO errors.
Take a look at [Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html).
When everything succeed, the function shall return a [vector](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of `Vertex`.

## Step 4 : Load input file - open file

In the function `load_input`, open the file with the file name given in argument.
Take a look at [File::open](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open).
Propagate the error produced by `File::open` using the error propagation operator `?`.

## Step 5 : Load input file - iterate on lines

Once the file is open, iterate on all lines in the file with a `for` loop.
Take a look at [BufReader](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html) and [BufRead::lines](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.lines).

## Step 6 : Load input file - iterate on words

Within the loop on all lines, iterate on all words in the line, separated by whitespaces.
Take a look at [str::split_ascii_whitespace](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace).

## Step 7 : Load input file - parse x, y, z

Use an iterator produced by [str::split_ascii_whitespace](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace) to get access to each number on the line and parse each number.
To parse a number from a `&str`, you can do :

```rust
"1.3".parse::<f64>();
```

In the end, you should have the three coordinates `x`, `y` and `z`, as `f64` values.

## Step 8 : Load input file - push vertex on result

Use the parsed `x`, `y` and `z` coordinates to instantiate a `Vertex` for each line and push it on the vector of vertices to be returned.

## Step 9 : Load input file - collect as `Result<Vec<_>, _>`

Up to this point, we used an imperative style with a `for` loop to iterate on the lines and push into a mutable vector of vertices. Now, we want to change that to use the functional style.

Rewrite the `for` loop that iterates on lines so that we directly use the iterator over lines and map each line to something that we can accumulate in a vector using [Iterator::map](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map) and [Iterator::collect](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect).

Note that we do not want to `collect` as a `Vec<Result<Vertex, _>>` but instead as a `Result<Vec<Vertex>, _>`. Take a look at [these examples](https://doc.rust-lang.org/stable/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E).

## Step 10 : Load input file - custom error type

Up to this point we only handled and propagated `std::io::Error`.
Results with other error types were simply handled with `unwrap`, making a hypothesis that everything is fine. In this step we want to properly handle parsing error, se we need a way to propagate both `std::io::Error` and `ParseFloatError` which is produced by `parse`.

* Create an enumeration called `FpsError` that will encapsulate in variants each type of error that may arise from `load_input`.
* Propagate the `ParseFloatError` with the `?` operator on `parse`.
* For the propagation to work we need a way to transform the `std::io::Error` and `ParseFloatError` into `FpsError`. Use [From trait](https://doc.rust-lang.org/stable/std/convert/trait.From.html) implementations.

## Step 11 : Load input file - add missing component error

Now we want to correctly handle the fact that lines in the input file may be malformed and not have enough numbers.
In the present state, a missing number on a line is represented by the fact that the `words` iterator returns an [Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html) on `next`; and we simply `unwrap` the option.

To correctly handle this problem we want to emit a new kind of `FpsError` representing the fact that one component is missing on the line.

Then, we want to propagate this new error when we know that a component is missing.
Take a look at the [`ok_or` method](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.ok_or) on `Option` for this purpose.

## Step 12 : Load input file - implement error trait

Our `FpsError` is nearly there. Now we want to make it look like an error as any other.
In Rust, error types usually implement the [std::error::Error](https://doc.rust-lang.org/stable/std/error/trait.Error.html) trait.
Implement this trait on `FpsError` and all that is required for this to work.

## Step 13 : Load input file - use load function

Our `load_input` function is now complete!

* Use it in `main` to load the vertices using the file name passed as argument to the program.
* Propagate the error produced by `load_input` to the `main` function.

## Step 14 : Implement distance between two points

Before we can implement the FPS algorithm, we need the distance between two vertices.

Implement a `distance` method on `Vertex` to compute the euclidean distance between two vertices.

## Step 15 : FPS - begin

Implement a stub `fps` function that takes as argument a slice of vertices and the number of vertices to select and return a vector of selected vertices.
For the time being :

* Write the function's signature and just return an empty vector
* Call this `fps` function in `main` to obtain a reduced model

## Step 16 : FPS - select first vertex

In `fps`, select a first vertex (named `p0`) from the input vertices and push it onto the resulting vector of selected vertices.

## Step 17 : FPS - compute initial distances

Compute a vector of distances that contains the distance of each vertex to the first selected `p0`.

* Use the `distance` method on `Vertex`
* Use the functional style with `map` and `collect`.

## Step 18 : FPS - compute index of farthest

Using the vector of distances only, compute the index of the vertex farthest from `p0`.

Tips:
* You can iterate on both distances and index at the same time with [enumerate](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.enumerate)
* Look into the [max_by](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.max_by) method on iterators.

## Step 19 : FPS - select second point

Now that we have the index of the vertex that is the farthest from `p0`, let's call this newly selected vertex `p1` and push it onto the vector of selected vertices.

## Step 20 : FPS - update vector of distances

Since we now have two selected vertices `p0` and `p1`, we need to update the vector of distances so that it contains, for each vertex, the shortest distance (min) to either `p0` and `p1`.

Tips:
* You can jointly iterate on vertices and distances with [zip](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.zip).
* You can iterate over mutable references to distances with [iter_mut](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.iter_mut).

## Step 21 : FPS - finalize FPS loop

At the moment, we have written the code for an unrolled loop.
All that is left to do in `fps` is to use a loop to repeat the operations of

* looking for the index of the farthest vertex,
* selecting it (and pushing it onto the vector of selected vertices),
* and updating the vector of distances.

> By construction, the vector of distances will always contain, for each vertex, the shortest distance to any selected vertex. And we always look for the farthest distance in this vector, that is to say the vertex that is the farthest from any previously selected vertex, thus implementing the FPS algorithm.

## Step 22 : Write output to file

Write a `write_output` function to write the vector of selected vertices to a file in the same format as the input file.

## Step 23 : Plot the model before and after

Write a `plot` function to plot the original and reduced models using `gnuplot`.

Look at examples [in gnuplot documentation](https://docs.rs/gnuplot/latest/gnuplot/).
Be sure to use [axes3d](https://docs.rs/gnuplot/latest/gnuplot/struct.Figure.html#method.axes3d).

## Step 24 : All done! Congratulations!
