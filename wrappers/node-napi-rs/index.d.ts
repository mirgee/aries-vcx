/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function createAgencyClientForMainWallet(config: string): void
export function provisionCloudAgent(config: string): Promise<string>
export function messagesUpdateStatus(statusCode: string, uidsByConns: string): Promise<void>
export function credentialCreateWithOffer(sourceId: string, offer: string): number
export function credentialRelease(handle: number): void
export function credentialSendRequest(handle: number, handleConnection: number): Promise<void>
export function credentialDeclineOffer(handle: number, handleConnection: number, comment?: string | undefined | null): Promise<void>
export function credentialSerialize(handle: number): string
export function credentialDeserialize(data: string): number
export function v2CredentialUpdateState(handleCredential: number, message: string | undefined | null, connectionHandle: number): Promise<number>
export function credentialGetState(handle: number): number
export function credentialGetOffers(handleConnection: number): Promise<string>
export function credentialGetAttributes(handle: number): string
export function credentialGetAttachment(handle: number): string
export function credentialGetTailsLocation(handle: number): string
export function credentialGetTailsHash(handle: number): string
export function credentialGetRevRegId(handle: number): string
export function credentialGetThreadId(handle: number): string
export function schemaDeserialize(serialized: string): number
export function credentialdefCreateV2(sourceId: string, schemaId: string, tag: string, supportRevocation: boolean): Promise<number>
export function credentialdefPublish(handle: number): Promise<void>
export function credentialdefDeserialize(serialized: string): number
export function credentialdefRelease(handle: number): void
export function credentialdefSerialize(handle: number): string
export function credentialdefGetCredDefId(handle: number): string
export function credentialdefUpdateState(handle: number): Promise<number>
export function credentialdefGetState(handle: number): number
export function disclosedProofCreateWithRequest(sourceId: string, proofReq: string): number
export function disclosedProofRelease(handle: number): void
export function disclosedProofSendProof(handle: number, handleConnection: number): Promise<void>
export function disclosedProofRejectProof(handle: number, handleConnection: number): Promise<void>
export function disclosedProofGetProofMsg(handle: number): string
export function disclosedProofSerialize(handle: number): string
export function disclosedProofDeserialize(data: string): number
export function v2DisclosedProofUpdateState(handle: number, message: string, connectionHandle: number): Promise<number>
export function disclosedProofGetState(handle: number): number
export function disclosedProofGetRequests(handleConnection: number): Promise<string>
export function disclosedProofRetrieveCredentials(handle: number): Promise<string>
export function disclosedProofGetProofRequestAttachment(handle: number): string
export function disclosedProofGenerateProof(handle: number, credentials: string, selfAttestedAttrs: string): Promise<void>
export function disclosedProofDeclinePresentationRequest(handle: number, connectionHandle: number, reason?: string | undefined | null, proposal?: string | undefined | null): Promise<void>
export function disclosedProofGetThreadId(handle: number): string
export function issuerCredentialDeserialize(credentialData: string): number
export function issuerCredentialSerialize(handleCredential: number): string
export function issuerCredentialUpdateStateV2(handleCredential: number, connectionHandle: number): Promise<number>
export function issuerCredentialUpdateStateWithMessageV2(handleCredential: number, connectionHandle: number, message: string): Promise<number>
export function issuerCredentialGetState(handleCredential: number): number
export function issuerCredentialGetRevRegId(handleCredential: number): string
export function issuerCredentialCreate(sourceId: string): number
export function issuerCredentialRevokeLocal(handleCredential: number): Promise<void>
export function issuerCredentialIsRevokable(handleCredential: number): boolean
export function issuerCredentialSendCredential(handleCredential: number, handleConnection: number): Promise<number>
export function issuerCredentialSendOfferV2(handleCredential: number, handleConnection: number): Promise<void>
export function issuerCredentialMarkOfferMsgSent(handleCredential: number): void
export function issuerCredentialBuildOfferMsgV2(credentialHandle: number, credDefHandle: number, revRegHandle: number, credentialJson: string, comment?: string | undefined | null): Promise<void>
export function issuerCredentialGetOfferMsg(credentialHandle: number): string
export function issuerCredentialRelease(credentialHandle: number): void
export function issuerCredentialGetThreadId(credentialHandle: number): string
export function getLedgerAuthorAgreement(): Promise<string>
export function setActiveTxnAuthorAgreementMeta(text: string | undefined | null, version: string | undefined | null, hash: string | undefined | null, accMechType: string, timeOfAcceptance: number): void
export function initDefaultLogger(pattern?: string | undefined | null): void
export function mediatedConnectionGeneratePublicInvite(publicDid: string, label: string): string
export function mediatedConnectionGetPwDid(handle: number): string
export function mediatedConnectionGetTheirPwDid(handle: number): string
export function mediatedConnectionGetThreadId(handle: number): string
export function mediatedConnectionGetState(handle: number): number
export function mediatedConnectionGetSourceId(handle: number): string
export function mediatedConnectionCreate(sourceId: string): Promise<number>
export function mediatedConnectionCreateWithInvite(sourceId: string, details: string): Promise<number>
export function mediatedConnectionCreateWithConnectionRequest(request: string, agentHandle: number): Promise<number>
export function mediatedConnectionSendMessage(handle: number, msg: string): Promise<void>
export function mediatedConnectionCreateWithConnectionRequestV2(request: string, pwInfo: string): Promise<number>
export function mediatedConnectionSendHandshakeReuse(handle: number, oobMsg: string): Promise<void>
export function mediatedConnectionUpdateStateWithMessage(handle: number, message: string): Promise<number>
export function mediatedConnectionHandleMessage(handle: number, message: string): Promise<void>
export function mediatedConnectionUpdateState(handle: number): Promise<number>
export function mediatedConnectionDeleteConnection(handle: number): Promise<void>
export function mediatedConnectionConnect(handle: number): Promise<string | null>
export function mediatedConnectionSerialize(handle: number): string
export function mediatedConnectionDeserialize(connectionData: string): number
export function mediatedConnectionRelease(handle: number): void
export function mediatedConnectionInviteDetails(handle: number): string
export function mediatedConnectionSendPing(handle: number, comment?: string | undefined | null): Promise<void>
export function mediatedConnectionSendDiscoveryFeatures(handle: number, query?: string | undefined | null, comment?: string | undefined | null): Promise<void>
export function mediatedConnectionInfo(handle: number): Promise<string>
export function mediatedConnectionMessagesDownload(connHandles: Array<number>, statusCodes?: string | undefined | null, uids?: string | undefined | null): Promise<string>
export function mediatedConnectionSignData(handle: number, data: Buffer): Promise<Buffer>
export function mediatedConnectionVerifySignature(handle: number, data: Buffer, signature: Buffer): Promise<boolean>
export function outOfBandSenderCreate(config: string): Promise<number>
export function outOfBandReceiverCreate(msg: string): number
export function outOfBandReceiverExtractMessage(handle: number): string
export function outOfBandReceiverConnectionExists(handle: number, connHandles: Array<number>): Promise<number>
export function outOfBandReceiverBuildConnection(handle: number): Promise<string>
export function outOfBandReceiverGetThreadId(handle: number): string
export function outOfBandReceiverSerialize(handle: number): string
export function outOfBandReceiverDeserialize(oobData: string): number
export function outOfBandReceiverRelease(handle: number): void
export function outOfBandSenderCreate(config: string): Promise<number>
export function outOfBandSenderAppendMessage(handle: number, msg: string): void
export function outOfBandSenderAppendService(handle: number, service: string): void
export function outOfBandSenderAppendServiceDid(handle: number, did: string): void
export function outOfBandSenderToMessage(handle: number): string
export function outOfBandSenderGetThreadId(handle: number): string
export function outOfBandSenderSerialize(handle: number): string
export function outOfBandSenderDeserialize(oobData: string): number
export function outOfBandSenderRelease(handle: number): void
export function openMainPool(poolConfig: string): Promise<void>
export function closeMainPool(): Promise<void>
export function credentialCreateWithOffer(sourceId: string, offer: string): number
export function proofCreate(sourceId: string, requestedAttrs: string, requestedPredicates: string, revocationDetails: string, name: string): Promise<number>
export function getProofMsg(handle: number): string
export function proofRelease(handle: number): void
export function proofSendRequest(handleProof: number, handleConnection: number): Promise<void>
export function proofGetRequestMsg(handle: number): string
export function proofSerialize(handle: number): string
export function proofDeserialize(data: string): number
export function v2ProofUpdateState(handleProof: number, connectionHandle: number): Promise<number>
export function v2ProofUpdateStateWithMessage(handleProof: number, message: string, connectionHandle: number): Promise<number>
export function proofGetState(handle: number): number
export function proofGetThreadId(handle: number): Promise<string>
export function markPresentationRequestMsgSent(handle: number): string
export function publicAgentCreate(sourceId: string, institutionDid: string): Promise<number>
export function publicAgentDownloadConnectionRequests(handle: number, uids?: string | undefined | null): Promise<string>
export function publicAgentDownloadMessage(handle: number, uid: string): Promise<string>
export function publicAgentGetService(handle: number): string
export function publicAgentSerialize(handle: number): string
export function publicAgentDeserialize(data: string): number
export function publicAgentRelease(handle: number): void
export function revocationRegistryCreate(config: string): Promise<number>
export function revocationRegistryPublish(handle: number, tailsUrl: string): Promise<number>
export function revocationRegistryPublishRevocations(handle: number): Promise<void>
export function revocationRegistryGetRevRegId(handle: number): string
export function revocationRegistryGetTailsHash(handle: number): string
export function revocationRegistrySerialize(handle: number): string
export function revocationRegistryDeserialize(data: string): number
export function revocationRegistryRelease(handle: number): void
export function schemaGetAttributes(sourceId: string, schemaId: string): void
export function schemaPrepareForEndorser(): void
export function schemaCreate(sourceId: string, name: string, version: string, data: string): Promise<number>
export function schemaGetSchemaId(handleSchema: number): string
export function schemaDeserialize(serialized: string): number
export function schemaSerialize(handleSchema: number): string
export function schemaRelease(handleSchema: number): void
export function schemaUpdateState(handleSchema: number): Promise<number>
export function schemaGetState(handleSchema: number): number
export function enableMocks(): void
export function shutdown(deleteAll?: boolean | undefined | null): void
export function walletOpenAsMain(walletConfig: string): Promise<number>
export function walletCreateMain(walletConfig: string): Promise<void>
export function walletCloseMain(): Promise<void>
export function vcxInitIssuerConfig(config: string): Promise<void>
export function configureIssuerWallet(enterpriseSeed: string): Promise<string>
