export interface VcxErrorInfo {
  vcxErrKind: string;
  vcxErrCode: number;
  vcxErrMessage: string;
}

const VCX_ERR_PREFIX = 'vcx_err_json:';
const VCX_ERR_PREFIX_LENGTH = VCX_ERR_PREFIX.length;

export class VCXInternalErrorNapirs extends Error {
  public readonly vcxError: VcxErrorInfo | undefined;
  public readonly vcxCode: number | undefined;
  public readonly napiCode: string;
  public readonly inheritedStackTraces: string[] = [];

  constructor(err: any) {
    // console.error(`building vcx error from: ${JSON.stringify(err)}`);
    // // @ts-ignore
    //   console.error(`message: ${err.message}`);
    // // @ts-ignore
    //   console.error(`code: ${err.code}`);
    // // @ts-ignore
    //   console.error(`stack: ${err.stack}`);

    const message = err.message || 'foobar';
    super(message);

    if (err instanceof VCXInternalErrorNapirs) {
      this.vcxError = err.vcxError;
      this.vcxCode = err.vcxCode;
      this.napiCode = err.napiCode;
      this.inheritedStackTraces.unshift(...err.inheritedStackTraces);
      return this;
    }
    if (err.stack) {
      this.inheritedStackTraces.push(err.stack);
    }

    if (err.message.startsWith(VCX_ERR_PREFIX)) {
      const vcxErrJson = err.message.slice(VCX_ERR_PREFIX_LENGTH);
      this.vcxError = JSON.parse(vcxErrJson);
      // @ts-ignore
      this.vcxCode = this.vcxError.vcxErrCode;
    } else {
      this.vcxError = undefined;
      this.vcxCode = undefined;
    }
    this.napiCode = err.code;
    return this;
  }
}
