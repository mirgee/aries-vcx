import { VCXInternalError } from '../errors';
import { Connection } from './mediated-connection';
import { VcxBase } from './vcx-base-napirs';

export abstract class VcxBaseWithState<SerializedData, StateType> extends VcxBase<SerializedData> {
  protected abstract _updateStFnV2: (
    handle: number,
    connHandle: number,
  ) => Promise<StateType>;
  protected abstract _getStFn: (handle: number) => StateType;

  public async updateStateV2(connection: Connection): Promise<StateType> {
    try {
      return await this._updateStFnV2(this.handle, connection.handle);
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }

  public async getState(): Promise<StateType> {
    try {
      return await this._getStFn(this.handle);
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }
}

