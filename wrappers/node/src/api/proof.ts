import * as ffi from '../../../node-napi-rs';
import { ISerializedData, VerifierStateType } from './common';
import { Connection } from './mediated-connection';
import { VCXBaseWithState1 } from './vcx-base-with-state-1';
import { VCXInternalErrorNapirs } from '../errors-napirs';

export interface IProofCreateData {
  sourceId: string;
  attrs: IProofAttr[];
  preds: IProofPredicate[];
  name: string;
  revocationInterval: IRevocationInterval;
}

export interface IProofConstructorData {
  attrs: IProofAttr[];
  preds: IProofPredicate[];
  name: string;
}

export interface IProofData {
  source_id: string;
  handle: number;
  requested_attrs: string;
  requested_predicates: string;
  prover_did: string;
  state: number;
  name: string;
  proof_state: ProofState;
  proof: any; // eslint-disable-line @typescript-eslint/no-explicit-any
}

export interface IProofResponses {
  proof?: string;
  proofState: ProofState;
}

export enum ProofFieldType {
  Revealed = 'revealed',
  Unrevealed = 'unrevealed',
  SelfAttested = 'self_attested',
  Predicate = 'predicate',
}

export enum PredicateTypes {
  GE = 'GE',
  LE = 'LE',
  EQ = 'EQ',
}

export interface IProofAttr {
  restrictions?: IFilter[] | IFilter;
  // Requested attribute name
  name?: string;
  // Requested attribute names. Can be used to specify several attributes that have to match a single credential.
  // NOTE: should either be "name" or "names", not both and not none of them.
  names?: string[];
}

export interface IFilter {
  schema_id?: string;
  schema_issuer_did?: string;
  schema_name?: string;
  schema_version?: string;
  issuer_did?: string;
  cred_def_id?: string;
}

export enum ProofState {
  Undefined = 0,
  Verified = 1,
  Invalid = 2,
}

export interface IProofPredicate {
  name: string;
  p_type: string;
  p_value: number;
  restrictions?: IFilter[];
}

export interface IRevocationInterval {
  from?: number;
  to?: number;
}

export class Proof extends VCXBaseWithState1<IProofData, VerifierStateType> {
  public static async create({ sourceId, ...createDataRest }: IProofCreateData): Promise<Proof> {
    try {
      const proof = new Proof(sourceId);
      const handle = await ffi.proofCreate(
        proof.sourceId,
        JSON.stringify(createDataRest.attrs),
        JSON.stringify(createDataRest.preds || []),
        JSON.stringify(createDataRest.revocationInterval),
        createDataRest.name,
      );
      proof._setHandle(handle);
      return proof;
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public static async deserialize(proofData: ISerializedData<IProofData>): Promise<Proof> {
    const params: IProofConstructorData = (() => {
      // todo: clean up this historic piece, we go thin philosophy in wrapper
      switch (proofData.version) {
        case '1.0':
          return Proof.getParams(proofData);
        case '2.0':
          return { attrs: [{ name: '' }], preds: [], name: '' };
        case '3.0':
          return Proof.getParams(proofData);
        default:
          throw Error(
            `Unsupported version provided in serialized proof data: ${JSON.stringify(
              proofData.version,
            )}`,
          );
      }
    })();
    try {
      return super._deserialize<Proof, IProofConstructorData>(Proof, proofData, params);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  protected _releaseFn = ffi.proofRelease;
  protected _updateStFnV2 = ffi.v2ProofUpdateState;
  protected _getStFn = ffi.proofGetState;
  protected _serializeFn = ffi.proofSerialize;
  protected _deserializeFn = ffi.proofDeserialize;

  private static getParams(proofData: ISerializedData<IProofData>): IProofConstructorData {
    const {
      data: { requested_attrs, requested_predicates, name },
    } = proofData;
    const attrs = JSON.parse(requested_attrs);
    const preds = JSON.parse(requested_predicates);
    return { attrs, name, preds };
  }

  public async updateStateWithMessage(connection: Connection, message: string): Promise<number> {
    try {
      return await ffi.v2DisclosedProofUpdateStateWithMessage(
        this.handle,
        message,
        connection.handle,
      );
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async requestProof(connection: Connection): Promise<void> {
    try {
      return ffi.proofSendRequest(this.handle, connection.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public getProofRequestMessage(): string {
    try {
      return ffi.proofGetRequestMsg(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public markPresentationRequestMsgSent(): void {
    try {
      return ffi.markPresentationRequestMsgSent(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async getThreadId(): Promise<string> {
    try {
      return ffi.proofGetThreadId(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public getProof(): Promise<IProofResponses> {
    try {
      const msgs = ffi.proofGetProofMsg(this.handle);
      return JSON.parse(msgs);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }
}
