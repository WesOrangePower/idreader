#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum APDUResponse {
    ClassIsNotSupported = 0x0600, // 6 Class not supported. (ERROR)

    // 61 - Response bytes still available (INFORMATION)
    CommandSuccess = 0x6100, // 61 Command successfully executed; ‘XX’ bytes of data are available and can be requested using GET RESPONSE.

    // 62 - State of non-volatile memory unchanged. (WARN)
    NoInformationGivenNVRAMUnchanged = 0x6200, // 62 00 No information given (NV-Ram not changed)
    NVRamNotChanged = 0x6201,                  // 62 01 NV-Ram not changed 1.
    ReturnedDataMayBeCorrupted = 0x6281,       // 62 81 Part of returned data may be corrupted.
    EOFReachedBeforeReadingLeBytes = 0x6282, // 62 82 End of file/record reached before reading Le bytes.
    SelectedFileInvalidated = 0x6283,        // 62 83  Selected file invalidated. (WARNING)
    SelectedFileNotValid = 0x6284, // 62 84 Selected file is not valid. FCI not formated according to ISO.
    NoInputFromCardSensor = 0x6285, // 62 85 No input data available from a sensor on the card. No Purse Engine enslaved for R3bc
    WrongRMac = 0x62A2,             // 62 A2 OR 62 F1 Wrong R-MAC.
    CardLocked = 0x62A4,            // 62 A4 Card locked (during reset()).
    CounterWithValueSW2_2 = 0x62C0, // 62 Cx Counter with value x (command dependent).
    InternalReset = 0x62F3,         // 62 F3 Internal Reset.
    DefaultAgentLocked = 0x62F5,    // 62 F5 Default agent locked.
    CardholderLocked = 0x62F7,      // 62 F7 Cardholder locked.
    BasementIsCurrentAgent = 0x62F8, // 62 F8 Basement is current agent.
    CALCKeySetNotUnblocked = 0x62F9, // 62 F9 CALC Key Set not unblocked.

    // 63 - State of non-volatile memory changed (WARN)
    NoInformationGivenNVRAMChanged = 0x6300, // 63 00 No information given (NV-Ram changed)
    FileFilledUpByTheLastWrite = 0x6381, // 63 81 File filled up by the last write. Loading/updating is not allowed.
    CardKeyNotSupported = 0x6382,        // 63 82 Card key not supported.
    ReaderKeyNotSupported = 0x6383,      // 63 83 Reader key not supported.
    PlaintextTransmissionNotSupported = 0x6384, // 63 84 Plaintext transmission not supported.
    SecuredTransmissionNotSupported = 0x6385, // 63 85 Secured transmission not supported.
    VolatileMemoryNotAvailable = 0x6386, // 63 86 Volatile memory is not available.
    NonVolatileMemoryNotAvailable = 0x6387, // 63 87 Non-volatile memory is not available.
    KeyNumberInvalid = 0x6388,           // 63 88 Key number not valid.
    KeyLengthInvalid = 0x6389,           // 63 89 Key length is not correct.
    VerifyFailNoRetries = 0x63C0,        // 63 C0 Verify fail, no try left.
    VerifyFail1retry = 0x63C1,           // 63 C1 Verify fail, 1 try left.
    VerifyFail2retries = 0x63C2,         // 63 C2 Verify fail, 2 tries left.
    VerifyFail3retries = 0x63C3,         // 63 C3 Verify fail, 3 tries left.
    CounterReachedValueSW2_2 = 0x63CF, // 63 CX The counter has reached the value ‘x’ (0 = x = 15) (command dependent).
    MoreDataExpected = 0x63F1,         // 63 F1 More data expected.
    MoreDataExpectedCommandPending = 0x63F2, // 63 F2 More data expected and proactive command pending.

    // 64 - State of non-volatile memory unchanged (ERROR)
    CommandTimeout = 0x6401, // 64 01 Command timeout. Immediate response required by the card.

    // 65 - State of non-volatile memory changed
    NoInformationGivenStateOfNVRAMChanged = 0x6500, // 65 00 No information given
    WriteErrorMemFailure = 0x6501, // 65 01 Write error. Memory failure. There have been problems in writing or reading the EEPROM. Other hardware problems may also bring this error.
    MemFailure = 0x6581,           // 65 81 Memory failure
    ReceivingTimeout = 0x6600,     // 66 00 Error while receiving (timeout)
    ReceivingCharParityError = 0x6601, // 66 01 Error while receiving (character parity error)
    WrongChecksum = 0x6602,        // 66 02 Wrong checksum
    CurrentDFWithoutFC = 0x6603,   // 66 03 The current DF file without FC
    NoSFOrKFUnderCurrentDF = 0x6604, // 66 04 No SF or KF under the current DF
    IncorrectEncryptionDecryptionPadding = 0x6669, // 66 69 Incorrect Encryption/Decryption Padding

    // 67 XX length incorrect (procedure)(ISO 7816-3)
    WrongLength = 0x6700, // 67 00 Wrong length

    // 68 - Functions in CLA not supported
    NoInformationGivenOperationNotSupported = 0x6800, // 68 00 No information given (The request function is not supported by the card)
    LogicalChannelNotSupported = 0x6881,              // 68 81 Logical channel not supported
    SecureMessagingNotSupported = 0x6882,             // 68 82 Secure messaging not supported
    LastCommandOfTheChainExpected = 0x6883,           // 68 83 Last command of the chain expected
    CommandChainingNotSupported = 0x6884,             // 68 84 Command chaining not supported

    // 69 - Command not allowed
    NoInformationGivenCommandNotAllowed = 0x6900, // 69 00 No information given (Command not allowed)
    CommandNotAcceptedInInactiveState = 0x6901,   // 69 01 Command not accepted (inactive state)
    CommandIncompatibleWithFileStructure = 0x6981, // 69 81 Command incompatible with file structure
    SecurityConditionNotSatisfied = 0x6982,       // 69 82 Security condition not satisfied.
    AuthenticationMethodBlocked = 0x6983,         // 69 83 Authentication method blocked
    ReferencedDataReversiblyBlocked = 0x6984, // 69 84 Referenced data reversibly blocked (invalidated)
    ConditionsOfUseNotSatisfied = 0x6985,     // 69 85 Conditions of use not satisfied.
    CommandNotAllowed = 0x6986,               // 69 86 Command not allowed (no current EF)
    ExpectedSecureMessagingObjectMissing = 0x6987, // 69 87 Expected secure messaging (SM) object missing
    IncorrectSecureMessagingDataObject = 0x6988, // 69 88 Incorrect secure messaging (SM) data object
    Reserved = 0x698D,                           // 69 8D Reserved
    DataMustBeUpdatedAgain = 0x6996,             // 69 96 Data must be updated again
    BlockedByPOL1 = 0x69E1, // 69 E1 POL1 of the currently Enabled Profile prevents this action.
    PermissionDenied = 0x69F0, // 69 F0 Permission Denied
    PermissionDeniedMissingPrivilege = 0x69F1, // 69 F1 Permission Denied &#8211; Missing Privilege

    // 6A - Wrong parameter(s) P1-P2
    NoInformationGivenP1P2Incorrect = 0x6A00, // 6A 00 No information given (Bytes P1 and/or P2 are incorrect)
    DataFieldIncorrect = 0x6A80, // 6A 80 The parameters in the data field are incorrect.
    FunctionNotSupported = 0x6A81, // 6A 81 Function not supported
    FileNotFound = 0x6A82,       // 6A 82 File not found
    RecordNotFound = 0x6A83,     // 6A 83 Record not found
    InsufficientMemorySpaceInRecordOrFile = 0x6A84, // 6A 84 There is insufficient memory space in record or file
    LCInconsistentWithTLVStructure = 0x6A85,        // 6A 85 Lc inconsistent with TLV structure
    IncorrectP1OrP2 = 0x6A86,                       // 6A 86 Incorrect P1 or P2 parameter.
    LCInconsistentWithP1P2 = 0x6A87,                // 6A 87 Lc inconsistent with P1-P2
    ReferencedDataNotFound = 0x6A88,                // 6A 88 Referenced data not found
    FileAlreadyExists = 0x6A89,                     // 6A 89 File already exists
    DFNameAlreadyExists = 0x6A8A,                   // 6A 8A DF name already exists.
    WrongParameterValue = 0x6AF0,                   // 6A F0 Wrong parameter value

    WrongParametersP1P2 = 0x6B00, // 6B 00 Wrong parameter(s) P1-P2

    // 6C - Wrong length Le
    IncorrectP3Length = 0x6C00,   // 6C 00 Incorrect P3 length.
    IncorrectLengthInLe = 0x6CFF, // 6C XX Bad length value in Le; &#8216;xx&#8217; is the correct exact Le
    InstructionCodeNotSupportedOrInvalid = 0x6D00, // 6D 00 Instruction code not supported or invalid
    ClassNotSupported = 0x6E00,                    // 6E 00 Class not supported
    // 6F - Internal exception
    CommandAborted = 0x6F00, // 6F 00 Command aborted &#8211; more exact diagnosis not possible (e.g., operating system error).
    CardDead = 0x6FFF,       // 6F FF Card dead (overuse, …)
    Ok = 0x9000, // 90 00 Command successfully executed (OK).
    PinInvalid3OrMoreTries = 0x9004, // 90 04 PIN not succesfully verified, 3 or more PIN tries left
    KeyOrFileNotFound = 0x9008, // 90 08 Key/file not found 90
    UnblockTryCounterReachedZero = 0x8000, // 80 Unblock Try Counter has reached zero
    OK = 0x9100,             // 91 00 OK
    ActivityLockOrLockableHasWrongValue = 0x9101, // 91 01 States.activity, States.lock Status or States.lockable has wrong value
    TransactionNumberReachedLimit = 0x9102,       // 91 02 Transaction number reached its limit
    NoChanges = 0x910C,                           // 91 0C No changes
    InsufficientNVRAMToCompleteCommand = 0x910E, // 91 0E Insufficient NV-Memory to complete command
    CommandCoreNotSupported = 0x911C,            // 91 1C Command code not supported
    CRCOrMACDoesNotMatchData = 0x911E,           // 91 1E CRC or MAC does not match data
    InvalidKeyNumberSpecified = 0x9140,          // 91 40 Invalid key number specified
    LengthOFCommandStringInvalid = 0x917E,       // 91 7E Length of command string invalid
    RequestedCommandNotAllowed = 0x919D,         // 91 9D Not allow the requested command
    InvalidValueOfTheParameter = 0x919E,         // 91 9E Value of the parameter invalid
    RequestedAIDNotPresentOnPICC = 0x91A0,       // 91 A0 Requested AID not present on PICC
    UnrecoverableErrorWithinApplication = 0x91A1, // 91 A1 Unrecoverable error within application
    AuthenticationStatusDoesNotAllowRequestedCommand = 0x91AE, // 91 AE Authentication status does not allow the requested command
    AdditionalDataFrameIsExpectedToBeSent = 0x91AF, // 91 AF Additional data frame is expected to be sent
    OutOfBoundary = 0x91BE,                         // 91 BE Out of boundary
    UnrecoverableErrorWithinPICC = 0x91C1,          // 91 C1 Unrecoverable error within PICC
    PreviousCommandWasNotCompleted = 0x91CA, // 91 CA Previous Command was not fully completed
    PICCWasDisabledByUnrecoverableError = 0x91CD, // 91 CD PICC was disabled by an unrecoverable error
    NumberOfApplicationsIsLimitedTo28 = 0x91CE,   // 91 CE Number of Applications limited to 28
    FileOrApplicationExists = 0x91DE,             // 91 DE File or application already exists
    CouldNotCompleteNVWriteOperationDueToLossOfPower = 0x91EE, // 91 EE Could not complete NV-write operation due to loss of power
    FileNumberDoesNotExist = 0x91F0, // 91 F0 Specified file number does not exist
    UnrecoverableErrorWithinFile = 0x91F1, // 91 F1 Unrecoverable error within file
    WritingToEEPROMSuccessful = 0x9200, // 92 0x Writing to EEPROM successful after &#8216;x&#8217; attempts.
    OutOfStorage = 0x9210,              // 92 10 Insufficient memory. No more storage available.
    WritingToEEPROMNotSuccessful = 0x9240, // 92 40 Writing to EEPROM not successful.
    IntegrityError = 0x9301,            // 93 01 Integrity error
    CandidateS2Invalid = 0x9302,        // 93 02 Candidate S2 invalid
    ApplicationIsPermanentlyLocked = 0x9303, // 93 03 Application is permanently locked
    NoEFSelected = 0x9400,              // 94 00 No EF selected.
    CandidateCurrencyCodeDoesNotMatchPurseCurrency = 0x9401, // 94 01 Candidate currency code does not match purse currency
    CadidateAmountTooHigh = 0x9402,                          // 94 02 Candidate amount too high
    // AddressRangeExceeded = 0x9402, // 94 02 Address range exceeded.
    CandidateAmountTooLow = 0x9403, // 94 03 Candidate amount too low
    FIDOrRecordOrComparisonPatternNotFound = 0x9404, // 94 04 FID not found, record not found or comparison pattern not found.
    ProblemsInDataField = 0x9405,                    // 94 05 Problems in the data field
    RequiredMACUnavailable = 0x9406,                 // 94 06 Required MAC unavailable
    BadCurrency = 0x9407, // 94 07 Bad currency : purse engine has no slot with R3bc currency
    CurrencyNotSupported = 0x9408, // 94 08 R3bc currency not supported in purse engine
    // SelectedFileTypeDoesNotMatchCommand = 0x9408, // 94 08 Selected file type does not match command.
    BadSequence = 0x9580,                               // 95 80 Bad sequence
    SlaveNotFound = 0x9681,                             // 96 81 Slave not found
    PinBlockedAndTryCounterIs2or1 = 0x9700, // 97 00 PIN blocked and Unblock Try Counter is 1 or 2
    MainKeysAreBlocked = 0x9702,            // 97 02 Main keys are blocked
    PinVerificationUnsuccessful3orMoreRetries = 0x9704, // 97 04 PIN not succesfully verified, 3 or more PIN tries left
    BaseKey = 0x9784,                                   // 97 84 Base key
    CMACLimitExceeded = 0x9785,                         // 97 85 Limit exceeded &#8211; C-MAC key
    RMACLimitExceeded = 0x9786, // 97 86 SM error &#8211; Limit exceeded &#8211; R-MAC key
    SequenceCounterLimitExceeded = 0x9787, // 97 87 Limit exceeded &#8211; sequence counter
    RMACLengthLimitExceeded = 0x9788, // 97 88 Limit exceeded &#8211; R-MAC length
    ServiceNotAvailable = 0x9789, // 97 89 Service not available
    NoPinDefined = 0x9802,      // 98 02 No PIN defined.
    AuthenticationFailed = 0x9804, // 98 04 Access conditions not satisfied, authentication failed.
    ASKRANDOMOrGIVERANDOMNotSatisfied = 0x9835, // 98 35 ASK RANDOM or GIVE RANDOM not executed.
    PinVerificationUnsuccessful = 0x9840, // 98 40 PIN verification not successful.
    INCREASEOrDECREASENotExecutedLimitReached = 0x9850, // 98 50 INCREASE or DECREASE could not be executed because a limit has been reached.
    AuthenticationError = 0x9862, // 98 62 Authentication Error, application specific (incorrect MAC)
    OnePinRetryLeft = 0x9900,     // 99 00 1 PIN try left
    PinUnsuccessfullyVerified1Retry = 0x9904, // 99 04 PIN not succesfully verified, 1 PIN try left
    WrongStatusCardholderLock = 0x9985, // 99 85 Wrong status &#8211; Cardholder lock
    MissingPrivilege = 0x9986,    // 99 86 Missing privilege
    PinNotInstalled = 0x9987,     // 99 87 PIN is not installed
    WrongStatusRMACState = 0x9988, // 99 88 Wrong status &#8211; R-MAC state
    TwoPinRetryLeft = 0x9A00,     // 9A 00 2 PIN try left
    PinUnsuccessfullyVerified2Retries = 0x9A04, // 9A 04 PIN not succesfully verified, 2 PIN try left
    WrongParameterValueDoubleAgentAID = 0x9A71, // 9A 71 Wrong parameter value &#8211; Double agent AID
    WrongParameterValueDoubleAgentType = 0x9A72, // 9A 72 Wrong parameter value &#8211; Double agent Type
    IncorrectCertificateType = 0x9D05,           // 9D 05 Incorrect certificate type
    IncorrectSessionDataSize = 0x9D07,           // 9D 07 Incorrect session data size
    IncorrectDIRFileOrRecordSize = 0x9D08,       // 9D 08 Incorrect DIR file record size
    IncorrectFCIRecordSize = 0x9D09,             // 9D 09 Incorrect FCI record size
    IncorrectCodeSize = 0x9D0A,                  // 9D 0A Incorrect code size
    InsufficientMemoryToLoadApplication = 0x9D10, // 9D 10 Insufficient memory to load application
    InvalidAID = 0x9D11,                         // 9D 11 Invalid AID
    DuplicateAID = 0x9D12,                       // 9D 12 Duplicate AID
    ApplicationPreviouslyLoaded = 0x9D13,        // 9D 13 Application previously loaded
    ApplicationHistoryListFull = 0x9D14,         // 9D 14 Application history list full
    ApplicationNotOpen = 0x9D15,                 // 9D 15 Application not open
    InvalidOffset = 0x9D17,                      // 9D 17 Invalid offset
    ApplicationAlreadyLoaded = 0x9D18,           // 9D 18 Application already loaded
    InvalidCertificate = 0x9D19,                 // 9D 19 Invalid certificate
    InvalidSignature = 0x9D1A,                   // 9D 1A Invalid signature
    InvalidKTU = 0x9D1B,                         // 9D 1B Invalid KTU
    MSMControlsNotSet = 0x9D1D,                  // 9D 1D MSM controls not set
    ApplicationSignatureDoesNotExist = 0x9D1E,   // 9D 1E Application signature does not exist
    KTUDoesNotExist = 0x9D1F,                    // 9D 1F KTU does not exist
    ApplicationNotLoaded = 0x9D20,               // 9D 20 Application not loaded
    InvalidOpenCommandDataLength = 0x9D21,       // 9D 21 Invalid Open command data length
    CheckDataParameterIsIncorrectInvalidStartAddress = 0x9D30, // 9D 30 Check data parameter is incorrect (invalid start address)
    CheckDataParameterIsIncorrectInvalidLength = 0x9D31, // 9D 31 Check data parameter is incorrect (invalid length)
    CheckDataParameterIsIncorrectCheckDataParameterIncorrect = 0x9D32, // 9D 32 Check data parameter is incorrect (illegal memory check area)
    InvalidMSMControlsCiphertext = 0x9D40, // 9D 40 Invalid MSM Controls ciphertext
    MSMControlsAlreadySet = 0x9D41,        // 9D 41 MSM controls already set
    MSMControlsDataLengthLessThan2B = 0x9D42, // 9D 42 Set MSM Controls data length less than 2 bytes
    InvalidMSMControlsDataLength = 0x9D43,    // 9D 43 Invalid MSM Controls data length
    ExcessMSMControlsCiphertext = 0x9D44,     // 9D 44 Excess MSM Controls ciphertext
    VerificationOfMSMControlsDataFailed = 0x9D45, // 9D 45 Verification of MSM Controls data failed
    InvalidMCDIssuerProductionID = 0x9D50,    // 9D 50 Invalid MCD Issuer production ID
    InvalidMCDIssuerID = 0x9D51,              // 9D 51 Invalid MCD Issuer ID
    InvalidSetMSMControlsDataDate = 0x9D52,   // 9D 52 Invalid set MSM controls data date
    InvalidMCDNumber = 0x9D53,                // 9D 53 Invalid MCD number
    MacVerificationFailed = 0x9D60,           // 9D 60 MAC verification failed
    MaximumNumberOfUnblocksReached = 0x9D61,  // 9D 61 Maximum number of unblocks reached
    CardWasNotBlocked = 0x9D62,               // 9D 62 Card was not blocked
    CryptoFunctionsNotAvailable = 0x9D63,     // 9D 63 Crypto functions not available
    NoApplicationLoaded = 0x9D64,             // 9D 64 No application loaded
    PinIsNotInstalled = 0x9E00,               // 9E 00 PIN not installed
    PinUnsuccessfullyVerifiedPinNotInstalled = 0x9E04, // 9E 04 PIN not successfully verified, PIN not installed
    PinBlockedUnblockTryCounterIs3 = 0x9F00, // 9F 00 PIN blocked and Unblock Try Counter is 3
    PinUnsuccessfullyVerifiedPinBlockedAndUnblockTryCounterIs3 = 0x9F04, // 9F 04 PIN not successfully verified, PIN blocked and Unblock Try Counter is 3
    CommandSuccessfullyExecuted = 0x9FFF, // 9F XX Command successfully executed; &#8216;xx&#8217; bytes of data are available and can be requested using GET RESPONSE.
}

