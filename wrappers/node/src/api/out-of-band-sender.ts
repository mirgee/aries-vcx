import * as ffi from 'node-napi-rs';
import { VCXInternalError1 } from '../errors-1';
import { VCXBase1 } from './vcx-base-1';
import { ISerializedData } from './common';

export interface IOOBSerializedData {
  source_id: string;
  id: string;
  label?: string;
  goal_code?: string;
  goal?: string;
  accept?: string;
  handshake_protocols?: string;
  requests_attach: string;
}

export interface IOOBCreateData {
  source_id: string;
  label?: string;
  goalCode?: GoalCode;
  goal?: string;
  handshake_protocols?: HandshakeProtocol[];
}

export enum GoalCode {
  IssueVC = 'issue-vc',
  RequestProof = 'request-proof',
  CreateAccount = 'create-account',
  P2PMessaging = 'p2p-messaging',
}

export enum HandshakeProtocol {
  ConnectionV1 = "ConnectionV1",
  DidExchangeV1 = "DidExchangeV1",
}

export class OutOfBandSender extends VCXBase1<IOOBSerializedData> {
  public static async create(config: IOOBCreateData): Promise<OutOfBandSender> {
    const oob = new OutOfBandSender(config.source_id);
    try {
      oob._setHandle(await ffi.outOfBandSenderCreate(JSON.stringify(config)));
      return oob;
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public static deserialize(
    data: ISerializedData<IOOBSerializedData>,
  ): OutOfBandSender {
    const newObj = { ...data, source_id: 'foo' };
    return super._deserialize(OutOfBandSender, newObj);
  }

  public appendMessage(message: string): void {
    try {
      ffi.outOfBandSenderAppendMessage(this.handle, message);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public appendServiceDid(did: string): void {
      try {
        ffi.outOfBandSenderAppendServiceDid(this.handle, did);
      } catch (err: any) {
        throw new VCXInternalError1(err);
      }
  }

  public appendService(service: string): void {
    try {
        ffi.outOfBandSenderAppendService(this.handle, service);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public toMessage(): string {
    try {
      return ffi.outOfBandSenderToMessage(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public getThreadId(): string {
    try {
      return ffi.outOfBandSenderGetThreadId(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  protected _serializeFn = ffi.outOfBandSenderSerialize;
  protected _deserializeFn = ffi.outOfBandSenderDeserialize;
  protected _releaseFn = ffi.outOfBandSenderRelease;
}
