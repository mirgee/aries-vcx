import { VCXInternalError } from '../errors';
// import { rustAPI } from '../rustlib';

export const errorMessage = (errorCode: number | Error): string => {
  if (errorCode instanceof VCXInternalError) {
    return errorCode.message;
  }
  if (errorCode instanceof Error) {
    // const message = rustAPI().vcx_error_c_message(VCXCode.UNKNOWN_ERROR);
    return `${errorCode.message}`;
  }
  return `${errorCode}`
};
