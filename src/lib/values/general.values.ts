import { astar, moonbaseAlpha, moonbeam } from 'viem/chains';

export const Environments = {
  dev: 'development',
  stg: 'staging',
  prod: 'production',
};

export const chains = [moonbaseAlpha, moonbeam, astar];