impl APDUResponse {
    pub fn iterator() -> impl Iterator<Item = APDUResponse> {
        [
    APDUResponse::ClassIsNotSupported, APDUResponse::CommandSuccess, APDUResponse::NoInformationGivenNVRAMUnchanged, APDUResponse::NVRamNotChanged, APDUResponse::ReturnedDataMayBeCorrupted, APDUResponse::EOFReachedBeforeReadingLeBytes, APDUResponse::SelectedFileInvalidated, APDUResponse::SelectedFileNotValid, APDUResponse::NoInputFromCardSensor, APDUResponse::WrongRMac, APDUResponse::CardLocked, APDUResponse::CounterWithValueSW2_2, APDUResponse::InternalReset, APDUResponse::DefaultAgentLocked, APDUResponse::CardholderLocked, APDUResponse::BasementIsCurrentAgent, APDUResponse::CALCKeySetNotUnblocked, APDUResponse::NoInformationGivenNVRAMChanged, APDUResponse::FileFilledUpByTheLastWrite, APDUResponse::CardKeyNotSupported, APDUResponse::ReaderKeyNotSupported, APDUResponse::PlaintextTransmissionNotSupported, APDUResponse::SecuredTransmissionNotSupported, APDUResponse::VolatileMemoryNotAvailable, APDUResponse::NonVolatileMemoryNotAvailable, APDUResponse::KeyNumberInvalid, APDUResponse::KeyLengthInvalid, APDUResponse::VerifyFailNoRetries, APDUResponse::VerifyFail1retry, APDUResponse::VerifyFail2retries, APDUResponse::VerifyFail3retries, APDUResponse::CounterReachedValueSW2_2, APDUResponse::MoreDataExpected, APDUResponse::MoreDataExpectedCommandPending, APDUResponse::CommandTimeout, APDUResponse::NoInformationGivenStateOfNVRAMChanged, APDUResponse::WriteErrorMemFailure, APDUResponse::MemFailure, APDUResponse::ReceivingTimeout, APDUResponse::ReceivingCharParityError, APDUResponse::WrongChecksum, APDUResponse::CurrentDFWithoutFC, APDUResponse::NoSFOrKFUnderCurrentDF, APDUResponse::IncorrectEncryptionDecryptionPadding, APDUResponse::WrongLength, APDUResponse::NoInformationGivenOperationNotSupported, APDUResponse::LogicalChannelNotSupported, APDUResponse::SecureMessagingNotSupported, APDUResponse::LastCommandOfTheChainExpected, APDUResponse::CommandChainingNotSupported, APDUResponse::NoInformationGivenCommandNotAllowed, APDUResponse::CommandNotAcceptedInInactiveState, APDUResponse::CommandIncompatibleWithFileStructure, APDUResponse::SecurityConditionNotSatisfied, APDUResponse::AuthenticationMethodBlocked, APDUResponse::ReferencedDataReversiblyBlocked, APDUResponse::ConditionsOfUseNotSatisfied, APDUResponse::CommandNotAllowed, APDUResponse::ExpectedSecureMessagingObjectMissing, APDUResponse::IncorrectSecureMessagingDataObject, APDUResponse::Reserved, APDUResponse::DataMustBeUpdatedAgain, APDUResponse::BlockedByPOL1, APDUResponse::PermissionDenied, APDUResponse::PermissionDeniedMissingPrivilege, APDUResponse::NoInformationGivenP1P2Incorrect, APDUResponse::DataFieldIncorrect, APDUResponse::FunctionNotSupported, APDUResponse::FileNotFound, APDUResponse::RecordNotFound, APDUResponse::InsufficientMemorySpaceInRecordOrFile, APDUResponse::LCInconsistentWithTLVStructure, APDUResponse::IncorrectP1OrP2, APDUResponse::LCInconsistentWithP1P2, APDUResponse::ReferencedDataNotFound, APDUResponse::FileAlreadyExists, APDUResponse::DFNameAlreadyExists, APDUResponse::WrongParameterValue, APDUResponse::WrongParametersP1P2, APDUResponse::IncorrectP3Length, APDUResponse::IncorrectLengthInLe, APDUResponse::InstructionCodeNotSupportedOrInvalid, APDUResponse::ClassNotSupported, APDUResponse::CommandAborted, APDUResponse::CardDead, APDUResponse::Ok, APDUResponse::PinInvalid3OrMoreTries, APDUResponse::KeyOrFileNotFound, APDUResponse::UnblockTryCounterReachedZero, APDUResponse::OK, APDUResponse::ActivityLockOrLockableHasWrongValue, APDUResponse::TransactionNumberReachedLimit, APDUResponse::NoChanges, APDUResponse::InsufficientNVRAMToCompleteCommand, APDUResponse::CommandCoreNotSupported, APDUResponse::CRCOrMACDoesNotMatchData, APDUResponse::InvalidKeyNumberSpecified, APDUResponse::LengthOFCommandStringInvalid, APDUResponse::RequestedCommandNotAllowed, APDUResponse::InvalidValueOfTheParameter, APDUResponse::RequestedAIDNotPresentOnPICC, APDUResponse::UnrecoverableErrorWithinApplication, APDUResponse::AuthenticationStatusDoesNotAllowRequestedCommand, APDUResponse::AdditionalDataFrameIsExpectedToBeSent, APDUResponse::OutOfBoundary, APDUResponse::UnrecoverableErrorWithinPICC, APDUResponse::PreviousCommandWasNotCompleted, APDUResponse::PICCWasDisabledByUnrecoverableError, APDUResponse::NumberOfApplicationsIsLimitedTo28, APDUResponse::FileOrApplicationExists, APDUResponse::CouldNotCompleteNVWriteOperationDueToLossOfPower, APDUResponse::FileNumberDoesNotExist, APDUResponse::UnrecoverableErrorWithinFile, APDUResponse::WritingToEEPROMSuccessful, APDUResponse::OutOfStorage, APDUResponse::WritingToEEPROMNotSuccessful, APDUResponse::IntegrityError, APDUResponse::CandidateS2Invalid, APDUResponse::ApplicationIsPermanentlyLocked, APDUResponse::NoEFSelected, APDUResponse::CandidateCurrencyCodeDoesNotMatchPurseCurrency, APDUResponse::CadidateAmountTooHigh, APDUResponse::CandidateAmountTooLow, APDUResponse::FIDOrRecordOrComparisonPatternNotFound, APDUResponse::ProblemsInDataField, APDUResponse::RequiredMACUnavailable, APDUResponse::BadCurrency, APDUResponse::CurrencyNotSupported, APDUResponse::BadSequence, APDUResponse::SlaveNotFound, APDUResponse::PinBlockedAndTryCounterIs2or1, APDUResponse::MainKeysAreBlocked, APDUResponse::PinVerificationUnsuccessful3orMoreRetries, APDUResponse::BaseKey, APDUResponse::CMACLimitExceeded, APDUResponse::RMACLimitExceeded, APDUResponse::SequenceCounterLimitExceeded, APDUResponse::RMACLengthLimitExceeded, APDUResponse::ServiceNotAvailable, APDUResponse::NoPinDefined, APDUResponse::AuthenticationFailed, APDUResponse::ASKRANDOMOrGIVERANDOMNotSatisfied, APDUResponse::PinVerificationUnsuccessful, APDUResponse::INCREASEOrDECREASENotExecutedLimitReached, APDUResponse::AuthenticationError, APDUResponse::OnePinRetryLeft, APDUResponse::PinUnsuccessfullyVerified1Retry, APDUResponse::WrongStatusCardholderLock, APDUResponse::MissingPrivilege, APDUResponse::PinNotInstalled, APDUResponse::WrongStatusRMACState, APDUResponse::TwoPinRetryLeft, APDUResponse::PinUnsuccessfullyVerified2Retries, APDUResponse::WrongParameterValueDoubleAgentAID, APDUResponse::WrongParameterValueDoubleAgentType, APDUResponse::IncorrectCertificateType, APDUResponse::IncorrectSessionDataSize, APDUResponse::IncorrectDIRFileOrRecordSize, APDUResponse::IncorrectFCIRecordSize, APDUResponse::IncorrectCodeSize, APDUResponse::InsufficientMemoryToLoadApplication, APDUResponse::InvalidAID, APDUResponse::DuplicateAID, APDUResponse::ApplicationPreviouslyLoaded, APDUResponse::ApplicationHistoryListFull, APDUResponse::ApplicationNotOpen, APDUResponse::InvalidOffset, APDUResponse::ApplicationAlreadyLoaded, APDUResponse::InvalidCertificate, APDUResponse::InvalidSignature, APDUResponse::InvalidKTU, APDUResponse::MSMControlsNotSet, APDUResponse::ApplicationSignatureDoesNotExist, APDUResponse::KTUDoesNotExist, APDUResponse::ApplicationNotLoaded, APDUResponse::InvalidOpenCommandDataLength, APDUResponse::CheckDataParameterIsIncorrectInvalidStartAddress, APDUResponse::CheckDataParameterIsIncorrectInvalidLength, APDUResponse::CheckDataParameterIsIncorrectCheckDataParameterIncorrect, APDUResponse::InvalidMSMControlsCiphertext, APDUResponse::MSMControlsAlreadySet, APDUResponse::MSMControlsDataLengthLessThan2B, APDUResponse::InvalidMSMControlsDataLength, APDUResponse::ExcessMSMControlsCiphertext, APDUResponse::VerificationOfMSMControlsDataFailed, APDUResponse::InvalidMCDIssuerProductionID, APDUResponse::InvalidMCDIssuerID, APDUResponse::InvalidSetMSMControlsDataDate, APDUResponse::InvalidMCDNumber, APDUResponse::MacVerificationFailed, APDUResponse::MaximumNumberOfUnblocksReached, APDUResponse::CardWasNotBlocked, APDUResponse::CryptoFunctionsNotAvailable, APDUResponse::NoApplicationLoaded, APDUResponse::PinIsNotInstalled, APDUResponse::PinUnsuccessfullyVerifiedPinNotInstalled, APDUResponse::PinBlockedUnblockTryCounterIs3, APDUResponse::PinUnsuccessfullyVerifiedPinBlockedAndUnblockTryCounterIs3, APDUResponse::CommandSuccessfullyExecuted
        ].iter().copied()
    }
}


struct rapdu {
    sw1: u8,
    sw2: u8,
}

impl rapdu {
    pub fn asAPDUResponse(&self) -> Result<APDUResponse, &'static str> {
        translate_response(self.sw1, self.sw2)
    }
}

pub fn translate_response(sw1: u8, sw2: u8) -> Result<APDUResponse, &'static str> {
    let swu16 = u16::from_be_bytes([sw1, sw2]);
    let iter = APDUResponse::iterator();
    for code in iter {
        if swu16 == code as u16 {
            return Ok(code);
        }
    }
    Err("No known code with such sw1 and sw2.")
    // let as_u16 = u16::from_be_bytes([sw1, sw2]);
    // std::mem::transmute_copy::<u16, APDUResponse>(&as_u16)
}
