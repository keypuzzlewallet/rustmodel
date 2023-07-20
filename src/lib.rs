#![allow(dead_code)]
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

// HotSigningRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotSigningRequest {
    // signingRequest
    #[serde(rename = "signingRequest")]
    pub signing_request: SigningRequest,
}

// GetSigningListRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSigningListRequest {
    // walletId
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

// ProtectedRegisterHotWallet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedRegisterHotWallet {
    // keygenId
    #[serde(rename = "keygenId")]
    pub keygen_id: String,
    // numberOfMembers
    #[serde(rename = "numberOfMembers")]
    pub number_of_members: i32,
    // threshold
    #[serde(rename = "threshold")]
    pub threshold: i32,
    // walletName
    #[serde(rename = "walletName")]
    pub wallet_name: String,
    // partyId
    #[serde(rename = "partyId")]
    pub party_id: Option<i32>,
    // members
    #[serde(rename = "members")]
    pub members: Vec<KeygenMember>,
    // encrypted local key
    #[serde(rename = "encryptedKeygenResult")]
    pub encrypted_keygen_result: EncryptedKeygenResult,
    // walletCreationConfig
    #[serde(rename = "walletCreationConfig")]
    pub wallet_creation_config: WalletCreationConfig,
    // userId which can request this hot wallet to sign a transaction
    #[serde(rename = "authorizedUsers")]
    pub authorized_users: Vec<String>,
}

// SignedPartialSignatureBase64
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedPartialSignatureBase64 {
    // party_id
    #[serde(rename = "party_id")]
    pub party_id: i32,
    // part_base64
    #[serde(rename = "part_base64")]
    pub part_base64: String,
    // signed_at
    #[serde(rename = "signed_at")]
    pub signed_at: String,
}

// SigningStateBase64
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningStateBase64 {
    // t
    #[serde(rename = "t")]
    pub t: i32,
    // n
    #[serde(rename = "n")]
    pub n: i32,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // signing_parts_base64
    #[serde(rename = "signing_parts_base64")]
    pub signing_parts_base64: Vec<SignedPartialSignatureBase64>,
    // signature_hex
    #[serde(rename = "signature")]
    pub signature: Option<SignatureRecidHex>,
}

// SigningResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningResult {
    // signingHashes
    #[serde(rename = "signingHashes")]
    pub signing_hashes: Vec<SigningHash>,
    // hex transaction to be signed. We could use this to verify details in the request.
    #[serde(rename = "unsignedTransaction")]
    pub unsigned_transaction: String,
    // transaction id/hash which could be obtained after signing or submit in some blockchains
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    // hex signedTransaction to be sent
    #[serde(rename = "signedTransaction")]
    pub signed_transaction: Option<String>,
}

// NativeSigningRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeSigningRequest {
    // signing state
    #[serde(rename = "stateBase64")]
    pub state_base64: SigningStateBase64,
    // data to sign in hex format. no 0x prefix. lower case.
    #[serde(rename = "hexData")]
    pub hex_data: String,
    // encryptedLocalKey
    #[serde(rename = "encryptedLocalKey")]
    pub encrypted_local_key: EncryptedLocalKey,
    // key scheme used to sign this message
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // party id who is signing
    #[serde(rename = "partyId")]
    pub party_id: i32,
    // signers who are assigned to sign this message
    #[serde(rename = "signers")]
    pub signers: Vec<i32>,
    // password to decrypt the generated private key
    #[serde(rename = "password")]
    pub password: String,
    // nonce index to sign. This only use for EDDSA at the moment
    #[serde(rename = "nonce")]
    pub nonce: i32,
}

// SigningSessionFailed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningSessionFailed {
    // signingId
    #[serde(rename = "signingId")]
    pub signing_id: String,
    // error
    #[serde(rename = "error")]
    pub error: String,
}

// GetSigningList
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSigningListResult {
    // signings
    #[serde(rename = "signings")]
    pub signings: Vec<SigningRequest>,
}

// SigningHash
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningHash {
    // signing state that contains part signed from parties. If all required part signed are included, it will generate signature
    #[serde(rename = "state")]
    pub state: Option<SigningStateBase64>,
    // private key nonce to sign this hash
    #[serde(rename = "nonce")]
    pub nonce: i32,
    // hash to sign
    #[serde(rename = "hash")]
    pub hash: String,
}

// SigningRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningRequest {
    // session id
    #[serde(rename = "id")]
    pub id: String,
    // walletId
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    // blockchain requesting for this transaction
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin to send
    #[serde(rename = "coin")]
    pub coin: Coin,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // pubkey public to sign. this is to verify after signing to ensure that signer is correct
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // address that is create and sign the transaction
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    // threshold
    #[serde(rename = "threshold")]
    pub threshold: i32,
    // request transaction type
    #[serde(rename = "requestTransactionType")]
    pub request_transaction_type: RequestTransactionType,
    // signing status
    #[serde(rename = "status")]
    pub status: SigningStatus,
    // status message of this request e.g. error message
    #[serde(rename = "message")]
    pub message: Option<String>,
    // signingResult
    #[serde(rename = "signingResult")]
    pub signing_result: Option<SigningResult>,
    // details of request for sending transaction type
    #[serde(rename = "sendRequest")]
    pub send_request: Option<SendRequest>,
    // details of request for sending token transaction type
    #[serde(rename = "sendTokenRequest")]
    pub send_token_request: Option<SendTokenRequest>,
    // detail of a request from ethereum smart contract call
    #[serde(rename = "ethSmartContractRequest")]
    pub eth_smart_contract_request: Option<EthContractRequest>,
    // Party_id of signing members who are assigned to sign the transaction
    #[serde(rename = "signers")]
    pub signers: Vec<i32>,
    // feeLevel
    #[serde(rename = "feeLevel")]
    pub fee_level: FeeLevel,
    // total amount needs to pay for the transaction
    #[serde(rename = "fee")]
    pub fee: Option<BigDecimal>,
    // current version of the transaction request. Increase one every update. When update a signing request, if the version is old, it will be rejected
    #[serde(rename = "version")]
    pub version: i32,
    // time when the transaction request was created
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

// SignatureRecidHex
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureRecidHex {
    // r
    #[serde(rename = "r")]
    pub r: String,
    // s
    #[serde(rename = "s")]
    pub s: String,
    // recid
    #[serde(rename = "recid")]
    pub recid: i32,
}

// SendRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendRequest {
    // toAddress
    #[serde(rename = "toAddress")]
    pub to_address: String,
    // amount
    #[serde(rename = "amount")]
    pub amount: BigDecimal,
}

// SendRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthContractRequest {
    // smart contract address
    #[serde(rename = "toAddress")]
    pub to_address: String,
    // amount of native coin that we send to the contract
    #[serde(rename = "amount")]
    pub amount: BigDecimal,
    // gas limit provided by contract
    #[serde(rename = "gasLimit")]
    pub gas_limit: BigDecimal,
    // smart contract data
    #[serde(rename = "data")]
    pub data: String,
}

// SendTokenRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendTokenRequest {
    // toAddress
    #[serde(rename = "toAddress")]
    pub to_address: String,
    // tokenContractAddress
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: String,
    // amount
    #[serde(rename = "amount")]
    pub amount: BigDecimal,
    // decimal places of token
    #[serde(rename = "decimals")]
    pub decimals: i32,
}

// NativeKeygenRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeKeygenRequest {
    // callback port to report the final keygen result
    #[serde(rename = "port")]
    pub port: i64,
    // address to report the final keygen result
    #[serde(rename = "address")]
    pub address: String,
    // key generation unique session id
    #[serde(rename = "sessionId")]
    pub session_id: String,
    // threshold
    #[serde(rename = "t")]
    pub t: i32,
    // total number of parties
    #[serde(rename = "n")]
    pub n: i32,
    // signer name
    #[serde(rename = "signerName")]
    pub signer_name: String,
    // password to encrypt the generated private key
    #[serde(rename = "password")]
    pub password: String,
    // requestId to easily identify the request
    #[serde(rename = "requestId")]
    pub request_id: String,
    // token to authenticate the request
    #[serde(rename = "token")]
    pub token: String,
    // unique party id 1-base index
    #[serde(rename = "partyId")]
    pub party_id: i32,
}

// ProtectedHotWalletGenerateNonceRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedHotWalletGenerateNonceRequest {
    // pubkey that client request generate nonces
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // nonceStartIndex
    #[serde(rename = "nonceStartIndex")]
    pub nonce_start_index: i32,
    // nonceSize
    #[serde(rename = "nonceSize")]
    pub nonce_size: i32,
    // roomId
    #[serde(rename = "roomId")]
    pub room_id: String,
    // encryptedLocalKey
    #[serde(rename = "encryptedLocalKey")]
    pub encrypted_local_key: EncryptedLocalKey,
}

// NativeGenerateDynamicNonceRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeGenerateDynamicNonceRequest {
    // callback port to report the final keygen result
    #[serde(rename = "port")]
    pub port: i64,
    // address to report the final keygen result
    #[serde(rename = "address")]
    pub address: String,
    // key generation unique session id
    #[serde(rename = "sessionId")]
    pub session_id: String,
    // requestId to easily identify the request
    #[serde(rename = "requestId")]
    pub request_id: String,
    // token to authenticate the request
    #[serde(rename = "token")]
    pub token: String,
    // password to dencrypt the private key
    #[serde(rename = "password")]
    pub password: String,
    // nonce start index. This is 0 base and starting from the last previous generated nonce. For example, previous generated from 0 with 100 nonce. the next value starting from 100
    #[serde(rename = "nonceStartIndex")]
    pub nonce_start_index: i32,
    // Number of nonces to generate
    #[serde(rename = "nonceSize")]
    pub nonce_size: i32,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // encryptedLocalKey
    #[serde(rename = "encryptedLocalKey")]
    pub encrypted_local_key: EncryptedLocalKey,
}

// IssueIndexMsg
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssueIndexMsg {
    // parties
    #[serde(rename = "parties")]
    pub parties: Vec<i32>,
    // party_name
    #[serde(rename = "party_name")]
    pub party_name: String,
}

// ProtectedHotWalletGenerateNonceRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedUpdateHotWalletNonce {
    // pubkey that client request generate nonces
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // nonceStartIndex
    #[serde(rename = "nonceStartIndex")]
    pub nonce_start_index: i32,
    // nonceSize
    #[serde(rename = "nonceSize")]
    pub nonce_size: i32,
    // encryptedLocalKey
    #[serde(rename = "encryptedLocalKey")]
    pub encrypted_local_key: EncryptedLocalKey,
}

// KeygenMember
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeygenMember {
    // party_id
    #[serde(rename = "party_id")]
    pub party_id: i32,
    // name of party member. This is to help easier to identify and assign signer when signing a transaction
    #[serde(rename = "party_name")]
    pub party_name: String,
}

// HotWalletGenerateNonceRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotWalletGenerateNonceRequest {
    // pubkey that client request generate nonces
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // nonceStart
    #[serde(rename = "nonceStart")]
    pub nonce_start: i32,
    // nonceSize
    #[serde(rename = "nonceSize")]
    pub nonce_size: i32,
    // roomId
    #[serde(rename = "roomId")]
    pub room_id: String,
}

// EncryptedKeygenWithScheme
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedKeygenWithScheme {
    // encryptedLocalKey
    #[serde(rename = "encryptedLocalKey")]
    pub encrypted_local_key: EncryptedLocalKey,
    // nonce start index
    #[serde(rename = "nonceStartIndex")]
    pub nonce_start_index: i32,
    // number of nonces generated
    #[serde(rename = "nonceSize")]
    pub nonce_size: i32,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
}

// HotWalletKeygenRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotWalletKeygenRequest {
    // keygenId
    #[serde(rename = "keygenId")]
    pub keygen_id: String,
    // numberOfMembers
    #[serde(rename = "numberOfMembers")]
    pub number_of_members: i32,
    // threshold
    #[serde(rename = "threshold")]
    pub threshold: i32,
    // walletName
    #[serde(rename = "walletName")]
    pub wallet_name: String,
    // roomId
    #[serde(rename = "roomId")]
    pub room_id: String,
    // walletCreationConfig
    #[serde(rename = "walletCreationConfig")]
    pub wallet_creation_config: WalletCreationConfig,
}

// Encrypted key and nonce for localkey
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedLocalKey {
    // pubkey
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // encryptedKey
    #[serde(rename = "encryptedKey")]
    pub encrypted_key: String,
    // encryptedNonce
    #[serde(rename = "encryptedNonce")]
    pub encrypted_nonce: String,
    // signature algorithm
    #[serde(rename = "algorithm")]
    pub algorithm: String,
}

