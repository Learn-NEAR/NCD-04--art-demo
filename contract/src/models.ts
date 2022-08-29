import { NearBindgen, near } from "near-sdk-js";

@NearBindgen
export class Design {
  owner: string;

  constructor(public instructions: string) {
    this.owner = near.signerAccountId();
  }
}
