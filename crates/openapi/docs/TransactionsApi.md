# \TransactionsApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_transactions_details_txid**](TransactionsApi.md#get_transactions_details_txid) | **GET** /transactions/details/{txId} | Get transaction details
[**get_transactions_raw_txid**](TransactionsApi.md#get_transactions_raw_txid) | **GET** /transactions/raw/{txId} | Get raw transaction in hex format
[**get_transactions_rich_details_txid**](TransactionsApi.md#get_transactions_rich_details_txid) | **GET** /transactions/rich-details/{txId} | Get transaction with enriched input information when node indexes are enabled.
[**get_transactions_status**](TransactionsApi.md#get_transactions_status) | **GET** /transactions/status | Get tx status
[**get_transactions_tx_id_from_outputref**](TransactionsApi.md#get_transactions_tx_id_from_outputref) | **GET** /transactions/tx-id-from-outputref | Get transaction id from transaction output ref
[**post_transactions_build**](TransactionsApi.md#post_transactions_build) | **POST** /transactions/build | Build an unsigned transfer transaction to a number of recipients
[**post_transactions_build_chained**](TransactionsApi.md#post_transactions_build_chained) | **POST** /transactions/build-chained | Build a chain of transactions
[**post_transactions_build_multi_addresses**](TransactionsApi.md#post_transactions_build_multi_addresses) | **POST** /transactions/build-multi-addresses | Build an unsigned transaction with multiple addresses to a number of recipients
[**post_transactions_build_transfer_from_one_to_many_groups**](TransactionsApi.md#post_transactions_build_transfer_from_one_to_many_groups) | **POST** /transactions/build-transfer-from-one-to-many-groups | Build unsigned transfer transactions from an address of one group to addresses of many groups. Each target group requires a dedicated transaction or more in case large number of outputs needed to be split.
[**post_transactions_decode_unsigned_tx**](TransactionsApi.md#post_transactions_decode_unsigned_tx) | **POST** /transactions/decode-unsigned-tx | Decode an unsigned transaction
[**post_transactions_submit**](TransactionsApi.md#post_transactions_submit) | **POST** /transactions/submit | Submit a signed transaction
[**post_transactions_sweep_address_build**](TransactionsApi.md#post_transactions_sweep_address_build) | **POST** /transactions/sweep-address/build | Build unsigned transactions to send all unlocked ALPH and token balances of one address to another address



## get_transactions_details_txid

> models::Transaction get_transactions_details_txid(tx_id, from_group, to_group)
Get transaction details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** |  | [required] |
**from_group** | Option<**i32**> |  |  |
**to_group** | Option<**i32**> |  |  |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_raw_txid

> models::RawTransaction get_transactions_raw_txid(tx_id, from_group, to_group)
Get raw transaction in hex format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** |  | [required] |
**from_group** | Option<**i32**> |  |  |
**to_group** | Option<**i32**> |  |  |

### Return type

[**models::RawTransaction**](RawTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_rich_details_txid

> models::RichTransaction get_transactions_rich_details_txid(tx_id, from_group, to_group)
Get transaction with enriched input information when node indexes are enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** |  | [required] |
**from_group** | Option<**i32**> |  |  |
**to_group** | Option<**i32**> |  |  |

### Return type

[**models::RichTransaction**](RichTransaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_status

> models::TxStatus get_transactions_status(tx_id, from_group, to_group)
Get tx status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** |  | [required] |
**from_group** | Option<**i32**> |  |  |
**to_group** | Option<**i32**> |  |  |

### Return type

[**models::TxStatus**](TxStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_tx_id_from_outputref

> String get_transactions_tx_id_from_outputref(hint, key)
Get transaction id from transaction output ref

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hint** | **i32** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_build

> models::BuildTransferTxResult post_transactions_build(build_transfer_tx)
Build an unsigned transfer transaction to a number of recipients

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_transfer_tx** | [**BuildTransferTx**](BuildTransferTx.md) | Format 1: `1000000000000000000`  Format 2: `x.y ALPH`, where `1 ALPH = 1000000000000000000  Field fromPublicKeyType can be  `default` or `bip340-schnorr` | [required] |

### Return type

[**models::BuildTransferTxResult**](BuildTransferTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_build_chained

> Vec<models::BuildChainedTxResult> post_transactions_build_chained(build_chained_tx)
Build a chain of transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_chained_tx** | [**Vec<models::BuildChainedTx>**](BuildChainedTx.md) |  | [required] |

### Return type

[**Vec<models::BuildChainedTxResult>**](BuildChainedTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_build_multi_addresses

> models::BuildSimpleTransferTxResult post_transactions_build_multi_addresses(build_multi_addresses_transaction)
Build an unsigned transaction with multiple addresses to a number of recipients

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_multi_addresses_transaction** | [**BuildMultiAddressesTransaction**](BuildMultiAddressesTransaction.md) | Format 1: `1000000000000000000`  Format 2: `x.y ALPH`, where `1 ALPH = 1000000000000000000  Field fromPublicKeyType can be  `default` or `bip340-schnorr` | [required] |

### Return type

[**models::BuildSimpleTransferTxResult**](BuildSimpleTransferTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_build_transfer_from_one_to_many_groups

> Vec<models::BuildSimpleTransferTxResult> post_transactions_build_transfer_from_one_to_many_groups(build_transfer_tx)
Build unsigned transfer transactions from an address of one group to addresses of many groups. Each target group requires a dedicated transaction or more in case large number of outputs needed to be split.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_transfer_tx** | [**BuildTransferTx**](BuildTransferTx.md) | Format 1: `1000000000000000000`  Format 2: `x.y ALPH`, where `1 ALPH = 1000000000000000000  Field fromPublicKeyType can be  `default` or `bip340-schnorr` | [required] |

### Return type

[**Vec<models::BuildSimpleTransferTxResult>**](BuildSimpleTransferTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_decode_unsigned_tx

> models::DecodeUnsignedTxResult post_transactions_decode_unsigned_tx(decode_unsigned_tx)
Decode an unsigned transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decode_unsigned_tx** | [**DecodeUnsignedTx**](DecodeUnsignedTx.md) |  | [required] |

### Return type

[**models::DecodeUnsignedTxResult**](DecodeUnsignedTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_submit

> models::SubmitTxResult post_transactions_submit(submit_transaction)
Submit a signed transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_transaction** | [**SubmitTransaction**](SubmitTransaction.md) |  | [required] |

### Return type

[**models::SubmitTxResult**](SubmitTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_transactions_sweep_address_build

> models::BuildSweepAddressTransactionsResult post_transactions_sweep_address_build(build_sweep_address_transactions)
Build unsigned transactions to send all unlocked ALPH and token balances of one address to another address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_sweep_address_transactions** | [**BuildSweepAddressTransactions**](BuildSweepAddressTransactions.md) |  | [required] |

### Return type

[**models::BuildSweepAddressTransactionsResult**](BuildSweepAddressTransactionsResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

