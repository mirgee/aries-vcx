import * as ffiNapi from 'node-napi-rs';
import * as ref from 'ref-napi';
import { VCXInternalErrorNapirs } from '../errors-napirs';
import { ISerializedData, ConnectionStateType } from './common';
import { VCXBaseWithState1 } from './vcx-base-with-state';
import { IPwInfo } from './utils';

export interface IConnectionData {
  source_id: string;
  invite_detail: string;
  handle: number;
  pw_did: string;
  pw_verkey: string;
  did_endpoint: string;
  endpoint: string;
  uuid: string;
  wallet: string;
  state: ConnectionStateType;
}

/**
 * @description Interface that represents the parameters for `Connection.create` function.
 * @interface
 */
export interface IConnectionCreateData {
  // Institution's personal identification for the connection
  id: string;
}

// A string representing a invitation json object.
export type IConnectionInvite = string;

/**
 * @description Interface that represents the parameters for `Connection.createWithInvite` function.
 * @interface
 */
export interface IRecipientInviteInfo extends IConnectionCreateData {
  // Invitation provided by an entity that wishes to make a connection.
  invite: IConnectionInvite;
}

export interface IFromRequestInfoV2 extends IConnectionCreateData {
  pwInfo: IPwInfo;
  request: string;
}

/**
 * @description Interface that represents the parameters for `Connection.connect` function.
 * @interface
 */
export interface IConnectOptions {
  // Provides details indicating if the connection will be established by text or QR Code
  data: string;
}

/**
 * @description Interface that represents the parameters for `Connection.sendMessage` function.
 * @interface
 */
export interface IMessageData {
  // Actual message to send
  msg: string;
  // Type of message to send. Can be any string
  type: string;
  // Message title (user notification)
  title: string;
  // If responding to a message, id of the message
  refMsgId?: string;
}

/**
 * @description Interface that represents the parameters for `Connection.verifySignature` function.
 * @interface
 */
export interface ISignatureData {
  // Message was signed
  data: Buffer;
  // Generated signature
  signature: Buffer;
}

/**
 * @description A string representing a connection info json object.
 *      {
 *         "current": {
 *             "did": <str>
 *             "recipientKeys": array<str>
 *             "routingKeys": array<str>
 *             "serviceEndpoint": <str>,
 *             "protocols": array<str> -  The set of protocol supported by current side.
 *         },
 *         "remote: { <Option> - details about remote connection side
 *             "did": <str> - DID of remote side
 *             "recipientKeys": array<str> - Recipient keys
 *             "routingKeys": array<str> - Routing keys
 *             "serviceEndpoint": <str> - Endpoint
 *             "protocols": array<str> - The set of protocol supported by side. Is filled after DiscoveryFeatures process was completed.
 *          }
 *    }
 */
export type IConnectionInfo = string;

export function voidPtrToUint8Array(origPtr: Buffer, length: number): Buffer {
  /**
   * Read the contents of the pointer and copy it into a new Buffer
   */
  const ptrType = ref.refType('uint8 *');
  return ref.reinterpret(origPtr, length * ptrType.size);
}

export interface IDownloadMessagesConfigsV2 {
  connections: [Connection];
  status: string;
  uids: string;
}

export interface IConnectionDownloadMessages {
  status: string;
  uids: string;
}

export interface IConnectionDownloadAllMessages extends IConnectionDownloadMessages {
  pwdids: string;
}

export async function downloadMessagesV2({
  connections,
  status,
  uids,
}: IDownloadMessagesConfigsV2): Promise<string> {
  try {
    const handles = connections.map((connection) => connection.handle);
    return await ffiNapi.mediatedConnectionMessagesDownload(handles, status, uids);
  } catch (err: any) {
    throw new VCXInternalErrorNapirs(err);
  }
}

export function generatePublicInvite(public_did: string, label: string): string {
  try {
    return ffiNapi.mediatedConnectionGeneratePublicInvite(public_did, label);
  } catch (err: any) {
    throw new VCXInternalErrorNapirs(err);
  }
}

