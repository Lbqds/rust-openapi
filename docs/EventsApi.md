# \EventsApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events_block_hash_blockhash**](EventsApi.md#get_events_block_hash_blockhash) | **GET** /events/block-hash/{blockHash} | Get contract events for a block
[**get_events_contract_contractaddress**](EventsApi.md#get_events_contract_contractaddress) | **GET** /events/contract/{contractAddress} | Get events for a contract within a counter range
[**get_events_contract_contractaddress_current_count**](EventsApi.md#get_events_contract_contractaddress_current_count) | **GET** /events/contract/{contractAddress}/current-count | Get current value of the events counter for a contract
[**get_events_tx_id_txid**](EventsApi.md#get_events_tx_id_txid) | **GET** /events/tx-id/{txId} | Get contract events for a transaction



## get_events_block_hash_blockhash

> models::ContractEventsByBlockHash get_events_block_hash_blockhash(block_hash, group)
Get contract events for a block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |
**group** | Option<**i32**> |  |  |

### Return type

[**models::ContractEventsByBlockHash**](ContractEventsByBlockHash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_contract_contractaddress

> models::ContractEvents get_events_contract_contractaddress(contract_address, start, limit, group)
Get events for a contract within a counter range

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** |  | [required] |
**start** | **i32** |  | [required] |
**limit** | Option<**i32**> |  |  |
**group** | Option<**i32**> |  |  |

### Return type

[**models::ContractEvents**](ContractEvents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_contract_contractaddress_current_count

> i32 get_events_contract_contractaddress_current_count(contract_address)
Get current value of the events counter for a contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | **String** |  | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_tx_id_txid

> models::ContractEventsByTxId get_events_tx_id_txid(tx_id, group)
Get contract events for a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** |  | [required] |
**group** | Option<**i32**> |  |  |

### Return type

[**models::ContractEventsByTxId**](ContractEventsByTxId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