// client request to get status of the key generation process
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeygenProgress {
    // joined members
    #[serde(rename = "members")]
    pub members: Vec<KeygenMember>,
    // approximate percentage of the key generation process. 0 to 100
    #[serde(rename = "progress")]
    pub progress: i32,
}

// IssuedUniqueIdx
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssuedUniqueIdx {
    // unique_idx
    #[serde(rename = "unique_idx")]
    pub unique_idx: i32,
}

// EncryptedKeygenResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedKeygenResult {
    // party_id
    #[serde(rename = "party_id")]
    pub party_id: i32,
    // encryptedKeygenWithScheme
    #[serde(rename = "encryptedKeygenWithScheme")]
    pub encrypted_keygen_with_scheme: Vec<EncryptedKeygenWithScheme>,
}

// FeeLevel
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum FeeLevel {
    // Low fee transaction. It costs less but transaction may take longer to be mined
    LOW,
    // Medium fee transaction. Balanced cost and mined time
    MEDIUM,
    // High fee transaction. Transaction could be mined faster.
    HIGH,
}

// Keygen status event
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum KeygenStatus {
    // Key generation session created and waiting for parties to join
    KEYGEN_SESSION_CREATED,
    // Key generation completed
    KEYGEN_COMPLETED,
    // Key generation failed
    KEYGEN_FAILED,
}

// Supported blockchains
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum Blockchain {
    // Bitcoin blockchain
    BITCOIN,
    // Ethereum blockchain
    ETHEREUM,
    // Polygon/Matic blockchain
    POLYGON,
    // Cardano blockchain
    CARDANO,
}

// Fiat currencies
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum Fiat {
    // United States Dollar
    USD,
    // British Pound Sterling
    GBP,
    // Euro
    EUR,
}

// AlertLevel
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum AlertLevel {
    // ERROR
    ERROR,
    // WARN
    WARN,
    // INFO
    INFO,
    // DEBUG
    DEBUG,
}

// SigningStatus event
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum SigningStatus {
    // Signing information is created but information that is required for signing has not been populated yet
    SIGNING_SESSION_CREATED,
    // Signing is in progress by parties
    SIGNING_IN_PROGRESS,
    // All required parties has signed but not broadcasted yet
    SIGNING_COMPLETED,
    // Signing failed or signed but failed on broadcast
    SIGNING_FAILED,
    // transaction has been broadcasted to network. Transaction may not included in a block
    SIGNING_BROADCASTED,
}

// RequestTransactionType event
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum RequestTransactionType {
    // Send native fund from one address to another address
    SEND,
    // Send token from one address to another address
    SEND_TOKEN,
    // Ethereum like smart contract transaction
    ETH_SMART_CONTRACT_CALL,
}

// Supported signature schemes
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum KeyScheme {
    // ECDSA
    ECDSA,
    // EDDSA
    EDDSA,
}

// Supported currencies/coin
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString)]
pub enum Coin {
    // Bitcoin
    BTC,
    // Ethereum
    ETH,
    // Polygon/Matic
    MATIC,
    // Tether USDT
    USDT,
    // ADA
    ADA,
}

// CoinPrice
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinPrice {
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // coinPriceItems
    #[serde(rename = "coinPriceItems")]
    pub coin_price_items: Vec<CoinPriceItem>,
}

// CoinPrices
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinPrices {
    // coinPrices
    #[serde(rename = "coinPrices")]
    pub coin_prices: Vec<CoinPrice>,
}

// CoinPrice
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinPriceItem {
    // fiat
    #[serde(rename = "fiat")]
    pub fiat: Fiat,
    // price
    #[serde(rename = "price")]
    pub price: BigDecimal,
}

// FiatConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FiatConfig {
    // fiat
    #[serde(rename = "fiat")]
    pub fiat: Fiat,
    // Fiat Name
    #[serde(rename = "fiatName")]
    pub fiat_name: String,
    // Fiat symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
    // ID of price feed to use for this fiat
    #[serde(rename = "priceFeedId")]
    pub price_feed_id: String,
    // flags
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    // enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

