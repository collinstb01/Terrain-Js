import type { Env } from "@terra-money/terrain";
import { DAppClient } from './clients/DAppClient';

export class Lib extends DAppClient {
  env: Env;

  constructor(env: Env) {
    super(env.client, env.defaultWallet, env.refs['dApp'].contractAddresses.default);
    this.env = env;
  }
};

export default Lib;
