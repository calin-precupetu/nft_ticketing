import { EnvironmentsEnum } from 'types';

export * from './sharedConfig';

export const contractAddress =
  'erd1qqqqqqqqqqqqqpgq0ggr9wnkzcrma6y58ulv3d96m73wqgmw325su43v8s';
export const API_URL = 'https://devnet-template-api.multiversx.com';
export const sampleAuthenticatedDomains = [API_URL];
export const environment = EnvironmentsEnum.devnet;
