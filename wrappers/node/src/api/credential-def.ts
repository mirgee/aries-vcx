import * as ffi from 'ffi-napi';
import { VCXInternalError } from '../errors';
import { rustAPI } from '../rustlib';
import { createFFICallbackPromise } from '../utils/ffi-helpers';
import { ISerializedData } from './common';
import { VCXBase } from './vcx-base';

export interface ICredentialDefCreateData {
  sourceId: string;
  schemaId: string;
  revocationDetails: IRevocationDetails;
  tailsUrl?: string;
}

export interface ICredentialDefCreateDataV2 {
  sourceId: string;
  schemaId: string;
  supportRevocation: boolean;
  tag: string;
}

export interface ICredentialDefData {
  source_id: string;
  handle: number;
  name: string;
  credential_def: ICredentialDefDataObj;
}

export interface ICredentialDefDataObj {
  ref: number;
  origin: string;
  signature_type: string;
  data: any; // eslint-disable-line @typescript-eslint/no-explicit-any
}

export interface ICredentialDefParams {
  schemaId?: string;
  name?: string;
  credDefId?: string;
  tailsDir?: string;
}

export interface IRevocationDetails {
  maxCreds?: number;
  supportRevocation?: boolean;
  tailsDir?: string;
}

export enum CredentialDefState {
  Built = 0,
  Published = 1,
}

export class CredentialDef extends VCXBase<ICredentialDefData> {
  public static async create({
    supportRevocation,
    schemaId,
    sourceId,
    tag
  }: ICredentialDefCreateDataV2): Promise<CredentialDef> {
    const credentialDef = new CredentialDef(sourceId, { schemaId });
    const commandHandle = 0;
    try {
      await credentialDef._create((cb) =>
        rustAPI().vcx_credentialdef_create_v2(
          commandHandle,
          sourceId,
          schemaId,
          tag,
          supportRevocation,
          cb,
        ),
      );
      return credentialDef;
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }

  public static async deserialize(
    credentialDef: ISerializedData<ICredentialDefData>,
  ): Promise<CredentialDef> {
    // Todo: update the ICredentialDefObj
    const {
      data: { name },
    } = credentialDef;
    const credentialDefParams = {
      name,
      schemaId: null,
    };
    return super._deserialize(CredentialDef, credentialDef, credentialDefParams);
  }

  protected _releaseFn = rustAPI().vcx_credentialdef_release;
  protected _serializeFn = rustAPI().vcx_credentialdef_serialize;
  protected _deserializeFn = rustAPI().vcx_credentialdef_deserialize;
  private _name: string | undefined;
  private _schemaId: string | undefined;
  private _credDefId: string | undefined;
  private _tailsDir: string | undefined;
  private _credDefTransaction: string | null;
  private _revocRegDefTransaction: string | null;
  private _revocRegEntryTransaction: string | null;

  constructor(sourceId: string, { name, schemaId, credDefId, tailsDir }: ICredentialDefParams) {
    super(sourceId);
    this._name = name;
    this._schemaId = schemaId;
    this._credDefId = credDefId;
    this._tailsDir = tailsDir;
    this._credDefTransaction = null;
    this._revocRegDefTransaction = null;
    this._revocRegEntryTransaction = null;
  }

  public async publish(tailsUrl?: string): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_credentialdef_publish(0, this.handle, tailsUrl || null, cb);
          if (rc) {
            reject(rc);
          }
        },
        (resolve, reject) =>
          ffi.Callback(
            'void',
            ['uint32', 'uint32'],
            (handle: number, err: number) => {
              if (err) {
                reject(err);
              }
              resolve();
            },
          ),
      );
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }


  public async getCredDefId(): Promise<string> {
    try {
      const credDefId = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_credentialdef_get_cred_def_id(0, this.handle, cb);
          if (rc) {
            reject(rc);
          }
        },
        (resolve, reject) =>
          ffi.Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xcommandHandle: number, err: number, credDefIdVal: string) => {
              if (err) {
                reject(err);
                return;
              }
              this._credDefId = credDefIdVal;
              resolve(credDefIdVal);
            },
          ),
      );
      return credDefId;
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }

  public async updateState(): Promise<CredentialDefState> {
    try {
      const state = await createFFICallbackPromise<CredentialDefState>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_credentialdef_update_state(0, this.handle, cb);
          if (rc) {
            reject(rc);
          }
        },
        (resolve, reject) =>
          ffi.Callback(
            'void',
            ['uint32', 'uint32', 'uint32'],
            (handle: number, err: number, _state: CredentialDefState) => {
              if (err) {
                reject(err);
              }
              resolve(_state);
            },
          ),
      );
      return state;
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }

  public async getState(): Promise<CredentialDefState> {
    try {
      const stateRes = await createFFICallbackPromise<CredentialDefState>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_credentialdef_get_state(0, this.handle, cb);
          if (rc) {
            reject(rc);
          }
        },
        (resolve, reject) =>
          ffi.Callback(
            'void',
            ['uint32', 'uint32', 'uint32'],
            (handle: number, err: number, state: CredentialDefState) => {
              if (err) {
                reject(err);
              }
              resolve(state);
            },
          ),
      );
      return stateRes;
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }

  get name(): string | undefined {
    return this._name;
  }

  get schemaId(): string | undefined {
    return this._schemaId;
  }

  get credDefId(): string | undefined {
    return this._credDefId;
  }

  get tailsDir(): string | undefined {
    return this._tailsDir;
  }

  protected _setHandle(handle: number): void {
    super._setHandle(handle);
  }

  get credentialDefTransaction(): string | null {
    return this._credDefTransaction;
  }

  get revocRegDefTransaction(): string | null {
    return this._revocRegDefTransaction;
  }

  get revocRegEntryTransaction(): string | null {
    return this._revocRegEntryTransaction;
  }
}
