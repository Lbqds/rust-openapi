# \MempoolApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_mempool_transactions**](MempoolApi.md#delete_mempool_transactions) | **DELETE** /mempool/transactions | Remove all transactions from mempool
[**get_mempool_transactions**](MempoolApi.md#get_mempool_transactions) | **GET** /mempool/transactions | List mempool transactions
[**put_mempool_transactions_rebroadcast**](MempoolApi.md#put_mempool_transactions_rebroadcast) | **PUT** /mempool/transactions/rebroadcast | Rebroadcase a mempool transaction to the network
[**put_mempool_transactions_validate**](MempoolApi.md#put_mempool_transactions_validate) | **PUT** /mempool/transactions/validate | Validate all mempool transactions and remove invalid ones



## delete_mempool_transactions

> delete_mempool_transactions()
Remove all transactions from mempool

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mempool_transactions

> Vec<models::MempoolTransactions> get_mempool_transactions()
List mempool transactions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MempoolTransactions>**](MempoolTransactions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_mempool_transactions_rebroadcast

> put_mempool_transactions_rebroadcast(tx_id)
Rebroadcase a mempool transaction to the network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_mempool_transactions_validate

> put_mempool_transactions_validate()
Validate all mempool transactions and remove invalid ones

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