// transaction was confirmed including in a block
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionIncluded {
    // signing session that created the transaction
    #[serde(rename = "signingSessionId")]
    pub signing_session_id: String,
    // result transaction id after broadcasted
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

// GenerateTransactionResponse
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTransactionError {
    // signingId
    #[serde(rename = "signingId")]
    pub signing_id: String,
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // reason why transaction failed to be created
    #[serde(rename = "error")]
    pub error: String,
}

// GenerateTransactionRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTransactionRequest {
    // signingId
    #[serde(rename = "signingId")]
    pub signing_id: String,
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // detail of signing request
    #[serde(rename = "signingRequest")]
    pub signing_request: SigningRequest,
}

// SignTransactionRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignTransactionRequest {
    // signingId
    #[serde(rename = "signingId")]
    pub signing_id: String,
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // raw/unsigned transaction that is requested to be signed
    #[serde(rename = "unsignedTransaction")]
    pub unsigned_transaction: String,
    // wallet pubkey that is signing this transaction
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // hash string form raw transaction that are signing
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
    // signature for each signing hash in hashes array
    #[serde(rename = "signatures")]
    pub signatures: Vec<SignatureRecidHex>,
}

// GenerateTransactionResponse
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTransactionResponse {
    // signingId
    #[serde(rename = "signingId")]
    pub signing_id: String,
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // created unsigned transaction
    #[serde(rename = "rawTx")]
    pub raw_tx: String,
    // estimated transaction fee
    #[serde(rename = "fee")]
    pub fee: BigDecimal,
    // list of hashes that required user to sign. If there are multiple hashes, the order is the same as inputs in the request
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
}

// EstimateFeeResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstimateFeeResult {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // low estimated fee. The actual fee may be slightly different depending on the network conditions
    #[serde(rename = "lowEstimatedFee")]
    pub low_estimated_fee: BigDecimal,
    // medium estimated fee. The actual fee may be slightly different depending on the network conditions
    #[serde(rename = "mediumEstimatedFee")]
    pub medium_estimated_fee: BigDecimal,
    // high estimated fee. The actual fee may be slightly different depending on the network conditions
    #[serde(rename = "highEstimatedFee")]
    pub high_estimated_fee: BigDecimal,
}

// TransactionSigned
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionSigned {
    // signingId
    #[serde(rename = "signingId")]
    pub signing_id: String,
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // signed transaction that is created from unsigned transaction and signatures
    #[serde(rename = "signedTransaction")]
    pub signed_transaction: String,
    // unique transaction id for the signed transaction. In some other blockchains, it is also called transaction hash
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

// UnspentOutput
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnspentOutput {
    // transactionHash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    // index
    #[serde(rename = "index")]
    pub index: i32,
    // script
    #[serde(rename = "script")]
    pub script: String,
    // unspent amount
    #[serde(rename = "amount")]
    pub amount: BigDecimal,
}

// EstimateFeeRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstimateFeeRequest {
    // signingRequest
    #[serde(rename = "signingRequest")]
    pub signing_request: SigningRequest,
}

// transaction was broadcasted to blockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionBroadcasted {
    // signing session that created the transaction
    #[serde(rename = "signingSessionId")]
    pub signing_session_id: String,
    // result transaction id after broadcasted
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

// BlockchainConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockchainConfig {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
    // Blockchain Name
    #[serde(rename = "blockchainName")]
    pub blockchain_name: String,
    // flags
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    // chainId
    #[serde(rename = "chainId")]
    pub chain_id: Option<String>,
    // public rpc
    #[serde(rename = "rpc")]
    pub rpc: Option<String>,
    // explorer to view transaction status
    #[serde(rename = "txExplorer")]
    pub tx_explorer: String,
    // explorer to view address balance
    #[serde(rename = "addressExplorer")]
    pub address_explorer: String,
    // enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

// BlockchainCoinConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockchainCoinConfig {
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // Coin Name
    #[serde(rename = "coinName")]
    pub coin_name: String,
    // ID of price feed to use for this coin
    #[serde(rename = "priceFeedId")]
    pub price_feed_id: String,
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // decimals
    #[serde(rename = "decimals")]
    pub decimals: i32,
    // whether the coin is native to the blockchain
    #[serde(rename = "isNative")]
    pub is_native: bool,
    // address of token contract to interact with
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    // flags
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    // enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

// CoinConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinConfig {
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // Coin Name
    #[serde(rename = "coinName")]
    pub coin_name: String,
    // ID of price feed to use for this coin
    #[serde(rename = "priceFeedId")]
    pub price_feed_id: String,
    // configForBlockchain
    #[serde(rename = "configForBlockchain")]
    pub config_for_blockchain: Vec<ConfigForBlockchain>,
}

// ConfigForBlockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigForBlockchain {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // decimals
    #[serde(rename = "decimals")]
    pub decimals: i32,
    // whether the coin is native to the blockchain
    #[serde(rename = "isNative")]
    pub is_native: bool,
    // address of token contract to interact with
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    // flags
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    // enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

// CreateTransactionResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTransactionResult {
    // created raw transaction from the request
    #[serde(rename = "rawTransaction")]
    pub raw_transaction: String,
    // total amount needs to pay for the transaction
    #[serde(rename = "fee")]
    pub fee: BigDecimal,
    // hashes of the created raw transaction
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
}

// CreateSignTransactionRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSignTransactionRequest {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // signer ecopint which will be used to verify if signature is valid
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // raw transaction that is used to decoded and sign
    #[serde(rename = "rawTransaction")]
    pub raw_transaction: String,
    // hashes of the transaction
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
    // signatures of the transaction
    #[serde(rename = "signatures")]
    pub signatures: Vec<SignatureRecidHex>,
}

// VerifyTransactionRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyTransactionRequest {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // transaction that user will sign. we need to verify it against sigingRequest before signing
    #[serde(rename = "rawTransaction")]
    pub raw_transaction: String,
    // this object is visible to user. we need to make sure rawTransaction is signed according to this object
    #[serde(rename = "signingRequest")]
    pub signing_request: SigningRequest,
}

// VerifyTransactionResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyTransactionResult {
    // reason why verification failed. If verification succeeded, this field is empty
    #[serde(rename = "failedReason")]
    pub failed_reason: Option<String>,
}

// GetAddressResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAddressResult {
    // address
    #[serde(rename = "address")]
    pub address: String,
}

// More details to support creating transaction. This information is often provided by backend side
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // details of the transaction to be signed which created by user
    #[serde(rename = "signingRequest")]
    pub signing_request: SigningRequest,
    // parameters required for signing UTXO chains like BTC
    #[serde(rename = "requestParamsBtc")]
    pub request_params_btc: Option<RequestParamsBtc>,
    // parameters required for signing EVM chains like ETH POLYGON
    #[serde(rename = "requestParamsEthLegacy")]
    pub request_params_eth_legacy: Option<RequestParamsEthLegacy>,
    // parameters required for signing EVM chains like ETH POLYGON using EIP1559 gas model
    #[serde(rename = "requestParamsEthEip1559")]
    pub request_params_eth_eip1559: Option<RequestParamsEthEip1559>,
    // parameters required for signing Cardano chains
    #[serde(rename = "requestParamsAda")]
    pub request_params_ada: Option<RequestParamsAda>,
}

// GetAddressRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAddressRequest {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // config of created wallet
    #[serde(rename = "walletConfig")]
    pub wallet_config: WalletCreationConfig,
}

