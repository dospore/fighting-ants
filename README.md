# Fighting Ants

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
