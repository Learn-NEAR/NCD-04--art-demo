import { logging, RNG, context, PersistentSet } from 'near-sdk-as';
import { generate } from './generate';
import { Design, designs, owners, tempDesigns } from './models';

 
function randomNum(): u32 {
    const rng = new RNG<u32>(1, <u32>context.blockIndex);
    return rng.next()
}

export function claimMyDesign(seed: i32) : void {
    assert(seed >= 0, "Seed needs to be valid.");
    assert(!designs.contains(context.sender), "You can only own one design.")

    let instructions = generate(seed);

    let design = new Design(seed,  instructions);

    logging.log(`\n\n\t> ART / Seed: ${seed} \n\n\t` + instructions.replaceAll("\n", "\n\t") + "\n")

    logging.log("\n\n\tClaimed Art")

    designs.set(context.sender, design);

    owners.add(context.sender);
}

export function viewMyDesign(accountId: string) : Design | null{
    if (designs.contains(accountId)) {
        let design = designs.getSome(accountId);
        logging.log(`\n\n\t> Your Art \n\n\t` + design.instructions.replaceAll("\n", "\n\t") + "\n")
        return design;
    }

    return null
}

export function burnMyDesign() : void {
    assert(designs.contains(context.sender), "No design to burn here.");
    logging.log("\n\n\t Found design")
    designs.delete(context.sender);

    logging.log("\n\n\t Error is not in delete design")

    if (owners.has(context.sender)) {
        owners.delete(context.sender);   
    }

    logging.log(`\n\n\t> Design burned \n\n\t`)
} 

export function getOwners() : number {
    return owners.size;
}

export function design( seed : i32 = 0) : void {
    let a : i32 = 0;
   
    if (seed == 0) {
        a = <i32>randomNum();
        logging.log(`\n\n\tCall claimMyDesign with the seed number ${a} to claim it.\n`)
    } else {
        a = <i32>seed;
    }

    let instructions = generate(a);

    logging.log(`\n\n\t> ART \n\n\t` + instructions.replaceAll("\n", "\n\t") + "\n")

    logging.log('new one')

    if (tempDesigns.contains(context.sender)) {
        tempDesigns.delete(context.sender)
    }
    tempDesigns.set(context.sender, new Design(a, instructions))
}

export function getTempDesign( accountId : string) : Design | null{
    if (tempDesigns.contains(accountId)) {
        logging.log('You already have generated design')
    } else {
        logging.log('You don`t have generated design  yet')
    }

    //assert(tempDesigns.contains(accountId), "No designs generated for  user with such accountId")
    if (tempDesigns.contains(accountId)) {
        return tempDesigns.getSome(accountId)
    }
    return null
}

export function viewDesigns() : void {
    const ownersValues = owners.values();
    let design : Design;
    
    for (let i = 0; i < owners.size; i++) {
        design = designs.getSome(ownersValues[i]);
        logging.log(`\n\n\t> Owner : ${design.owner} \n\n\t` + design.instructions.replaceAll("\n", "\n\t") + "\n")
    }
}


// burn/delete user might want to do it to create new one


