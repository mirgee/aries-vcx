import * as ffiNapi from 'node-napi-rs';
import * as ref from 'ref-napi';
import { VCXInternalError1 } from '../errors-1';
import { ISerializedData, ConnectionStateType } from './common';
import { VCXBaseWithState1 } from './vcx-base-with-state';
import { IPwInfo } from './utils';

/**
 *   The object of the VCX API representing a pairwise relationship with another identity owner.
 *   Once the relationship, or connection, is established communication can happen securely and privately.
 *   Credentials and Proofs are exchanged using this object.
 *
 *   # States
 *
 *   The set of object states and transitions depends on communication method is used.
 *   The communication method can be specified as config option on one of *_init function.
 *
 *       Inviter:
 *           VcxStateType::VcxStateInitialized - once `vcx_connection_create` (create Connection object) is called.
 *
 *           VcxStateType::VcxStateOfferSent - once `vcx_connection_connect` (prepared Connection invite) is called.
 *
 *           VcxStateType::VcxStateRequestReceived - once `ConnectionRequest` messages is received.
 *                                                   accept `ConnectionRequest` and send `ConnectionResponse` message.
 *                                                   use `vcx_connection_update_state` or `vcx_connection_update_state_with_message` functions for state updates.
 *
 *           VcxStateType::VcxStateAccepted - once `Ack` messages is received.
 *                                            use `vcx_connection_update_state` or `vcx_connection_update_state_with_message` functions for state updates.
 *
 *           VcxStateType::VcxStateNone - once `vcx_connection_delete_connection` (delete Connection object) is called
 *                                           OR
 *                                       `ConnectionProblemReport` messages is received on state updates.
 *
 *       Invitee:
 *           VcxStateType::VcxStateOfferSent - once `vcx_connection_create_with_invite` (create Connection object with invite) is called.
 *
 *           VcxStateType::VcxStateRequestReceived - once `vcx_connection_connect` (accept `ConnectionInvite` and send `ConnectionRequest` message) is called.
 *
 *           VcxStateType::VcxStateAccepted - once `ConnectionResponse` messages is received.
 *                                            send `Ack` message if requested.
 *                                            use `vcx_connection_update_state` or `vcx_connection_update_state_with_message` functions for state updates.
 *
 *           VcxStateType::VcxStateNone - once `vcx_connection_delete_connection` (delete Connection object) is called
 *                                           OR
 *                                       `ConnectionProblemReport` messages is received on state updates.
 *
 *   # Transitions
 *
 *   aries - RFC: https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential
 *       Inviter:
 *           VcxStateType::None - `vcx_connection_create` - VcxStateType::VcxStateInitialized
 *
 *           VcxStateType::VcxStateInitialized - `vcx_connection_connect` - VcxStateType::VcxStateOfferSent
 *
 *           VcxStateType::VcxStateOfferSent - received `ConnectionRequest` - VcxStateType::VcxStateRequestReceived
 *           VcxStateType::VcxStateOfferSent - received `ConnectionProblemReport` - VcxStateType::VcxStateNone
 *
 *           VcxStateType::VcxStateRequestReceived - received `Ack` - VcxStateType::VcxStateAccepted
 *           VcxStateType::VcxStateRequestReceived - received `ConnectionProblemReport` - VcxStateType::VcxStateNone
 *
 *           VcxStateType::VcxStateAccepted - received `Ping`, `PingResponse`, `Query`, `Disclose` - VcxStateType::VcxStateAccepted
 *
 *           any state - `vcx_connection_delete_connection` - VcxStateType::VcxStateNone
 *
 *       Invitee:
 *           VcxStateType::None - `vcx_connection_create_with_invite` - VcxStateType::VcxStateOfferSent
 *
 *           VcxStateType::VcxStateOfferSent - `vcx_connection_connect` - VcxStateType::VcxStateRequestReceived
 *           VcxStateType::VcxStateOfferSent - received `ConnectionProblemReport` - VcxStateType::VcxStateNone
 *
 *           VcxStateType::VcxStateRequestReceived - received `ConnectionResponse` - VcxStateType::VcxStateAccepted
 *           VcxStateType::VcxStateRequestReceived - received `ConnectionProblemReport` - VcxStateType::VcxStateNone
 *
 *           VcxStateType::VcxStateAccepted - received `Ping`, `PingResponse`, `Query`, `Disclose` - VcxStateType::VcxStateAccepted
 *
 *           any state - `vcx_connection_delete_connection` - VcxStateType::VcxStateNone
 *
 *   # Messages
 *
 *       Invitation - https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol#0-invitation-to-connect
 *       ConnectionRequest - https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol#1-connection-request
 *       ConnectionResponse - https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol#2-connection-response
 *       ConnectionProblemReport - https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol#error-message-example
 *       Ack - https://github.com/hyperledger/aries-rfcs/tree/master/features/0015-acks#explicit-acks
 *       Ping - https://github.com/hyperledger/aries-rfcs/tree/master/features/0048-trust-ping#messages
 *       PingResponse - https://github.com/hyperledger/aries-rfcs/tree/master/features/0048-trust-ping#messages
 *       Query - https://github.com/hyperledger/aries-rfcs/tree/master/features/0031-discover-features#query-message-type
 *       Disclose - https://github.com/hyperledger/aries-rfcs/tree/master/features/0031-discover-features#disclose-message-type
 */