export class Connection extends VCXBaseWithState1<IConnectionData, ConnectionStateType> {
  public static async create({ id }: IConnectionCreateData): Promise<Connection> {
    try {
      const connection = new Connection(id);
      connection._setHandle(await ffiNapi.mediatedConnectionCreate(id));
      return connection;
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public static async createWithInvite({ id, invite }: IRecipientInviteInfo): Promise<Connection> {
    try {
      const connection = new Connection(id);
      connection._setHandle(await ffiNapi.mediatedConnectionCreateWithInvite(id, invite));
      return connection;
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public getThreadId(): string {
    try {
      return ffiNapi.mediatedConnectionGetThreadId(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public static async createWithConnectionRequestV2({
    id,
    pwInfo,
    request,
  }: IFromRequestInfoV2): Promise<Connection> {
    try {
      const connection = new Connection(id);
      connection._setHandle(
        await ffiNapi.mediatedConnectionCreateWithConnectionRequestV2(
          request,
          JSON.stringify(pwInfo),
        ),
      );
      return connection;
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public static deserialize(connectionData: ISerializedData<IConnectionData>): Connection {
    try {
      return super._deserialize(Connection, connectionData);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  protected _releaseFn = ffiNapi.mediatedConnectionRelease;
  protected _updateStFn = ffiNapi.mediatedConnectionUpdateState;
  protected _updateStFnV2 = async (_handle: number, _connHandle: number): Promise<number> => {
    throw new Error('_updateStFnV2 cannot be called for a Connection object');
  };
  protected _getStFn = ffiNapi.mediatedConnectionGetState;
  protected _serializeFn = ffiNapi.mediatedConnectionSerialize;
  protected _deserializeFn = ffiNapi.mediatedConnectionDeserialize;
  protected _inviteDetailFn = ffiNapi.mediatedConnectionInviteDetails;
  protected _infoFn = ffiNapi.mediatedConnectionInfo;

  public async updateStateWithMessage(message: string): Promise<number> {
    try {
      return await ffiNapi.mediatedConnectionUpdateStateWithMessage(this.handle, message);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async handleMessage(message: string) {
    try {
      return await ffiNapi.mediatedConnectionHandleMessage(this.handle, message);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async updateState(): Promise<number> {
    try {
      return await ffiNapi.mediatedConnectionUpdateState(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async delete(): Promise<void> {
    try {
      await ffiNapi.mediatedConnectionDeleteConnection(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async connect(): Promise<string> {
    try {
      return (await ffiNapi.mediatedConnectionConnect(this.handle)) || '{}';
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async sendMessage(msgData: IMessageData): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendMessage(
        this.handle,
        msgData.msg,
      );
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async sendHandshakeReuse(oobMsg: string): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendHandshakeReuse(this.handle, oobMsg);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async signData(data: Buffer): Promise<Buffer> {
    try {
      return await ffiNapi.mediatedConnectionSignData(this.handle, data);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async verifySignature(signatureData: ISignatureData): Promise<boolean> {
    try {
      return await ffiNapi.mediatedConnectionVerifySignature(
        this.handle,
        signatureData.data,
        signatureData.data,
      );
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public inviteDetails(): IConnectionInvite {
    try {
      return ffiNapi.mediatedConnectionInviteDetails(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async sendPing(comment: string | null | undefined): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendPing(this.handle, comment);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async sendDiscoveryFeatures(
    query: string | null | undefined,
    comment: string | null | undefined,
  ): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendDiscoveryFeatures(this.handle, query, comment);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public getPwDid(): string {
    try {
      return ffiNapi.mediatedConnectionGetPwDid(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public getTheirDid(): string {
    try {
      return ffiNapi.mediatedConnectionGetTheirPwDid(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async info(): Promise<IConnectionInfo> {
    try {
      return await ffiNapi.mediatedConnectionInfo(this.handle);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }

  public async downloadMessages({ status, uids }: IConnectionDownloadMessages): Promise<string> {
    try {
      return await ffiNapi.mediatedConnectionMessagesDownload([this.handle], status, uids);
    } catch (err: any) {
      throw new VCXInternalErrorNapirs(err);
    }
  }
}
