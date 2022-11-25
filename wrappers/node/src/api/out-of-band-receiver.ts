import * as ffi from 'node-napi-rs';
import { VCXInternalError1 } from '../errors-1';
import { IOOBSerializedData } from './out-of-band-sender';
import { Connection } from './mediated-connection';
import { VCXBase1 } from './vcx-base-1';
import { ISerializedData } from './common';

export class OutOfBandReceiver extends VCXBase1<IOOBSerializedData> {
  public static createWithMessage(msg: string): OutOfBandReceiver {
    const oob = new OutOfBandReceiver("");
    try {
      oob._setHandle(ffi.outOfBandReceiverCreate(msg))
      return oob;
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public static deserialize(
    data: ISerializedData<IOOBSerializedData>,
  ): OutOfBandReceiver {
    const newObj = { ...data, source_id: 'foo' };
    return super._deserialize(OutOfBandReceiver, newObj);
  }

  public extractMessage(): string {
    try {
      return ffi.outOfBandReceiverExtractMessage(this.handle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async connectionExists(connections: [Connection]): Promise<void | Connection> {
    try {
      const connHandles = connections.map((conn) => conn.handle);
      const connHandle = await ffi.outOfBandReceiverConnectionExists(this.handle, connHandles);
      return connections.find((conn) => conn.handle === connHandle);
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public async buildConnection(): Promise<Connection> {
    try {
      const connection = await ffi.outOfBandReceiverBuildConnection(this.handle);
      return Connection.deserialize(JSON.parse(connection));
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  public getThreadId(): string {
    try {
      return ffi.outOfBandReceiverGetThreadId(this.handle)
    } catch (err: any) {
      throw new VCXInternalError1(err);
    }
  }

  protected _serializeFn = ffi.outOfBandReceiverSerialize;
  protected _deserializeFn = ffi.outOfBandReceiverDeserialize;
  protected _releaseFn = ffi.outOfBandReceiverDeserialize;
}
