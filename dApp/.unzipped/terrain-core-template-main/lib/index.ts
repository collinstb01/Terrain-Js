import type { Env } from "@terra-money/terrain";
import { MyTerraDappClient } from './clients/MyTerraDappClient';

export class Lib extends MyTerraDappClient {
  env: Env;

  constructor(env: Env) {
    super(env.client, env.defaultWallet, env.refs['my_terra_dapp'].contractAddresses.default);
    this.env = env;
  }
};

export default Lib;
