import * as ffiNapi from 'node-napi-rs';

import {VCXInternalErrorNapirs} from "../errors-napirs";

export function defaultLogger(level: string): void {
  try {
    ffiNapi.initDefaultLogger(level)
  } catch (err: any) {
    throw new VCXInternalErrorNapirs(err);
  }
}