// CreateSignTransactionResult
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSignTransactionResult {
    // signed transaction which is ready to send to the network
    #[serde(rename = "signedTransaction")]
    pub signed_transaction: String,
    // hash of the transaction
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

// RequestParamsEth
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestParamsEthLegacy {
    // gas price
    #[serde(rename = "gasFee")]
    pub gas_fee: BigDecimal,
    // specify chain id for blockchain that may have different chain id in mainnet and testnet e.g. ETH
    #[serde(rename = "chainId")]
    pub chain_id: String,
    // account nonce number
    #[serde(rename = "nonce")]
    pub nonce: i32,
}

// RequestParamsBtc
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestParamsBtc {
    // used in UTXO transaction type e.g. BTC, LTC, BCH, DOGE
    #[serde(rename = "unspentOutputs")]
    pub unspent_outputs: Vec<UnspentOutput>,
    // (BTC) fee per byte
    #[serde(rename = "feePerByte")]
    pub fee_per_byte: BigDecimal,
}

// RequestParamsAda
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestParamsAda {
    // used in UTXO transaction type e.g. BTC, LTC, BCH, DOGE
    #[serde(rename = "unspentOutputs")]
    pub unspent_outputs: Vec<UnspentOutput>,
    // slot number when the transaction will be invalid. Calculate this by current slot + number of slots expect to be in the mempool.
    #[serde(rename = "ttl")]
    pub ttl: i32,
    // coeff value (unit) in fee linear coeff x bytes + constant. This finds in network parameters.
    #[serde(rename = "feeCoeff")]
    pub fee_coeff: i32,
    // constant value (unit) in fee linear coeff x bytes + constant. This finds in network parameters.
    #[serde(rename = "feeConstant")]
    pub fee_constant: i32,
    // ada value (unit) per utxo byte. This finds in network parameters.
    #[serde(rename = "coinPerUtxoByte")]
    pub coin_per_utxo_byte: i32,
}

// RequestParamsEth
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestParamsEthEip1559 {
    // specify chain id for blockchain that may have different chain id in mainnet and testnet e.g. ETH
    #[serde(rename = "chainId")]
    pub chain_id: String,
    // account nonce number
    #[serde(rename = "nonce")]
    pub nonce: i32,
    // base gas price
    #[serde(rename = "baseGasFee")]
    pub base_gas_fee: BigDecimal,
    // priority fee for transaction
    #[serde(rename = "priorityFee")]
    pub priority_fee: BigDecimal,
}

// Alert event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    // level
    #[serde(rename = "level")]
    pub level: AlertLevel,
    // message
    #[serde(rename = "message")]
    pub message: String,
    // alert code which reflect http status code
    #[serde(rename = "code")]
    pub code: Option<i32>,
}

// EmailActionRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailActionRequest {
    // command to send. We must verify if command is allowed to send by client email action. For example, HotSigningRequest command to request server hot signing
    #[serde(rename = "command")]
    pub command: String,
    // command body to send
    #[serde(rename = "commandBody")]
    pub command_body: String,
}

// ProtectedEmailActionVerify
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedEmailActionVerify {
    // actionId
    #[serde(rename = "actionId")]
    pub action_id: String,
    // token
    #[serde(rename = "token")]
    pub token: String,
}

// Update the current user's wallet balance for one specific coin
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletBalanceUpdate {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // wallet balance
    #[serde(rename = "balance")]
    pub balance: BigDecimal,
}

// contain raw transaction that is ready to be sent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendRawTransactionRequest {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coin
    #[serde(rename = "coin")]
    pub coin: Coin,
    // sessionId
    #[serde(rename = "sessionId")]
    pub session_id: String,
    // hex signed raw transaction to be sent
    #[serde(rename = "signedRawTransaction")]
    pub signed_raw_transaction: String,
    // sender address
    #[serde(rename = "fromAddress")]
    pub from_address: String,
}

// WalletCreationConfigPubkey
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletCreationConfigPubkey {
    // wallet public key for given signature scheme
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    // keyScheme
    #[serde(rename = "keyScheme")]
    pub key_scheme: KeyScheme,
}

// wallet is loaded on client app
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientWalletLoaded {
    // blockchains enabled for wallet
    #[serde(rename = "enabledBlockchains")]
    pub enabled_blockchains: Vec<EnabledBlockchain>,
    // config of created wallet
    #[serde(rename = "walletConfig")]
    pub wallet_config: WalletCreationConfig,
}

// EnabledBlockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnabledBlockchain {
    // blockchain
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    // coins to enable for this blockchain
    #[serde(rename = "coins")]
    pub coins: Vec<Coin>,
}

// Configuration for initializing wallet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletCreationConfig {
    // wallet public keys
    #[serde(rename = "pubkeys")]
    pub pubkeys: Vec<WalletCreationConfigPubkey>,
    // isMainnet
    #[serde(rename = "isMainnet")]
    pub is_mainnet: bool,
    // If this is set and value is true, wallet will create segwit address
    #[serde(rename = "isSegwit")]
    pub is_segwit: bool,
}

// UserPing event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPing {
    // userStreamId
    #[serde(rename = "userStreamId")]
    pub user_stream_id: String,
    // message
    #[serde(rename = "pingAt")]
    pub ping_at: String,
}

