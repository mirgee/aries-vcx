import * as ffiNapi from 'node-napi-rs';
import { VCXInternalErrorNapirs } from '../errors-napirs';

export function createAgencyClientForMainWallet(config: object): void {
  try {
    ffiNapi.createAgencyClientForMainWallet(JSON.stringify(config));
  } catch (err: any) {
    throw new VCXInternalErrorNapirs(err);
  }
}

export async function initIssuerConfig(config: object): Promise<void> {
  try {
    return await ffiNapi.vcxInitIssuerConfig(JSON.stringify(config));
  } catch (err: any) {
    throw new VCXInternalErrorNapirs(err);
  }
}

export async function openMainPool(config: object): Promise<void> {
  try {
    return await ffiNapi.openMainPool(JSON.stringify(config));
  } catch (err: any) {
    throw new VCXInternalErrorNapirs(err);
  }
}

export function enableMocks(): void {
  return ffiNapi.enableMocks();
}
