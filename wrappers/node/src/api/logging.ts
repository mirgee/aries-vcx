import * as ffiNapi from 'vcx-napi-rs';

import {VCXInternalError} from "../errors";

export function defaultLogger(level: string): void {
  try {
    ffiNapi.initDefaultLogger(level)
  } catch (err: any) {
    throw new VCXInternalError(err);
  }
}
