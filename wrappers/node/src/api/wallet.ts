import * as ffiNapi from 'vcx-napi-rs';
import { VCXInternalError } from '../errors';
import * as ffi from '../../../vcx-napi-rs';

export async function createWallet(config: object): Promise<void> {
  try {
    return await ffiNapi.walletCreateMain(JSON.stringify(config));
  } catch (err: any) {
    throw new VCXInternalError(err);
  }
}

export async function configureIssuerWallet(seed: string): Promise<string> {
  try {
    return await ffiNapi.configureIssuerWallet(seed);
  } catch (err: any) {
    throw new VCXInternalError(err);
  }
}

export async function openMainWallet(config: object): Promise<void> {
  try {
    await ffiNapi.walletOpenAsMain(JSON.stringify(config));
  } catch (err: any) {
    throw new VCXInternalError(err);
  }
}

export async function closeMainWallet(): Promise<void> {
  try {
    await ffiNapi.walletCloseMain();
  } catch (err: any) {
    throw new VCXInternalError(err);
  }
}

export class Wallet {
  public static async import(config: string): Promise<void> {
    try {
      return await ffi.walletImport(config);
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }

  public static async export(path: string, backupKey: string): Promise<void> {
    try {
      return await ffi.walletExport(path, backupKey);
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }
}
