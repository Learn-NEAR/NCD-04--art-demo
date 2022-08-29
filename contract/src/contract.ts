import {
  NearBindgen,
  NearContract,
  near,
  call,
  view,
  UnorderedMap,
  UnorderedSet,
  assert,
} from "near-sdk-js";
import { generate } from "./generate";
import { Design } from "./models";

// The @NearBindgen decorator allows this code to compile to Base64.
@NearBindgen
export class Contract extends NearContract {
  designs: UnorderedMap;
  owners: UnorderedSet;

  constructor() {
    //execute the NEAR Contract's constructor
    super();
    this.designs = new UnorderedMap("designs");
    this.owners = new UnorderedSet("owners");
  }

  default() {
    return new Contract();
  }

  // @call indicates that this is a 'change method' or a function
  // that changes state on the blockchain. Change methods cost gas.
  // For more info -> https://docs.near.org/docs/concepts/gas
  @call
  claimMyDesign(seed: number): void {
    assert(seed >= 0, "Seed needs to be valid.");
    assert(
      !this.designs.get(near.signerAccountId()),
      "You can only own one design."
    );

    const instructions = generate(seed);

    const design = new Design(instructions);

    near.log(
      `\n\n\t> ART / Seed: ${seed} \n\n\t ${instructions.replace(
        /\n/g,
        "\n\t"
      )}\n`
    );

    near.log("\n\n\tClaimed Art");

    this.designs.set(near.signerAccountId(), design);
    this.owners.set(near.signerAccountId());
  }

  // @view indicates a 'view method' or a function that returns
  // the current values stored on the blockchain. View calls are free
  // and do not cost gas.
  @view
  viewMyDesign(): void {
    let design = this.designs.get(near.signerAccountId()) as Design;

    near.log(
      `\n\n\t> Your Art \n\n\t${design.instructions.replace(/\n/g, "\n\t")}\n`
    );
  }

  @call
  burnMyDesign(): void {
    assert(
      !!this.designs.get(near.signerAccountId()),
      "No design to burn here."
    );

    this.designs.remove(near.signerAccountId());
    this.owners.remove(near.signerAccountId());

    near.log("\n\n\t> Design burned \n\n\t");
  }

  @view
  design(seed: number = 0): void {
    let instructions = generate(seed);

    near.log(`\n\n\t> ART \n\n\t${instructions.replace(/\n/g, "\n\t")}\n`);
  }

  @view
  viewDesigns(): void {
    const owners = this.owners.elements;

    for (const owner of owners.toArray()) {
      const design = this.designs.get(owner as string) as Design;

      near.log(
        `\n\n\t> Owner : ${owner} \n\n\t${design.instructions.replace(
          /\n/g,
          "\n\t"
        )}\n`
      );
    }
  }
}
