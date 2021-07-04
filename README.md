# Spinning Switch MoMath Problem

### Problem Statement

Four identical, unlabeled switches are wired in series to a light bulb.  The switches are simple buttons whose state cannot be directly observed, but can be changed by pushing; they are mounted on the corners of a rotatable square.  At any point, you may push, simultaneously, any subset of the buttons, but then an adversary spins the square.  Find a switch-pushing plan that is guaranteed to turn on the bulb in at most some fixed number of steps.


### Solution

First, let's name each state that the square can be in:

`1`: All on.

`-1`: All off.

`D`: Diagonal on.

`A`: Antidiagonal on.

`T`: Top two on.

`B`: Bottom two on.

`L`: Left two on.

`R`: Right two on.

`w`: Top left on.

`x`: Top right on.

`y`: Bottom right on.

`z`: Bottom left on.

`-w`: All but top left on.

`-x`: All but top right on.

`-y`: All but bottom right on.

`-z`: All but bottom left on.


Note that these form a group (let's call it `G`), and that they can be viewed both as states as well as the actions that the player can take.  For example, `T` + `D` = `R`, since the top left is flipped from on to off, the top right stays on, the bottom left stays off, and the bottom right is flipped from off to on.

The adversary, on the other hand, has four operations that he can use (let's call them `O`):
`O_0`, `O_1`, `O_2`, `O_3`, rotate by 0, 90, 180, 270 degrees respectively.

Next, note that there are closed orbits under `O`:

`{1}`

`{-1}`

`{D, A}`

`{T, B, L, R}`

`{w, x, y, z}`

`{-w, -x, -y, -z}`


Since the adversary can always map within these orbits, to solve the puzzle, we have to avoid the adversary being able to create cycles in our strategy.  In particular, we want to rule out starting states by forcing them across the boundaries of these orbits.  

For example, any strategy wins if the initial state was `1`.  If our strategy starts with `-1`, we can rule out the starting state `-1` as well, since we get to apply it before the adversary acts.  To rule out `D` and `A` as starting states, we can use the strategy `-1`, `D`, `-1`, since `1` and `-1` are ruled out as starting states and we have:

```
D --(-1)-> A --O_0/O_2-> A --D-> 1
D --(-1)-> A --O_1/O_3-> D --D-> -1 --O-> -1 --(-1)-> 1
```

And in either case we win.

We've now eliminated the first three orbits as starting states.  

Now, note that any representative element from the other three orbits will map you into that orbit, and then within that orbit the same strategy will work.  That is, we can do

`-1`, `D`, `-1`, `T` to get us into the `{T, B, L, R}` orbit and then repeat 
`-1`, `D`, `-1` to eliminate those as starting options.  And then we can apply `w` to get us into the next orbit, repeat, and then apply `T` to get us from the `w` orbit into the final one.  This leads to the following strategy:

```
-1, D, -1, T, -1, D, -1, w, -1, D, -1, T, -1, D, -1
```

This solves the puzzle.  Running this Rust file (`cargo run` in the repo) will test it for `100000` trials.