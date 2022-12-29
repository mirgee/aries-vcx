import { VCXInternalError } from '../errors';
// import { rustAPI } from '../rustlib';

export const errorMessage = (err: number | Error): string => {
  if (err instanceof VCXInternalError) {
    return err.message;
  }
  if (err instanceof Error) {
    // const message = rustAPI().vcx_error_c_message(VCXCode.UNKNOWN_ERROR);
    return `${err.message}`;
  }
  return `${err}`
};