/**
 * @description Interface that represents the attributes of a Connection object.
 * This data is expected as the type for deserialize's parameter and serialize's return value
 * @interface
 */
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
  return ref.reinterpret(origPtr, length * ptrType.size)
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
    throw new VCXInternalError1(err);
  }
}

export function generatePublicInvite(public_did: string, label: string): string {
  try {
    return ffiNapi.mediatedConnectionGeneratePublicInvite(public_did, label)
  } catch (err: any) {
    throw new VCXInternalError1(err);
  }
}

/**
 * @class Class representing a Connection
 */
export class Connection extends VCXBaseWithState1<IConnectionData, ConnectionStateType> {
  /**
 * Create a connection object, represents a single endpoint and can be used for sending and receiving
 * credentials and proofs
 *
 * Example:
 * ```
 * source_id = 'foobar123'
 * connection = await Connection.create(source_id)
 * ```
 */
  public static async create({ id }: IConnectionCreateData): Promise<Connection> {
    try {
      const connection = new Connection(id);
      connection._setHandle(await ffiNapi.mediatedConnectionCreate(id))
      return connection;
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Create a connection object with a provided invite, represents a single endpoint and can be used for
   * sending and receiving credentials and proofs.
   * Invite details are provided by the entity offering a connection and generally pulled from a provided QRCode.
   *
   * Example:
   * ```
   * sourceId = 'foobar123'
   * connection_handle = await Connection.createWithInvite({sourceId, inviteDetails})
   * ```
   */
  public static async createWithInvite({ id, invite }: IRecipientInviteInfo): Promise<Connection> {
    try {
      const connection = new Connection(id);
      connection._setHandle(await ffiNapi.mediatedConnectionCreateWithInvite(id, invite))
      return connection;
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public getThreadId(): string {
    try {
      return ffiNapi.mediatedConnectionGetThreadId(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public static async createWithConnectionRequestV2({
    id,
    pwInfo,
    request
  }: IFromRequestInfoV2): Promise<Connection> {
    try {
      const connection = new Connection(id);
      connection._setHandle(await ffiNapi.mediatedConnectionCreateWithConnectionRequestV2(request, JSON.stringify(pwInfo)))
      return connection;
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Create the object from a previously serialized object.
   * Example:
   * data = await connection1.serialize()
   * connection2 = await Connection.deserialize(data)
   */
  public static deserialize(
    connectionData: ISerializedData<IConnectionData>,
  ): Connection {
    try {
      return super._deserialize(Connection, connectionData);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  protected _releaseFn = ffiNapi.mediatedConnectionRelease;
  protected _updateStFn = ffiNapi.mediatedConnectionUpdateState;
  protected _updateStFnV2 = (
    _handle: number,
    _connHandle: number,
  ): number => {
    throw new Error('_updateStFnV2 cannot be called for a Connection object');
  };
  protected _getStFn = ffiNapi.mediatedConnectionGetState;
  protected _serializeFn = ffiNapi.mediatedConnectionSerialize;
  protected _deserializeFn = ffiNapi.mediatedConnectionDeserialize;
  protected _inviteDetailFn = ffiNapi.mediatedConnectionInviteDetails;
  protected _infoFn = ffiNapi.mediatedConnectionInfo;

  /**
   *
   * Updates the state of the connection from the given message.
   *
   * Example:
   * ```
   * await object.updateStateWithMessage(message)
   * ```
   * @returns {Promise<void>}
   */
  public async updateStateWithMessage(message: string): Promise<number> {
    try {
      return await ffiNapi.mediatedConnectionUpdateStateWithMessage(this.handle, message)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   *
   * Answers message if it there's "easy" way to do so (ping, disclose query, handshake-reuse)
   *
   * Example:
   * ```
   * await object.handleMessage(message)
   * ```
   * @returns {Promise<void>}
   */
  public async handleMessage(message: string) {
    try {
      return await ffiNapi.mediatedConnectionHandleMessage(this.handle, message)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }


  /**
   *
   * Communicates with the agent service for polling and setting the state of the entity.
   *
   * Example:
   * ```
   * await object.updateState()
   * ```
   * @returns {Promise<number>}
   */
  public async updateState(): Promise<number> {
    try {
      return await ffiNapi.mediatedConnectionUpdateState(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Delete the object from the agency and release any memory associated with it
   * NOTE: This eliminates the connection and any ability to use it for any communication.
   *
   * Example:
   * ```
   * def connection = await Connection.create(source_id)
   * await connection.delete()
   * ```
   */
  public delete(): void {
    try {
      ffiNapi.mediatedConnectionDeleteConnection(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Creates a connection between enterprise and end user.
   *
   * Example:
   * ```
   * connection = await Connection.create('foobar123')
   * inviteDetails = await connection.connect(
   *     {data: '{"connection_type":"SMS","phone":"5555555555"}',"use_public_did":true})
   * ```
   * @returns {Promise<string}
   */
  public async connect(): Promise<string> {
    try {
      return (await ffiNapi.mediatedConnectionConnect(this.handle) || "{}")
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Sends a message to the connection.
   *
   * Example:
   * ```
   * msg_id = await connection.send_message(
   *     {msg:"are you there?",type:"question","title":"Sending you a question"})
   * ```
   * @returns {Promise<void>}
   */
  public async sendMessage(msgData: IMessageData): Promise<void> {
    const sendMsgOptions = {
      msg_title: msgData.title,
      msg_type: msgData.type,
      ref_msg_id: msgData.refMsgId,
    };
    try {
      return await ffiNapi.mediatedConnectionSendMessage(this.handle, JSON.stringify(sendMsgOptions))
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async sendHandshakeReuse(oobMsg: string): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendHandshakeReuse(this.handle, oobMsg)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Sign data using connection pairwise key.
   *
   * Example:
   * ```
   * signature = await connection.signData(bufferOfBits)
   * ```
   * @returns {Promise<string}
   */
  public async signData(data: Buffer): Promise<Buffer> {
    try {
      return await ffiNapi.mediatedConnectionSignData(this.handle, data)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Verify the signature of the data using connection pairwise key.
   *
   * Example:
   * ```
   * valid = await connection.verifySignature({data: bufferOfBits, signature: signatureBits})
   * ```
   * @returns {Promise<string}
   */
  public async verifySignature(signatureData: ISignatureData): Promise<boolean> {
    try {
      return await ffiNapi.mediatedConnectionVerifySignature(this.handle, signatureData.data, signatureData.data)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Get the invite details that were sent or can be sent to the remote side.
   *
   * Example:
   * ```
   * phoneNumber = '8019119191'
   * connection = await Connection.create('foobar123')
   * inviteDetails = await connection.connect({phone: phoneNumber})
   * inviteDetailsAgain = await connection.inviteDetails()
   * ```
   */
  public inviteDetails(): IConnectionInvite {
    try {
      return ffiNapi.mediatedConnectionInviteDetails(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Send trust ping message to the specified connection to prove that two agents have a functional pairwise channel.
   *
   * Note that this function is useful in case `aries` communication method is used.
   * In other cases it returns ActionNotSupported error.
   */
  public async sendPing(comment: string | null | undefined): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendPing(this.handle, comment)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Send discovery features message to the specified connection to discover which features it supports, and to what extent.
   *
   * Note that this function is useful in case `aries` communication method is used.
   * In other cases it returns ActionNotSupported error.
   */
  public async sendDiscoveryFeatures(
    query: string | null | undefined,
    comment: string | null | undefined,
  ): Promise<void> {
    try {
      return await ffiNapi.mediatedConnectionSendDiscoveryFeatures(this.handle, query, comment)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Retrieves pw_did from Connection object
   */
  public getPwDid(): string {
    try {
      return ffiNapi.mediatedConnectionGetPwDid(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Retrieves their_pw_did from Connection object
   */
  public getTheirDid(): string {
    try {
      return ffiNapi.mediatedConnectionGetTheirPwDid(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  /**
   * Get the information about the connection state.
   *
   * Note: This method can be used for `aries` communication method only.
   *     For other communication method it returns ActionNotSupported error.
   *
   */
  public async info(): Promise<IConnectionInfo> {
    try {
      return await ffiNapi.mediatedConnectionInfo(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async downloadMessages({ status, uids }: IConnectionDownloadMessages): Promise<string> {
    try {
      return await ffiNapi.mediatedConnectionMessagesDownload([this.handle], status, uids)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }
}
