import * as ffiNapi from 'node-napi-rs';
import { ISerializedData, IssuerStateType } from './common';
import { Connection, IConnectionData } from './mediated-connection';
import { CredentialDef } from './credential-def';
import { RevocationRegistry } from './revocation-registry';
import { VCXBaseWithState1 } from './vcx-base-with-state-1';
import { VCXInternalError1 } from '../errors-1';


/**
 * @description Interface that represents the parameters for `IssuerCredential.create` function.
 * @interface
 */
export interface IIssuerCredentialCreateData {
  // Enterprise's personal identification for the user.
  sourceId: string;
  // Handle of the correspondent credential definition object
  credDefHandle: number;
  // Data attributes offered to person in the credential ('{"state":"UT"}')
  attr: {
    [index: string]: string;
  };
  // Name of the credential - ex. Drivers Licence
  credentialName: string;
  // price of credential
  price: string;
  issuerDid: string;
}

export interface IIssuerCredentialOfferSendData {
  connection: Connection;
  credDef: CredentialDef;
  attr: {
    [index: string]: string;
  };
}

export interface IIssuerCredentialBuildOfferData {
  credDef: CredentialDef;
  attr: {
    [index: string]: string;
  };
  comment: string;
}

export interface IIssuerCredentialBuildOfferDataV2 {
  credDef: CredentialDef;
  revReg?: RevocationRegistry;
  attr: {
    [index: string]: string;
  };
  comment?: string;
}

export interface IIssuerCredentialVCXAttributes {
  [index: string]: string;
}

export interface IIssuerCredentialParams {
  credDefHandle: number;
  credentialName: string;
  attr: IIssuerCredentialVCXAttributes;
  price: string;
}

/**
 * Interface that represents the attributes of an Issuer credential object.
 * This interface is expected as the type for deserialize's parameter and serialize's return value
 */
export interface IIssuerCredentialData {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  issuer_sm: Record<string, any>;
  source_id: string;
}

/**
 * A Credential created by the issuing party (institution)
 */
export class IssuerCredential extends VCXBaseWithState1<IIssuerCredentialData, IssuerStateType> {
  public static async create(sourceId: string): Promise<IssuerCredential> {
    try {
      const connection = new IssuerCredential(sourceId);
      connection._setHandle(await ffiNapi.issuerCredentialCreate(sourceId));
      return connection;
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public static deserialize(
    serializedData: ISerializedData<IIssuerCredentialData>,
  ): IssuerCredential {
    try {
      return super._deserialize(IssuerCredential, serializedData);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  protected _releaseFn = ffiNapi.issuerCredentialRelease;
  protected _updateStFnV2 = ffiNapi.issuerCredentialUpdateStateV2;
  protected _getStFn = ffiNapi.issuerCredentialGetState;
  protected _serializeFn = ffiNapi.issuerCredentialSerialize;
  protected _deserializeFn = ffiNapi.issuerCredentialDeserialize;

  constructor(sourceId: string) {
    super(sourceId);
  }

  public async updateStateWithMessage(connection: Connection, message: string): Promise<number> {
    try {
      return await ffiNapi.issuerCredentialUpdateStateWithMessageV2(
        this.handle,
        connection.handle,
        message,
      );
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async sendOfferV2(connection: Connection): Promise<void> {
    try {
      return await ffiNapi.issuerCredentialSendOfferV2(this.handle, connection.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async markCredentialOfferMsgSent(): Promise<void> {
    try {
      return await ffiNapi.issuerCredentialMarkOfferMsgSent(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async buildCredentialOfferMsgV2({
    credDef,
    attr,
    revReg,
    comment,
  }: IIssuerCredentialBuildOfferDataV2): Promise<void> {
    try {
      return await ffiNapi.issuerCredentialBuildOfferMsgV2(
        this.handle,
        credDef.handle,
        revReg?.handle || 0,
        JSON.stringify(attr),
        comment || '',
      );
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async getCredentialOfferMsg(): Promise<string> {
    try {
      return await ffiNapi.issuerCredentialGetOfferMsg(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async getThreadId(): Promise<string> {
    try {
      return await ffiNapi.issuerCredentialGetThreadId(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async sendCredential(connection: Connection): Promise<number> {
    try {
      return await ffiNapi.issuerCredentialSendCredential(this.handle, connection.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async getCredentialMsg(_myPwDid: string): Promise<string> {
    throw Error('Unimplemented');
  }

  public async revokeCredentialLocal(): Promise<void> {
    try {
      return await ffiNapi.issuerCredentialRevokeLocal(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async isRevokable(): Promise<boolean> {
    try {
      return await ffiNapi.issuerCredentialIsRevokable(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async getRevRegId(): Promise<string> {
    try {
      return await ffiNapi.issuerCredentialGetRevRegId(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  protected _setHandle(handle: number): void {
    super._setHandle(handle);
  }
}
