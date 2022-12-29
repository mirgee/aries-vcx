import { VCXInternalError } from '../errors';
import { Connection } from './mediated-connection';
import { VcxBaseNapirs } from './vcx-base-napirs';

export abstract class VCXBaseWithState1<SerializedData, StateType> extends VcxBaseNapirs<SerializedData> {
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

  /**
   * Gets the state of the entity.
   *
   * Example:
   * ```
   * state = await object.getState()
   * ```
   * @returns {Promise<StateType>}
   */
  public async getState(): Promise<StateType> {
    try {
      return await this._getStFn(this.handle);
    } catch (err: any) {
      throw new VCXInternalError(err);
    }
  }
}
