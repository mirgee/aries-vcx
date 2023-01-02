import * as ffiNapi from 'node-napi-rs';

import {VCXInternalError} from "../errors";

export function defaultLogger(level: string): void {
  try {
    ffiNapi.initDefaultLogger(level)
  } catch (err: any) {
    throw new VCXInternalError(err);
  }
}
