const fs = require('fs');
const readline = require('readline');
const _ = require('lodash');
const { formatAntList } = require('./helpers')

const DEFAULT_ANTS = 10;
const MAX_ITERATIONS = 10000;

export type World = {
    state: State,
    ants: Ants
}

export type Ants = Map<number, Colony>
export type State = Map<Colony, ColonyState>

// Here should be Option
export type Neighbour = string | undefined;
export type Direction = 'north' | 'south' | 'east' | 'west';

export type Colony = string;
export type ColonyState = Record<Direction, Neighbour>;

function loadWorld (numAnts: number, filePath: string): Promise<World> {
    return new Promise((resolve, reject) => {
        const fileStream = fs.createReadStream(filePath);

        const rl = readline.createInterface({
          input: fileStream,
          crlfDelay: Infinity,
        });

        const colonys: Map<Colony, ColonyState> = new Map();
        rl.on('line', (line: string) => {
            const [
                colony,
                ...surrounds
            ] = line.split(' ');

            const neighbours = surrounds.reduce((o, info) => {
                // assumes direction is north/south/east/west
                const [direction, neighbour] = info.split('=');
                o[direction as Direction] = neighbour;
                return o
            }, {} as Record<Direction, string>)

            colonys.set(colony, neighbours)
        });

        rl.on('close', () => {
            const availColonies = Array.from(colonys.keys());
            resolve({
                state: colonys,
                ants: new Map(Array.from(Array(numAnts).keys()).map(key => {
                    const randomIndex = Math.floor(Math.random() * availColonies.length);
                    // more than 1 ant can start at the same colony they will get killed
                    const startingColony = availColonies[randomIndex];
                    return ([key, startingColony])
                }))
            })
        });

        rl.on('error', (error: any) => {
          reject(error);
        });
    })
}

function moveAnts(state: State, ants: Ants) {
    // move ants
    ants.forEach((currentLocation, ant, map) => {
        const locationState = state.get(currentLocation);
        if (locationState) {
            const optionsToMove =  Object.values(locationState).filter((location) => location && state.has(location));
            const randomIndex = Math.floor(Math.random() * optionsToMove.length);
            if (optionsToMove.length > 0) {
                const nextLocation = optionsToMove[randomIndex];
                if (nextLocation) {
                    // console.debug(`Ant: ${ant} moving from ${currentLocation} to ${nextLocation}`);
                    map.set(ant, nextLocation)
                }
            }
        }
    })
}

function getAntWars(ants: Ants): Map<Colony, number[]> {
    const antWars = new Map<Colony, number[]>();
    ants.forEach((currentLocation, ant) => {
        // could make a small optimisation and only store ants
        if (!antWars.has(currentLocation)) {
            antWars.set(currentLocation, new Array());
        }
        antWars.set(currentLocation, [...(antWars.get(currentLocation) ?? []), ant])
    })
    return antWars;
}

function fightWars(state: State, ants: Ants, antWars: Map<Colony, number[]>): void {
    antWars.forEach((war, location, map) => {
        if (war.length > 1) { // there is a war going on remove location
            console.log(`${location} has been destroyed by ${formatAntList(war)}!`)

            // delete ants
            war.forEach((ant: number) => {
                ants.delete(ant)
            })
            // delete location
            state.delete(location)
        } 
    })
}

function writeWorld(state: State, filePath: string): Promise<void> {
    return new Promise((resolve, reject) => {
        // print remaining state to file
        const writeStream = fs.createWriteStream(filePath);

        state.forEach((state, location, map) => {
            // clean state
            Object.keys(state).forEach((direction) => {
                let d = direction as Direction;
                if (!!state[d] && !map.has(state[d] as string)) {
                    delete state[d]
                }
            })

            const formatList = (c: Colony, s: ColonyState) => {
                let r = `${c}`
                if (s.north) {
                    r += ` north=${s.north}`
                }
                if (s.south) {
                    r += ` south=${s.south}`
                }
                if (s.east) {
                    r += ` east=${s.east}`
                }
                if (s.west) {
                    r += ` west=${s.west}`
                }
                return r + `\n`
            }
            writeStream.write(formatList(location, state));
        })

        writeStream.end();

        writeStream.on('finish', () => {
            console.log('World has been written to', filePath);
            resolve()
        });

        writeStream.on('error', (error: any) => {
          reject(error)
        });
    })
}

async function main () {
    const args = process.argv.slice(2);
    if (args.length < 2) {
        console.log("Usage: ts-node ./index.ts <NUM_ANTS> <WORLD_FILE_PATH>")
        process.exit();
    }

    // Other validations on initial input
    // - validate ants against number of colonies but is not a requirement
    const numAnts = Number(args[0]);
    if (Number.isNaN(numAnts)) {
        console.error(`Invalid number of ants: ${args[0]}`)
        process.exit(-1);
    }

    // const filePath = './worlds/hiveum_map_medium.txt';
    const filePath = args[1];

    const { state, ants } = await loadWorld(numAnts, filePath);
    let iteration = 0;
    // after 10k iterations or if all the ants are dead
    while (iteration < MAX_ITERATIONS && ants.size !== 0) { 
        // move ants
        moveAnts(state, ants)

        // find ant wars
        const antWars = getAntWars(ants);

        // see through the wars
        fightWars(state, ants, antWars)

        // inc iteration
        iteration += 1;
    }

    const antsLength = ants.size;
    if (antsLength > 0) {
        console.log(`Reached end of the simulation with ${antsLength} surviving ant${antsLength > 1 ? 's' : ''}`);
    }

    const outPath = './worlds/remainder.txt'
    // write remaining world
    await writeWorld(state, outPath)
}

main()
