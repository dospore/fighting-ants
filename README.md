# Fighting Ants
Full [description](./description.txt) of the task.

## General implemntation idea
Basic notes taken during development.

Initial thoughts
- want to use mappings to have quick lookups as well as O(1) add or removals at a specific index of `Colonies` or `Ants`
- ants dont belong to colonies they just freeroam so dont need to assign them any information besides their current location
- easiest to just use index number for ants id
- ants also want O(1) removal at index
- ants might get stranded on land and land might lose all of its neighbours this is ok
- when removing a `Colony` after a war their will be a floating connection unless you clean up the `ants` previous locations
- more than 1 ant can move to the same `Colony` (more of an assumption) but just going to assume there can also be more than 1 ant involved in a war

Steps;
1. Construct world
The world will contain a mapping from Colony -> ColonyState as well as a list of Ants
```
    type Colony = string

    type ColonyState = {
        north: string | undefined,
        south: string | undefined,
        east: string | undefined,
        west: string | undefined,
    }

    type Ant = {
        location: Colony
    }

    type World {
        state: Map<Colony, ColonyState>,
        ants: Ant[]
    }
```

2. Each round will move the ants randomly.
Available directions are non undefined values, as well truthy values for state.get(direction) (this avoids taking the path to a dangling `Colony`). 
Move an ant, when more than one ant is marked at a location we will remove that location from the `World.state`
// no need to follow the paths back to the ants previous location since we can just remove once we get to it in cleanup / write stage
// This approach has a worse time complexity since at the end I have to loop through M remaining colonies and clean them

3. Continue step two until all the ants are dead or there have been 10k iterations

4. Print out the remaining state of the world
// remember to clean up the dangling `Colonies` if not done in step 2

## TS Solution
```
// install packages
yarn install

// run simulation
yarn simulate <NUM_ANTS> <WORLD_FILE_PATH>

// eg
yarn simulate 100 ./worlds/hiveum_map_medium.txt
```
Remaining output of world gets sent to `./worlds/remainder.txt`


## RS Solution
// TODO this weekend


## Assumptions

### TS Solution
Input file
- north/south/east/west are space seperated
- no directions exist outside of north/south/east/west


World Generation
- Ants are placed randomly amongst the colonies and can start in the same colony
- More than one ant can start in the same place, they dont kill eachother straight away
- There are no Ant teams, they fight for themselves, they have no loyalty
- Isolated colonies are fine


### RS Solution
// TODO
