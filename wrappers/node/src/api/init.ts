import * as ffiNapi from 'node-napi-rs';
// import { Callback } from 'ffi-napi';

import { VCXInternalError } from '../errors';
// import { rustAPI } from '../rustlib';
// import { createFFICallbackPromise } from '../utils/ffi-helpers';
import { VCXInternalErrorNapirs } from '../errors-napirs';

// export function initThreadpool(config: object) {
//   const rc = rustAPI().vcx_init_threadpool(JSON.stringify(config));
//   if (rc !== 0) {
//     throw new VCXInternalError(rc);
//   }
// }

export function createAgencyClientForMainWallet(config: object): void {
  try {
    ffiNapi.createAgencyClientForMainWallet(JSON.stringify(config));
  } catch (err: any) {
    throw new VCXInternalError(err);
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
